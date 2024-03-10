#[allow(unused_imports)]
use tracing::{trace, debug, info};
use axum::extract::{State, Json};
use crate::imports::dao_imports::*;
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::{errors, dtos, services};

/// List all added IPFS nodes.
#[axum_macros::debug_handler]
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
/// Upsert the database entry.
#[axum_macros::debug_handler]
pub async fn add_ipfs_node(State(state): State<AppState>, Json(args): Json<dtos::AddIpfsNodeArgs>) -> StandardApiResult<()> {
    let rpc_address = format!("{}:{}", args.ip, args.port.unwrap_or(5001));
    info!("Add IPFS node. rpc address: {}", rpc_address);
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
    debug!("Add IPFS node target peer id: {}", aim_peer_id);

    let new_node = node::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        peer_id: Set(aim_peer_id),
        rpc_address: Set(rpc_address),
        wrapper_address: Set(args.wrapper_address),
        status: Set(sea_orm_active_enums::Status::Online),
    };
    // upsert
    let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
        .update_columns([
            node::Column::RpcAddress,
            node::Column::WrapperAddress,
            node::Column::Status,
        ])
        .to_owned();
    Node::insert(new_node)
        .on_conflict(dup_conflict)
        .exec(&state.db_conn)
        .await.map_err(services::db::handle_db_error)?;

    Ok(().into())
}

#[axum_macros::debug_handler]
pub async fn re_bootstrap_all_ipfs_node(State(state): State<AppState>) -> StandardApiResult<()> {
    let node_vec: Vec<node::Model> = Node::find().all(&state.db_conn)
        .await.map_err(services::db::handle_db_error)?;

    let mut join_set = tokio::task::JoinSet::new();
    for node_model in node_vec {
        let task = services::ipfs::bootstrap_and_check_health(
            state.clone(), node_model.rpc_address,
        );
        join_set.spawn(task);
    }

    while let Some(join_res) = join_set.join_next().await {
        match join_res {
            Ok(res) => {
                match res {
                    Ok(_model) => {
                        todo!()
                    }
                    Err(e) => {
                        todo!()
                    }
                }
            }
            Err(join_err) => {
                if join_err.is_panic() {
                    std::panic::resume_unwind(join_err.into_panic());
                }
            }
        }
    }

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
            status: Set(sea_orm_active_enums::Status::Online),
        }.insert(&conn)
            .await.unwrap();
        println!("insert: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        // dup insert
        let new_node = node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("88.88.88.88:1234".to_string()),
            wrapper_address: Set("89.89.89.89:5678".to_string()),
            status: Set(sea_orm_active_enums::Status::Unhealthy),
        };
        let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
            .update_columns([
                node::Column::RpcAddress,
                node::Column::WrapperAddress,
                node::Column::Status,
            ])
            .to_owned();
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
