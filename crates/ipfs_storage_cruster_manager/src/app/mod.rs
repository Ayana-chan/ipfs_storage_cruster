use std::sync::Arc;
use tracing::error;
use axum::body::Body;
use axum::http::{StatusCode, Uri};
use axum::Router;
use tower_http::cors;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use tiny_ipfs_client::ReqwestIpfsClient;
use crate::app_builder::AppConfig;

pub mod handlers;
pub mod errors;
pub mod dtos;
pub mod common;

pub type RawHyperClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

/// State of app. Should be cheap and safe to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Unified reqwest client for use.
    pub reqwest_client: reqwest::Client,
    /// Contact self's IPFS node.
    pub ipfs_client: Arc<ReqwestIpfsClient>,
    /// Used to send raw hyper request.
    pub raw_hyper_client: RawHyperClient,
}

impl AppState {
    pub fn from_app_config(app_config: &AppConfig) -> AppState {
        let reqwest_client = reqwest::Client::new();
        AppState {
            reqwest_client: reqwest_client.clone(),
            ipfs_client: ReqwestIpfsClient::new_with_reqwest_client(
                app_config.ipfs_rpc_address.to_string(),
                reqwest_client
            ).into(),
            raw_hyper_client: hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
                .build(HttpConnector::new()),
        }
    }
}

pub fn generate_app_from_config(app_config: &AppConfig) -> Router {
    let app_state = AppState::from_app_config(app_config);

    // TODO pin没有api前缀。要分开生成路由
    let app = handlers::generate_router();

    let app = Router::new()
        .nest("/api", app)
        .with_state(app_state);

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
