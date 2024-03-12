use config::Config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing::Level;
use ipfs_storage_cruster_manager::app_builder;

fn config_tracing(){
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_env("APP_LOG"));

    let file_appender = RollingFileAppender::new(
        Rotation::HOURLY,
        "logs",
        "ipfs_storage_cruster_manager.log");
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

fn read_config() -> app_builder::AppConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("./crates/ipfs_storage_cruster_manager_app/Settings").required(false))
        .add_source(config::File::with_name("./Settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings.try_deserialize().unwrap()
}

#[tokio::main]
async fn main() {
    config_tracing();
    app_builder::serve(read_config()).await;
}
