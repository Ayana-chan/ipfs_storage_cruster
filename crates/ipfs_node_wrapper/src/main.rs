use axum::Router;
use tokio::net::ToSocketAddrs;
use tracing_subscriber::layer::SubscriberExt;

mod public_app;
mod admin_app;
mod models;
mod ipfs_client;

fn config_tracing(){
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout);
    let log_file = std::fs::File::create("log.txt").unwrap();
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(log_file)
        .with_ansi(false);

    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn generate_server(addr: impl ToSocketAddrs, app: Router) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[tokio::main]
async fn main() {
    config_tracing();
    let public_server = generate_server(("127.0.0.1", 3000), public_app::generate_public_app());
    let admin_server = generate_server(("127.0.0.1", 4000), admin_app::generate_admin_app());
    tokio::join!(public_server, admin_server);
}

