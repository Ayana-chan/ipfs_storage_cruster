#[allow(unused_imports)]
use tracing::{trace, info};
use axum::extract::{State, Json};
use ipfs_storage_cruster_manager_entity::prelude::*;
use ipfs_storage_cruster_manager_entity::*;
use sea_orm::prelude::*;
use sea_orm::*;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::errors;
use crate::app::dtos;
use crate::db_helper::handle_db_error;

/// List all added IPFS nodes.
#[axum_macros::debug_handler]
pub async fn list_ipfs_nodes(State(state): State<AppState>) -> StandardApiResult<dtos::ListIpfsNodesResponse> {
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
/// Upsert the database entry.
#[axum_macros::debug_handler]
pub async fn add_ipfs_node(State(state): State<AppState>, Json(args): Json<dtos::AddIpfsNodeArgs>) -> StandardApiResult<()> {
    let rpc_address = format!("{}:{}", args.ip, args.port.unwrap_or(5001));
    let aim_ipfs_client = ReqwestIpfsClient::new_with_reqwest_client(
        rpc_address.clone(), state.reqwest_client.clone(),
    );

    aim_ipfs_client.bootstrap_add(
        &state.ipfs_metadata.ipfs_swarm_ip,
        &state.ipfs_metadata.ipfs_swarm_port,
        &state.ipfs_metadata.ipfs_peer_id,
    ).await.map_err(errors::error_convert::from_ipfs_client_error)?;

    let aim_peer_id = aim_ipfs_client.get_id_info()
        .await.map_err(errors::error_convert::from_ipfs_client_error)?
        .id;

    let new_node = node::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        peer_id: Set(aim_peer_id),
        rpc_address: Set(rpc_address),
        wrapper_address: Set(args.wrapper_address),
    };
    // upsert
    let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
        .update_columns([node::Column::RpcAddress, node::Column::WrapperAddress])
        .to_owned();
    Node::insert(new_node)
        .on_conflict(dup_conflict)
        .exec(&state.db_conn)
        .await.map_err(handle_db_error)?;

    Ok(().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DB_URL: &str = "mysql://root:1234@localhost/ipfs_storage_cruster_manager";

    #[tokio::test]
    #[ignore]
    async fn test_db() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();

        let conn = Database::connect(DB_URL)
            .await
            .expect("Database connection failed");

        let new_uuid = uuid::Uuid::new_v4().to_string();

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("9.9.9.9:1234".to_string()),
            wrapper_address: Set("19.19.19.19:5678".to_string()),
        }.insert(&conn)
            .await.unwrap();
        println!("insert: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        // dup insert
        let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
            .update_columns([node::Column::RpcAddress, node::Column::WrapperAddress])
            .to_owned();
        let new_node = node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("88.88.88.88:1234".to_string()),
            wrapper_address: Set("89.89.89.89:5678".to_string()),
        };
        let result = Node::insert(new_node)
            .on_conflict(dup_conflict)
            .exec(&conn)
            .await.unwrap();
        assert_eq!(result.last_insert_id, new_uuid);

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
