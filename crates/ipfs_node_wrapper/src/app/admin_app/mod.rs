use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use crate::app::{AppConfig, AppState};

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct AdminAppState {
    pub app_state: Arc<AppState>,
}

#[allow(unused_variables)]
pub fn generate_admin_app(app_config: &AppConfig, app_state: &Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/", get(|| async { "Soyorin Love!" }));

    let admin_app_state = AdminAppState {
        app_state: app_state.clone(),
    };

    Router::new()
        .nest("/api", app)
        .with_state(admin_app_state)
}
