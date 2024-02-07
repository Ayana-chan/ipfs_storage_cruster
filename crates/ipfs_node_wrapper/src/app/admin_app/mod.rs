use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use crate::app::AppState;

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct AdminAppState {
    pub(crate) app_state: Arc<AppState>,
}

pub fn generate_admin_app() -> Router<AdminAppState> {
    let app = Router::new()
        .route("/", get(|| async { "Soyorin Love!" }));

    Router::new()
        .nest("/api", app)
}
