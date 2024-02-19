use tracing::error;
use std::sync::Arc;
use axum::{
    Router,
    routing::post,
};
use axum::http::{StatusCode, Uri};
use tower_http::cors;
use crate::app::{AppConfig, AppState};
use handlers::*;

mod handlers;

#[derive(Default, Clone, Debug)]
pub struct AdminAppState {
    pub app_state: Arc<AppState>,
    pub add_pin_recorder: async_tasks_recorder::AsyncTasksRecoder<Arc<String>>,
}

#[allow(unused_variables)]
pub fn generate_admin_app(app_config: &AppConfig, app_state: &Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/pin/add", post(add_pin))
        .route("/pin/add-async", post(add_pin_async));

    let admin_app_state = AdminAppState {
        app_state: app_state.clone(),
        add_pin_recorder: async_tasks_recorder::AsyncTasksRecoder::new(),
    };

    let tracing_layer = tower_http::trace::TraceLayer::new_for_http()
        // Create our own span for the request and include the matched path. The matched
        // path is useful for figuring out which handler the request was routed to.
        .make_span_with(|req: &axum::extract::Request| {
            let method = req.method();
            let uri = req.uri();

            // axum automatically adds this extension.
            let matched_path = req
                .extensions()
                .get::<axum::extract::MatchedPath>()
                .map(|matched_path| matched_path.as_str());

            tracing::debug_span!("request", %method, %uri, matched_path)
        });

    let cors_layer = cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    Router::new()
        .nest("/api", app)
        .with_state(admin_app_state)
        .layer(tracing_layer)
        .layer(cors_layer)
        .fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    error!("Receive a request but no route match. uri: {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
