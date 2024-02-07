use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use crate::app::AppState;
use handlers::*;

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct PublicAppState {
    pub(crate) app_state: Arc<AppState>,
}

pub fn generate_public_app() -> Router<PublicAppState> {
    let app = Router::new()
        .route("/", get(get_file));

    Router::new()
        .nest("/api", app)
}

