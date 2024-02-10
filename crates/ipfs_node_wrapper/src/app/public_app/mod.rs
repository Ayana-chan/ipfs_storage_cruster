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
        })
        // By default `TraceLayer` will log 5xx responses but we're doing our specific
        // logging of errors so disable that
        .on_failure(());

    let cors_layer = cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    Router::new()
        .nest("/api", app)
        .with_state(public_app_state)
        .layer(tracing_layer)
        .layer(cors_layer)
}

