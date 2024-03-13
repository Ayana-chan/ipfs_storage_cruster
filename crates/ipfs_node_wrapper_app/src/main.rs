use config::Config;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
#[allow(unused_imports)]
use tracing::{error, debug, info};
use tracing_subscriber::{Layer, EnvFilter};
use ipfs_node_wrapper::app_builder;

#[tokio::main]
async fn main() {
    config_tracing();
    app_builder::serve(read_config()).await;
}

fn config_tracing() {
    let env_filter = EnvFilter::try_from_env("APP_LOG")
        .unwrap_or_else(|_| EnvFilter::new("debug,ipfs_node_wrapper=trace,hyper=debug"));

    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(env_filter);

    let file_appender = RollingFileAppender::new(
        Rotation::HOURLY,
        "logs",
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

#[tracing::instrument(skip_all)]
fn read_config() -> app_builder::AppConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("./crates/ipfs_node_wrapper_app/Settings").required(false))
        .add_source(config::File::with_name("./Settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings.try_deserialize().unwrap()
}
