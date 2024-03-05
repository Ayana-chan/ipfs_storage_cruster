use axum::Router;
use axum::routing::{get, post};
use ipfs::*;
use crate::app::AppState;

mod ipfs;

pub fn generate_admin_router() -> Router<AppState>{
    Router::new()
        .route("/ipfs", get(list_ipfs_nodes))
        .route("/ipfs", post(add_ipfs_node))
}
