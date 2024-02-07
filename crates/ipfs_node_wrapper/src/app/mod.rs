use std::sync::Arc;
use axum::Router;
use tokio::net::ToSocketAddrs;
use crate::ipfs_client::IpfsNodeMetadata;

mod public_app;
mod admin_app;

#[derive(Default, Clone, Debug)]
pub struct AppState {
    ipfs_node_metadata: Arc<parking_lot::RwLock<IpfsNodeMetadata>>,
}

pub async fn serve(public_server_address: impl ToSocketAddrs, admin_server_address: impl ToSocketAddrs){
    let state = Arc::new(AppState::default());
    let public_server = generate_server(
        public_server_address,
        public_app::generate_public_app(&state)
    );
    let admin_server = generate_server(
        admin_server_address,
        admin_app::generate_admin_app(&state)
    );
    tokio::join!(public_server, admin_server);
}

async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

