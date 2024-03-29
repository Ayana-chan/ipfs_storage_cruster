use tracing::error;
use std::sync::Arc;
use async_tasks_state_map::AsyncTasksRecorder;
use axum::Router;
use axum::routing::{get, post, delete};
use axum::http::{StatusCode, Uri};
use tower_http::cors;
use crate::app::AppState;
use handlers::*;

pub mod handlers;
mod ipfs_helper;

#[derive(Default, Clone, Debug)]
pub struct AdminAppState {
    pub app_state: Arc<AppState>,
    pub add_pin_task_recorder: AsyncTasksRecorder<String>,
}

pub async fn generate_admin_app(app_state: &Arc<AppState>) -> Router {
    let admin_app_state = AdminAppState {
        app_state: app_state.clone(),
        add_pin_task_recorder: AsyncTasksRecorder::new(),
    };

    ipfs_helper::init_ipfs_contact(&admin_app_state).await;

    let app = Router::new()
        .route("/info", get(get_ipfs_node_info))
        .route("/pin", get(list_succeeded_pins))
        .route("/pin/:cid", get(check_pin))
        .route("/pin", post(add_pin))
        .route("/pin", delete(rm_pin))
        .route("/traffic", get(get_download_time_list));

    let app = Router::new()
        .nest("/api", app)
        .with_state(admin_app_state);

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
