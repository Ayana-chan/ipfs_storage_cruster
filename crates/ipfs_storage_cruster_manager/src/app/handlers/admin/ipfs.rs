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

        node::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            peer_id: Set("abcd peer id".to_string()),
            rpc_address: Set("9.9.9.9:1234".to_string()),
            wrapper_address: Set("19.19.19.19:5678".to_string()),
        }
            .insert(&conn)
            .await.unwrap();

        let res = Node::find().all(&conn).await.unwrap();

        println!("find all: {:#?}", res);
    }
}
