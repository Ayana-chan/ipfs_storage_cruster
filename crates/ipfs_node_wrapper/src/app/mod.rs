use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use axum::Router;
use tokio::net::ToSocketAddrs;
use crate::app::admin_app::AdminAppState;
use crate::app::public_app::PublicAppState;
use crate::ipfs_client::IpfsNodeMetadata;

mod public_app;
mod admin_app;

pub struct AppConfig {
    // server config
    public_server_ip: IpAddr,
    public_server_port: u16,
    admin_server_ip: IpAddr,
    admin_server_port: u16,
    // Ipfs node config
    ipfs_gateway_address: SocketAddr,
    ipfs_rpc_address: SocketAddr,
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
    pub ipfs_node_metadata: parking_lot::RwLock<IpfsNodeMetadata>,
}

pub async fn serve(app_config: AppConfig) {
    let app_state = Arc::new(AppState {
        ipfs_node_metadata: parking_lot::RwLock::new(IpfsNodeMetadata {
            gateway_address: app_config.ipfs_gateway_address.to_string(),
            rpc_address: app_config.ipfs_rpc_address.to_string(),
        })
    });
    let public_app_state = PublicAppState {
        app_state: app_state.clone(),
    };
    let admin_app_state = AdminAppState {
        app_state: app_state.clone(),
    };

    let public_server = generate_server(
        (app_config.public_server_ip, app_config.public_server_port),
        public_app::generate_public_app().with_state(public_app_state),
    );
    let admin_server = generate_server(
        (app_config.admin_server_ip, app_config.admin_server_port),
        admin_app::generate_admin_app().with_state(admin_app_state),
    );
    tokio::join!(public_server, admin_server);
}

/// Tool to bind server to port
async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

