use tracing_subscriber::layer::SubscriberExt;

mod app;
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

#[tokio::main]
async fn main() {
    config_tracing();
    app::serve(("127.0.0.1", 3000), ("127.0.0.1", 4000)).await;
}

