use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use crate::app::AppState;

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct AdminAppState {
    app_state: Arc<AppState>,
}

pub fn generate_admin_app(app_state: &Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/", get(|| async { "Soyorin Love!" }));

    Router::new()
        .nest("/api", app)
        .with_state(AdminAppState {
            app_state: app_state.clone(),
        })
}
