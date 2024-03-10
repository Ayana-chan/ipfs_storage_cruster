#[allow(unused_imports)]
use tracing::{trace, debug, info, error};
use crate::imports::dao_imports::*;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app::AppState;

/// Bootstrap target node.
/// Set the node status to `Online` when succeed, or `Unhealthy` when fail.
///
/// Return the result of database update .
#[tracing::instrument(skip_all)]
async fn bootstrap_and_check_health(state: AppState, target_rpc_address: String) -> Result<node::Model, DbErr> {
    let aim_ipfs_client = ReqwestIpfsClient::new_with_reqwest_client(
        target_rpc_address, state.reqwest_client.clone(),
    );
    let res = aim_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_ip,
        &state.ipfs_metadata.ipfs_swarm_port,
        &state.ipfs_metadata.ipfs_peer_id,
    ).await;

    let status = match res {
        Ok(_) => sea_orm_active_enums::Status::Online,
        Err(_) => sea_orm_active_enums::Status::Unhealthy,
    };

    node::ActiveModel {
        status: Set(status),
        ..Default::default()
    }.update(&state.db_conn)
        .await
}
