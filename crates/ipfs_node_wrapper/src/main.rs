use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;

mod utils;
mod app;
mod error;
mod ipfs_client;
mod common;

fn config_tracing(){
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout);

    let file_appender = RollingFileAppender::new(
        Rotation::HOURLY,
        "log",
        "ipfs_node_wrapper.log");
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false);
    // let log_file = std::fs::File::create("log.txt").unwrap();
    // let file_subscriber = tracing_subscriber::fmt::layer()
    //     .with_writer(log_file)
    //     .with_ansi(false);

    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

// TODO 可配置化
fn read_config() -> app::AppConfig {
    app::AppConfigBuilder::new()
        .public_server_ip("127.0.0.1".parse().unwrap())
        .public_server_port(3000)
        .admin_server_ip("127.0.0.1".parse().unwrap())
        .admin_server_port(4000)
        .finish()
}

#[tokio::main]
async fn main() {
    config_tracing();
    app::serve(read_config()).await;
}

