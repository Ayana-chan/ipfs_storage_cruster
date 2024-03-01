use tracing::error;
use std::sync::Arc;
use axum::{
    Router,
    routing::get,
};
use axum::http::{StatusCode, Uri};
use tower_http::cors;
use crate::app::AppState;
use handlers::*;

pub mod handlers;

#[derive(Default, Clone, Debug)]
pub struct PublicAppState {
    pub app_state: Arc<AppState>,
}

pub fn generate_public_app(app_state: &Arc<AppState>) -> Router {
    let public_app_state = PublicAppState {
        app_state: app_state.clone(),
    };

    let app = Router::new()
        .route("/:cid", get(get_file));

    let app = Router::new()
        .nest("/api", app)
        .with_state(public_app_state);

    decorate_router(app)
}

fn decorate_router(router: Router) -> Router {
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

    router
        .layer(tracing_layer)
        .layer(cors_layer)
        .fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    error!("Receive a request but no route match. uri: {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

