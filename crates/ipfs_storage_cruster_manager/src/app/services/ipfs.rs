use axum::http;
#[allow(unused_imports)]
use tracing::{error, debug, warn, info, trace};
use http_body_util::BodyExt;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::imports::dao_imports::*;
use crate::app::AppState;
use crate::utils::move_entry_between_header_map;
use crate::app::common::ApiResult;
use crate::app::{dtos, errors};
use crate::app::errors::ResponseError;
use crate::file_decision::TargetIpfsNodeMessage;

static RE_BOOTSTRAP_TIMEOUT_MS: u64 = 2000;

/// Regularly try until get peer id successfully.
#[tracing::instrument(skip_all)]
pub(crate) async fn get_peer_id_until_success(ipfs_client: &ReqwestIpfsClient, interval_time_ms: u64) -> String {
    loop {
        let res = ipfs_client.get_id_info().await;
        match res {
            Ok(res) => {
                return res.id;
            }
            Err(_e) => {
                error!("Failed to cache recursive pins. Try again in {} ms. msg: {:?}", interval_time_ms, _e);
                tokio::time::sleep(tokio::time::Duration::from_millis(interval_time_ms)).await;
            }
        };
    }
}

/// Bootstrap target node.
/// Set the node status to `Online` when succeed, or `Unhealthy` when fail.
///
/// Return the result of database update.
#[tracing::instrument(skip_all)]
pub(crate) async fn bootstrap_and_check_health(state: AppState, node_model: node::Model) -> Result<node::Model, ()> {
    let _target_peer_id = node_model.peer_id.clone();
    let target_ipfs_client = state.get_ipfs_client_with_rpc_addr(node_model.rpc_address.clone());
    let task = target_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_multi_address,
        &state.ipfs_metadata.ipfs_peer_id,
    );

    let res = tokio::time::timeout(
        tokio::time::Duration::from_millis(RE_BOOTSTRAP_TIMEOUT_MS),
        task,
    ).await;

    let status = match res {
        Ok(res) => {
            match res {
                Ok(_) => sea_orm_active_enums::NodeStatus::Online,
                Err(_) => sea_orm_active_enums::NodeStatus::Unhealthy,
            }
        }
        Err(_e) => {
            error!("Bootstrap node for timeout. peer_id: {}", _target_peer_id);
            sea_orm_active_enums::NodeStatus::Unhealthy
        }
    };

    let mut node_model: node::ActiveModel = node_model.into();
    node_model.node_status = Set(status);

    let res: Result<node::Model, DbErr> = node_model
        .update(&state.db_conn)
        .await;

    match res {
        Ok(m) => {
            Ok(m)
        }
        Err(e) => {
            error!("Failed to set status of node in database. peer_id {:?}. msg: {:?}", _target_peer_id, e);
            Err(())
        }
    }
}

/// Add a file to ipfs by stream, return the message of the added file.
pub(crate) async fn add_file_to_ipfs(state: &AppState, mut req: axum::extract::Request) -> ApiResult<dtos::IpfsAddFileResponse> {
    // log
    let file_size = req.headers().get(http::header::CONTENT_LENGTH);
    if file_size.is_none() {
        warn!("Add file without content length in headers");
    } else if let Some(file_size) = file_size {
        info!("Add file. Content size: {:?}", file_size);
    }

    // handle url
    let url = format!("http://{}/api/v0/add", state.ipfs_client.rpc_address);
    *req.uri_mut() = http::uri::Uri::try_from(url).expect("Impossible fail to parse url");

    // handle headers
    let old_hm_ref = req.headers();
    let mut hm = http::header::HeaderMap::new();
    hm.reserve(5);
    move_entry_between_header_map(http::header::HOST, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONNECTION, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONTENT_LENGTH, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::ACCEPT, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONTENT_TYPE, old_hm_ref, &mut hm);
    *req.headers_mut() = hm;
    trace!("add req: {:?}", req);

    // read body
    let res = state.raw_hyper_client
        .request(req)
        .await
        .map_err(|_e|
            errors::IPFS_REQUEST_ERROR.clone_to_error_with_log_with_content(_e)
        )?;
    if !res.status().is_success() {
        error!("Failed to add file to IPFS. Status code: {}", res.status());
        return Err(errors::IPFS_RESPOND_ERROR.clone_to_error());
    }
    let body = res.into_body().collect();
    let body = body.await
        .map_err(|_e| {
            error!("Failed to receive IPFS response when add file");
            errors::IPFS_FAIL.clone_to_error()
        })?;
    let body = body.to_bytes();
    let body: dtos::IpfsAddFileResponse = serde_json::from_slice(body.as_ref())
        .map_err(|_e| {
            error!("Unexpected IPFS response when add file");
            errors::IPFS_FAIL.clone_to_error()
        })?;
    info!("Add file succeed. {:?}", body);

    Ok(body)
}

// TODO Revoke storage (remove pin) when cluster unhealthy.
/// Make decision and store file with certain CID to cluster.
///
/// Return the list of nodes that stores the file.
#[tracing::instrument(skip_all)]
pub(crate) async fn store_file_to_cluster(state: &AppState, cid: String) -> ApiResult<Vec<TargetIpfsNodeMessage>> {
    let target_node_list = state.file_storage_decision_maker
        .decide_store_node(&cid, &state.db_conn, &state.reqwest_client)
        .await?;
    // error when empty nodes
    if target_node_list.is_empty() {
        return Err(errors::IPFS_NODE_CLUSTER_UNHEALTHY.clone_to_error());
    }
    debug!("Firstly store pin {cid} in nodes: {target_node_list:?}");
    // send file to nodes
    let mut join_set = tokio::task::JoinSet::new();
    for node in target_node_list.into_iter() {
        let client = state.reqwest_client.clone();
        let task = add_pin_to_node(client, node, cid.clone());
        join_set.spawn(task);
    }

    let mut final_stored_nodes = Vec::new();
    while let Some(res) = join_set.join_next().await {
        if let Ok(res) = res {
            match res {
                Ok(v) => {
                    debug!("Succeed add pin {cid} to {:?}", v);
                    final_stored_nodes.push(v);
                    continue;
                }
                Err(e) => {
                    // stop when cluster unhealthy
                    if e == errors::IPFS_NODE_CLUSTER_UNHEALTHY {
                        error!("Failed add pin {cid} to cluster");
                        return Err(e);
                    }
                }
            }
        }

        // Failed to add pin, retry
        let retry_target_node_list = state.file_storage_decision_maker
            .decide_store_node_fail_one(&cid, &state.db_conn, &state.reqwest_client)
            .await?;
        debug!("Retry to add pin {cid} to nodes: {retry_target_node_list:?}");
        for node in retry_target_node_list.into_iter() {
            let client = state.reqwest_client.clone();
            let task = add_pin_to_node(client, node, cid.clone());
            join_set.spawn(task);
        }
    }

    info!("Store pin {cid} in nodes: {final_stored_nodes:?}");
    Ok(final_stored_nodes)
}

/// Send add pin RPC to an IPFS node.
///
/// Return `TargetIPFSNodeMessage` when success.
async fn add_pin_to_node(client: reqwest::Client, node_message: TargetIpfsNodeMessage, cid: String) -> ApiResult<TargetIpfsNodeMessage> {
    let client = ReqwestIpfsClient::new_with_reqwest_client(node_message.rpc_address.clone(), client);
    let res = client.add_pin_recursive(&cid, None).await
        .map_err(Into::<ResponseError>::into);
    if let Err(e) = res {
        let rpc_address = client.rpc_address;
        error!("Failed to add pin of {cid} to IPFS node {rpc_address}, because: {e:?}");
        return Err(e);
    }
    Ok(node_message)
}
