use axum::routing::post;
use axum::Router;

mod file;
mod admin;

use file::*;
use crate::app::AppState;

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/file", post(upload_file))
        .nest("/admin", admin::generate_admin_router())
}
