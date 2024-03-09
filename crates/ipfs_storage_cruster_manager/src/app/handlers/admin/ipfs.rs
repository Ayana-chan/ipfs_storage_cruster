#[allow(unused_imports)]
use tracing::{trace, info};
use axum::extract::{State, Json};
use ipfs_storage_cruster_manager_entity::prelude::*;
use sea_orm::prelude::*;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::dtos;
use crate::db_helper::handle_db_error;

/// List all added IPFS nodes.
#[axum_macros::debug_handler]
pub async fn list_ipfs_nodes(State(state): State<AppState>) -> StandardApiResult<dtos::ListIpfsNodesResponse>{
    info!("List IPFS nodes");
    let node_vec = Node::find().all(&state.db_conn)
        .await.map_err(handle_db_error)?;
    trace!("All ipfs nodes: {:?}", node_vec);
    let res = dtos::ListIpfsNodesResponse {
        list: node_vec
    };

    Ok(res.into())
}

/// Let target IPFS node bootstrap self.
#[axum_macros::debug_handler]
pub async fn add_ipfs_node(State(state): State<AppState>, Json(args): Json<dtos::AddIpfsNodeArgs>) -> StandardApiResult<()> {
    let rpc_address = format!("{}:{}", args.ip, args.port.unwrap_or(5001));
    let aim_ipfs_client = ReqwestIpfsClient::new_with_reqwest_client(
        rpc_address, state.reqwest_client.clone()
    );

    let res = aim_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_ip,
        &state.ipfs_metadata.ipfs_swarm_port,
        &state.ipfs_metadata.ipfs_peer_id
    ).await;

    todo!();

    Ok(().into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::*;

    const DB_URL: &str = "mysql://root:1234@localhost/ipfs_storage_cruster_manager";

    #[tokio::test]
    #[ignore]
    async fn test_db() {
        let conn = Database::connect(DB_URL)
            .await
            .expect("Database connection failed");

        let new_uuid = uuid::Uuid::new_v4().to_string();

        node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("9.9.9.9:1234".to_string()),
            wrapper_address: Set("19.19.19.19:5678".to_string()),
        }
            .insert(&conn)
            .await.unwrap();
        println!("insert: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        let aim_opt = Node::find_by_id(new_uuid.clone()).one(&conn).await.unwrap();
        let aim = aim_opt.unwrap();
        aim.delete(&conn).await.unwrap();
        println!("delete: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);
    }
}
