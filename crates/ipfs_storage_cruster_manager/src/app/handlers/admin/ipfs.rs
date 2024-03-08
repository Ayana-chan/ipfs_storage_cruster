use ipfs_storage_cruster_manager_entity::prelude::*;
use ipfs_storage_cruster_manager_entity::node;
use sea_orm::Database;

/// List all added IPFS nodes.
#[axum_macros::debug_handler]
pub async fn list_ipfs_nodes() {}

/// Let target IPFS node bootstrap self.
#[axum_macros::debug_handler]
pub async fn add_ipfs_node() {}


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
