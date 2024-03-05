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
