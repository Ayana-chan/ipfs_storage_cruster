use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use tower_http::cors;
use crate::app::{AppConfig, AppState};
use handlers::*;

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct PublicAppState {
    pub app_state: Arc<AppState>,
}

#[allow(unused_variables)]
pub fn generate_public_app(app_config: &AppConfig, app_state: &Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/:cid", get(get_file));

    let public_app_state = PublicAppState {
        app_state: app_state.clone(),
    };

    let cors_layer = cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    Router::new()
        .nest("/api", app)
        .with_state(public_app_state)
        .layer(cors_layer)
}

