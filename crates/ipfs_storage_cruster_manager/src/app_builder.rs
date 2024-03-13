use axum::Router;
use tokio::net::ToSocketAddrs;
use tracing::info;
use serde::Deserialize;
use crate::app;

// TODO 日志级别可配置化

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub ipfs_rpc_address: String,
    pub ipfs_swarm_multi_address: String,
}

#[tracing::instrument(skip_all)]
pub async fn serve(app_config: AppConfig) {
    info!("--- Server Start ---");
    info!("Server listen at: {}:{}", "0.0.0.0", 5000);

    info!("IPFS Node rpc at: {}", app_config.ipfs_rpc_address);

    generate_server(
        "0.0.0.0:5000",
        app::generate_app_from_config(&app_config).await
    ).await
}

/// Tool to bind server to port
async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
