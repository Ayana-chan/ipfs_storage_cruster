use axum::routing::post;
use axum::Router;

mod file;
mod admin;

use file::*;
use crate::app::AppState;

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .nest("/admin", admin::generate_admin_router())
        .route("/file", post(upload_file))
}

#[cfg(test)]
mod tests {
    use crate::app::services;
    use crate::imports::dao_imports::*;

    const DB_URL: &str = "mysql://root:1234@localhost/ipfs_storage_cruster_manager";

    #[tokio::test]
    #[ignore]
    async fn try_db() {
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
            wrapper_public_address: Set(Some("19.19.19.19:5678".to_string())),
            wrapper_admin_address: Set(Some("19.19.19.19:9999".to_string())),
            node_status: Set(sea_orm_active_enums::NodeStatus::Online),
        }.insert(&conn)
            .await.unwrap();
        println!("insert: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        // dup insert with on conflict
        let new_node = node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("88.88.88.88:1234".to_string()),
            wrapper_public_address: Set(Some("89.89.89.89:5678".to_string())),
            wrapper_admin_address: Set(Some("89.89.89.89:9999".to_string())),
            node_status: Set(sea_orm_active_enums::NodeStatus::Unhealthy),
        };
        let dup_conflict = sea_query::OnConflict::column(node::Column::PeerId)
            .update_columns([
                node::Column::RpcAddress,
                node::Column::WrapperPublicAddress,
                node::Column::WrapperAdminAddress,
                node::Column::NodeStatus,
            ])
            .to_owned();
        let result = Node::insert(new_node)
            .on_conflict(dup_conflict)
            .exec(&conn)
            .await.unwrap();
        assert_eq!(result.last_insert_id, new_uuid);

        // dup insert with dup error check
        let new_node = node::ActiveModel {
            id: Set(new_uuid.clone()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("11.11.11.11:1234".to_string()),
            wrapper_public_address: Set(Some("11.11.11.11:5678".to_string())),
            wrapper_admin_address: Set(Some("11.11.11.11:9999".to_string())),
            node_status: Set(sea_orm_active_enums::NodeStatus::Offline),
        };
        let result = Node::insert(new_node)
            .exec(&conn)
            .await;
        match result {
            Ok(_) => {
                panic!("Should have dup err");
            }
            Err(e) => {
                let check_res = services::db::check_duplicate_key_error(e);
                println!("dup insert return err: {:?}", check_res);
                assert!(check_res.is_ok(), "Should be dup err, but get {:?}", check_res);
            }
        }

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);

        let aim_opt = Node::find_by_id(new_uuid.clone()).one(&conn).await.unwrap();
        let aim = aim_opt.unwrap();
        aim.delete(&conn).await.unwrap();
        println!("delete: {}", new_uuid);

        let res = Node::find().all(&conn).await.unwrap();
        println!("find all: {:#?}", res);
    }

    #[tokio::test]
    #[ignore]
    async fn try_join_sql() {
        let node_id = "aaaid";
        let sql = Pin::find()
            .join(
                JoinType::InnerJoin,
                Pin::belongs_to(PinsStoredNodes)
                    .from(pin::Column::Id)
                    .to(pins_stored_nodes::Column::PinId)
                    .into()
            )
            .filter(pins_stored_nodes::Column::NodeId.eq(node_id))
            .build(DbBackend::MySql)
            .to_string();
        println!("{sql}");
    }
}

