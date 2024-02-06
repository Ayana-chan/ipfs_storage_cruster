use axum::{
    routing::get,
    Router,
};
use tokio::net::ToSocketAddrs;

async fn generate_public_server(addr: impl ToSocketAddrs) {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn generate_admin_server(addr: impl ToSocketAddrs) {
    let app = Router::new().route("/", get(|| async { "Soyorin Love!" }));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[tokio::main]
async fn main() {
    let public_server = generate_public_server(("127.0.0.1", 3000));
    let admin_server = generate_admin_server(("127.0.0.1", 4000));
    tokio::join!(public_server, admin_server);
}

