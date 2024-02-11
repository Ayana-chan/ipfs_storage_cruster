use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use axum::Router;
use tokio::net::ToSocketAddrs;
use tracing::info;
use crate::ipfs_client::IpfsClient;

mod public_app;
mod admin_app;

pub struct AppConfig {
    // server config
    pub public_server_ip: IpAddr,
    pub public_server_port: u16,
    pub admin_server_ip: IpAddr,
    pub admin_server_port: u16,
    // Ipfs node config
    pub ipfs_gateway_address: SocketAddr,
    pub ipfs_rpc_address: SocketAddr,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct AppConfigBuilder {
    public_server_ip: Option<IpAddr>,
    public_server_port: Option<u16>,
    admin_server_ip: Option<IpAddr>,
    admin_server_port: Option<u16>,
    ipfs_gateway_address: Option<SocketAddr>,
    ipfs_rpc_address: Option<SocketAddr>,
}

#[allow(dead_code)]
impl AppConfigBuilder {
    pub fn new() -> Self {
        AppConfigBuilder::default()
    }

    // DEFAULT 0.0.0.0
    pub fn public_server_ip(mut self, value: IpAddr) -> Self {
        self.public_server_ip = Some(value);
        self
    }
    // DEFAULT 3000
    pub fn public_server_port(mut self, value: u16) -> Self {
        self.public_server_port = Some(value);
        self
    }
    // DEFAULT 0.0.0.0
    pub fn admin_server_ip(mut self, value: IpAddr) -> Self {
        self.admin_server_ip = Some(value);
        self
    }
    // DEFAULT 4000
    pub fn admin_server_port(mut self, value: u16) -> Self {
        self.admin_server_port = Some(value);
        self
    }
    // DEFAULT 127.0.0.1:8080
    pub fn ipfs_gateway_address(mut self, value: SocketAddr) -> Self {
        self.ipfs_gateway_address = Some(value);
        self
    }
    // DEFAULT 127.0.0.1:5001
    pub fn ipfs_rpc_address(mut self, value: SocketAddr) -> Self {
        self.ipfs_rpc_address = Some(value);
        self
    }

    pub fn finish(self) -> AppConfig {
        AppConfig {
            public_server_ip: self.public_server_ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            public_server_port: 3000,
            admin_server_ip: self.public_server_ip.unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
            admin_server_port: 4000,
            ipfs_gateway_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            ipfs_rpc_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
        }
    }
}

#[derive(Default, Debug)]
pub struct AppState {
    pub ipfs_client: IpfsClient,
}

#[tracing::instrument(skip_all)]
pub async fn serve(app_config: AppConfig) {
    let app_state = Arc::new(AppState {
        ipfs_client: IpfsClient::new_from_config(&app_config)
    });

    let public_server = generate_server(
        (app_config.public_server_ip, app_config.public_server_port),
        public_app::generate_public_app(&app_config, &app_state),
    );
    let admin_server = generate_server(
        (app_config.admin_server_ip, app_config.admin_server_port),
        admin_app::generate_admin_app(&app_config, &app_state),
    );

    info!("--- Server Start ---");

    info!("public service listen at: {}:{}", app_config.public_server_ip, app_config.public_server_port);
    info!("admin  service listen at: {}:{}", app_config.admin_server_ip, app_config.admin_server_port);

    info!("IPFS Node gateway at: {}", app_config.ipfs_gateway_address);
    info!("IPFS Node rpc     at: {}", app_config.ipfs_rpc_address);

    tokio::join!(public_server, admin_server);
}

/// Tool to bind server to port
async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

