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

    Router::new()
        .nest("/api", app)
        .with_state(admin_app_state)
        .layer(tracing_layer)
}
