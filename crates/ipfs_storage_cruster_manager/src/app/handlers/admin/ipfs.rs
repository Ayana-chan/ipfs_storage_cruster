//! API about ipfs node management.

#[allow(unused_imports)]
use tracing::{trace, debug, info};
use axum::extract::{State, Json};
use crate::imports::dao_imports::*;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::{dtos, services};

/// List all added IPFS nodes.
// #[axum_macros::debug_handler]
pub async fn list_ipfs_nodes(State(state): State<AppState>) -> StandardApiResult<dtos::ListIpfsNodesResponse> {
    info!("List IPFS nodes");
    let node_vec = Node::find().all(&state.db_conn)
        .await.map_err(services::db::handle_db_error)?;
    trace!("All ipfs nodes: {:?}", node_vec);
    let res = dtos::ListIpfsNodesResponse {
        list: node_vec
    };

    Ok(res.into())
}

/// Let target IPFS node bootstrap self.
/// Would set the status of node to `Online`.
/// Upsert the database entry.
// #[axum_macros::debug_handler]
pub async fn add_ipfs_node(State(state): State<AppState>, Json(args): Json<dtos::AddIpfsNodeArgs>) -> StandardApiResult<()> {
    info!("Add IPFS node. {:?}", args);
    let target_ipfs_client = state.get_ipfs_client_with_rpc_addr(args.rpc_address.clone());
    target_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_multi_address,
        &state.ipfs_metadata.ipfs_peer_id,
    ).await?;

    let aim_peer_id = target_ipfs_client.get_id_info()
        .await?
        .id;
    debug!("Add IPFS node target peer id: {}", aim_peer_id);

    let new_node = node::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        peer_id: Set(aim_peer_id),
        rpc_address: Set(args.rpc_address),
        wrapper_public_address: Set(Some(args.wrapper_public_address)),
        wrapper_admin_address: Set(Some(args.wrapper_admin_address)),
        node_status: Set(sea_orm_active_enums::NodeStatus::Online),
    };
    // upsert
    let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
        .update_columns([
            node::Column::RpcAddress,
            node::Column::WrapperPublicAddress,
            node::Column::WrapperAdminAddress,
            node::Column::NodeStatus,
        ])
        .to_owned();
    Node::insert(new_node)
        .on_conflict(dup_conflict)
        .exec(&state.db_conn)
        .await.map_err(services::db::handle_db_error)?;

    state.file_storage_decision_maker.on_add_new_node().await;

    Ok(().into())
}

/// Re-bootstrap all nodes in database that is not `Offline`.
// #[axum_macros::debug_handler]
pub async fn re_bootstrap_all_ipfs_node(State(state): State<AppState>) -> StandardApiResult<()> {
    info!("Re-bootstrap All IPFS Node.");
    let node_vec: Vec<node::Model> = Node::find()
        .filter(node::Column::NodeStatus.ne(sea_orm_active_enums::NodeStatus::Offline)) // No offline
        .all(&state.db_conn)
        .await.map_err(services::db::handle_db_error)?;

    let mut join_set = tokio::task::JoinSet::new();
    for node_model in node_vec {
        let task = services::ipfs::bootstrap_and_check_health(
            state.clone(), node_model,
        );
        join_set.spawn(task);
    }

    let mut success_count: u32 = 0;
    let mut fail_count: u32 = 0;
    while let Some(join_res) = join_set.join_next().await {
        match join_res {
            Ok(res) => {
                if res.is_ok() {
                    success_count += 1;
                } else {
                    fail_count += 1;
                }
            }
            Err(join_err) => {
                if join_err.is_panic() {
                    std::panic::resume_unwind(join_err.into_panic());
                }
            }
        }
    }
    info!("Re-bootstrap all IPFS node finished. Success: {}, Failed: {}", success_count, fail_count);

    Ok(().into())
}

