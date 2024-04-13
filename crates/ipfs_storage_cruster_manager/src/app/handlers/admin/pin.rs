//! API about pins.

use axum::extract::{State, Query};
use tracing::debug;
use crate::imports::dao_imports::*;
use crate::app::{AppState, daos};
use crate::app::common::StandardApiResult;
use crate::app::{dtos, services, errors};

/// List all pins in the certain node.
/// Just query by IPFS RPC, which returns pins that actually stored in target node.
// #[axum_macros::debug_handler]
pub async fn list_pins_in_one_node_actually(State(state): State<AppState>, Query(args): Query<dtos::ListPinsInOneNodeActuallyArgs>)
                                            -> StandardApiResult<dtos::ListPinsInOneNodeActuallyResponse> {
    let rpc_addr = daos::find_ipfs_node_rpc_by_id(args.node_id.clone(), &state.db_conn).await
        .map_err(services::db::handle_db_error)?
        .ok_or_else(|| errors::DB_TARGET_DATA_NOT_EXIST.clone_to_error())?;
    debug!("Find pins actually in node by RPC address: {rpc_addr:?}");

    let client = state.get_ipfs_client_with_rpc_addr(rpc_addr);
    let pins = client.list_recursive_pins_pinned(false).await?;
    let pins_cid = pins.keys.into_keys().collect();

    let res = dtos::ListPinsInOneNodeActuallyResponse {
        node_id: args.node_id.clone(),
        pins_cid,
    };
    Ok(res.into())
}

/// List all pins in the certain node.
/// Only query inside the database.
// #[axum_macros::debug_handler]
pub async fn list_pins_in_one_node(State(state): State<AppState>, Query(args): Query<dtos::ListPinsInOneNodeArgs>)
                                   -> StandardApiResult<dtos::ListPinsInOneNodeResponse> {
    let pins = Pin::find()
        .join(
            JoinType::InnerJoin,
            Pin::belongs_to(PinsStoredNodes)
                .from(pin::Column::Id)
                .to(pins_stored_nodes::Column::PinId)
                .into(),
        )
        .filter(pins_stored_nodes::Column::NodeId.eq(args.node_id.clone()))
        .all(&state.db_conn).await
        .map_err(services::db::handle_db_error)?;

    let res = dtos::ListPinsInOneNodeResponse {
        node_id: args.node_id.clone(),
        pins,
    };
    debug!("Find node {} has {} pins", res.node_id, res.pins.len());
    Ok(res.into())
}

/// List all nodes that store the certain pin.
/// Only query inside the database.
// #[axum_macros::debug_handler]
pub async fn list_nodes_with_pin(State(state): State<AppState>, Query(args): Query<dtos::ListNodesWithPinArgs>)
                                 -> StandardApiResult<dtos::ListNodesWithPinResponse> {
    let nodes = Node::find()
        .join(
            JoinType::InnerJoin,
            Node::belongs_to(PinsStoredNodes)
                .from(node::Column::Id)
                .to(pins_stored_nodes::Column::NodeId)
                .into(),
        )
        .filter(pins_stored_nodes::Column::PinId.eq(args.pin_id.clone()))
        .all(&state.db_conn).await
        .map_err(services::db::handle_db_error)?;

    let res = dtos::ListNodesWithPinResponse {
        pin_id: args.pin_id.clone(),
        nodes,
    };
    debug!("Find pin {} in {} nodes", res.pin_id, res.nodes.len());
    Ok(res.into())
}

