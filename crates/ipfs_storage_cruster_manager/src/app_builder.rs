use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::Router;
use tokio::net::ToSocketAddrs;
use tracing::info;

pub struct AppConfig {
    // server config
    pub server_ip: IpAddr,
    pub server_port: u16,
    pub ipfs_rpc_address: SocketAddr,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfigBuilder::new().finish()
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct AppConfigBuilder {
    server_ip: Option<IpAddr>,
    server_port: Option<u16>,
    ipfs_rpc_address: Option<SocketAddr>,
}

#[allow(dead_code)]
impl AppConfigBuilder {
    pub fn new() -> Self {
        AppConfigBuilder::default()
    }

    /// DEFAULT 0.0.0.0
    pub fn server_ip(mut self, value: IpAddr) -> Self {
        self.server_ip = Some(value);
        self
    }
    /// DEFAULT 5000
    pub fn server_port(mut self, value: u16) -> Self {
        self.server_port = Some(value);
        self
    }

    pub fn finish(self) -> AppConfig {
        AppConfig {
            server_ip: self.server_ip.unwrap_or(
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
            ),
            server_port: self.server_port.unwrap_or(
                5000
            ),
            ipfs_rpc_address: self.ipfs_rpc_address.unwrap_or(
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001)
            ),
        }
    }
}

#[tracing::instrument(skip_all)]
pub async fn serve(app_config: AppConfig) {
    info!("--- Server Start ---");
    info!("Server listen at: {}:{}", app_config.server_ip, app_config.server_port);

    info!("IPFS Node rpc at: {}", app_config.ipfs_rpc_address);


}

/// Tool to bind server to port
async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
