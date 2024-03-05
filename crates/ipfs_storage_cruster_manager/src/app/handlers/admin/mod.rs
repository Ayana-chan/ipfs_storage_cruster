use axum::Router;
use axum::routing::post;
use ipfs::*;
use crate::app::AppState;

mod ipfs;

pub fn generate_admin_router() -> Router<AppState>{
    Router::new()
        .route("/bootstrap", post(bootstrap_add))
}
