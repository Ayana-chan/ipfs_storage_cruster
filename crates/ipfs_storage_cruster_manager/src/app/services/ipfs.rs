#[allow(unused_imports)]
use tracing::{error, debug, warn, info, trace};
use crate::imports::dao_imports::*;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app::AppState;

/// Regularly try until get peer id successfully.
#[tracing::instrument(skip_all)]
pub async fn get_peer_id_until_success(ipfs_client: &ReqwestIpfsClient, interval_time_ms: u64) -> String {
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
pub(crate) async fn bootstrap_and_check_health(state: AppState, node_model: node::Model) -> Result<node::Model, DbErr> {
    let _target_peer_id = node_model.peer_id.clone();
    let target_ipfs_client = ReqwestIpfsClient::new_with_reqwest_client(
        node_model.rpc_address.clone(), state.reqwest_client.clone(),
    );
    let res = target_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_multi_address,
        &state.ipfs_metadata.ipfs_peer_id,
    ).await;

    let status = match res {
        Ok(_) => sea_orm_active_enums::NodeStatus::Online,
        Err(_) => sea_orm_active_enums::NodeStatus::Unhealthy,
    };

    let mut node_model: node::ActiveModel = node_model.into();
    node_model.node_status = Set(status);

    let res: Result<node::Model, DbErr> = node_model
        .update(&state.db_conn)
        .await;

    match res {
        Ok(m) => {
            Ok(m)
        },
        Err(e) => {
            error!("Failed to set status of node in database. peer_id {:?}", _target_peer_id);
            Err(e)
        }
    }
}
