use std::sync::Arc;
use axum::Router;
use tokio::net::ToSocketAddrs;
use tracing::info;
use tiny_ipfs_client::ReqwestIpfsClient;
use serde::Deserialize;
use crate::app::{admin_app, AppState, public_app};

#[derive(Deserialize)]
pub struct AppConfig {
    // Ipfs node config
    pub ipfs_gateway_address: String,
    pub ipfs_rpc_address: String,
}

#[tracing::instrument(skip_all)]
pub async fn serve(app_config: AppConfig) {
    info!("========** Server Preparing **========");
    info!("public service would listen at: {}:{}", "0.0.0.0", 3000);
    info!("admin  service would listen at: {}:{}", "0.0.0.0", 4000);

    info!("IPFS Node gateway at: {}", app_config.ipfs_gateway_address);
    info!("IPFS Node rpc     at: {}", app_config.ipfs_rpc_address);

    let app_state = Arc::new(AppState {
        ipfs_client: ReqwestIpfsClient::new(
            app_config.ipfs_gateway_address.to_string(),
            app_config.ipfs_rpc_address.to_string()
        ),
        file_traffic_counter: scc::HashMap::new(),
    });

    let public_server = generate_server(
        "0.0.0.0:3000",
        public_app::generate_public_app(&app_state),
    );
    let admin_server = generate_server(
        "0.0.0.0:4000",
        admin_app::generate_admin_app(&app_state).await,
    );

    info!("========*** Server start successfully ***========");
    tokio::join!(public_server, admin_server);
}

/// Tool to bind server to port
async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
