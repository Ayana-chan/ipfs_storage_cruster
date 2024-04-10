//! Admin APIs.

use axum::Router;
use axum::routing::{get, post};
use crate::app::AppState;

use ipfs::*;
use pin::*;

mod ipfs;
mod pin;

pub fn generate_admin_router() -> Router<AppState> {
    Router::new()
        .route("/ipfs", get(list_ipfs_nodes))
        .route("/ipfs", post(add_ipfs_node))
        .route("/ipfs/re-bootstrap", get(re_bootstrap_all_ipfs_node))
        .route("/pin/ls_pins_of_node_actually", get(list_pins_in_one_node_actually))
        .route("/pin/ls_pins_of_node", get(list_pins_in_one_node))
        .route("/pin/ls_nodes_of_pin", get(list_nodes_with_pin))
}
