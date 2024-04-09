//! API about pins.

use axum::extract::{State, Json};
use crate::imports::dao_imports::*;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::{dtos, services};

/// List all pins in the certain node.
/// Only query inside the database.
// #[axum_macros::debug_handler]
pub async fn list_pins_in_one_node(State(state): State<AppState>, Json(args): Json<dtos::ListPinsInOneNodeArgs>)
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
    Ok(res.into())
}

/// List all nodes that store the certain pin.
/// Only query inside the database.
// #[axum_macros::debug_handler]
pub async fn list_nodes_with_pin(State(state): State<AppState>, Json(args): Json<dtos::ListNodesWithPinArgs>)
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
    Ok(res.into())
}

