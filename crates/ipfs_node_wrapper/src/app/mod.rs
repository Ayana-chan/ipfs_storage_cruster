use std::sync::Arc;
use axum::Router;
use tokio::net::ToSocketAddrs;
use crate::app::admin_app::AdminAppState;
use crate::app::public_app::PublicAppState;
use crate::ipfs_client::IpfsNodeMetadata;

mod public_app;
mod admin_app;

#[derive(Default, Clone, Debug)]
pub struct AppState {
    ipfs_node_metadata: Arc<parking_lot::RwLock<IpfsNodeMetadata>>,
}

pub async fn serve(public_server_address: impl ToSocketAddrs, admin_server_address: impl ToSocketAddrs){
    let app_state = Arc::new(AppState::default());
    let public_app_state = PublicAppState {
        app_state: app_state.clone(),
    };
    let admin_app_state = AdminAppState {
        app_state: app_state.clone(),
    };

    let public_server = generate_server(
        public_server_address,
        public_app::generate_public_app().with_state(public_app_state)
    );
    let admin_server = generate_server(
        admin_server_address,
        admin_app::generate_admin_app().with_state(admin_app_state)
    );
    tokio::join!(public_server, admin_server);
}

async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
