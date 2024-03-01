use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use ipfs_storage_cruster_manager::app_builder;

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
fn read_config() -> app_builder::AppConfig {
    app_builder::AppConfigBuilder::new()
        .server_ip("127.0.0.1".parse().unwrap())
        .server_port(5000)
        .finish()
}

#[tokio::main]
async fn main() {
    config_tracing();
    app_builder::serve(read_config()).await;
}
