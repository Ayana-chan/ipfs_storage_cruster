//! Functions to contact database.

use crate::app::services::db::DbResult;
use crate::file_decision::TargetPublicWrapperMessage;
use crate::imports::dao_imports::*;

/// Find the RPC address of target node determined by node id.
///
/// Return Ok(None) if `node_id` not exist.
pub async fn find_ipfs_node_rpc_by_id(node_id: String, db_conn: &DatabaseConnection) -> DbResult<Option<String>> {
    #[derive(DerivePartialModel, FromQueryResult)]
    #[sea_orm(entity = "Node")]
    #[allow(dead_code)]
    struct PartialNode {
        rpc_address: String,
    }
    let models = Node::find_by_id(node_id.clone())
        .into_partial_model::<PartialNode>()
        .one(db_conn).await?;

    let rpc_address = models.map(|v| v.rpc_address);
    Ok(rpc_address)
}

/// Find all nodes that store the pin with certain CID.
pub async fn find_nodes_with_pin_cid(cid: &str, db_conn: &DatabaseConnection) -> DbResult<Vec<TargetPublicWrapperMessage>> {
    Node::find()
        .join(
            JoinType::InnerJoin,
            Node::belongs_to(PinsStoredNodes)
                .from(node::Column::Id)
                .to(pins_stored_nodes::Column::NodeId)
                .into(),
        )
        .join(
            JoinType::InnerJoin,
            PinsStoredNodes::belongs_to(Pin)
                .from(pins_stored_nodes::Column::PinId)
                .to(pin::Column::Id)
                .into(),
        )
        .filter(pin::Column::Cid.eq(cid))
        .into_partial_model::<TargetPublicWrapperMessage>()
        .all(db_conn).await
}
