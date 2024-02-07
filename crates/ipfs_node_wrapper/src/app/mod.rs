use axum::Router;
use tokio::net::ToSocketAddrs;

mod public_app;
mod admin_app;

pub async fn serve(){
    let public_server = generate_server(
        ("127.0.0.1", 3000),
        public_app::generate_public_app()
    );
    let admin_server = generate_server(
        ("127.0.0.1", 4000),
        admin_app::generate_admin_app()
    );
    tokio::join!(public_server, admin_server);
}

async fn generate_server(address: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

