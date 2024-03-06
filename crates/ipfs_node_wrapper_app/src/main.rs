use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
#[allow(unused_imports)]
use tracing::{error, debug, info};
use ipfs_node_wrapper::app_builder;

#[tokio::main]
async fn main() {
    config_tracing();
    app_builder::serve(read_config()).await;
}

fn config_tracing() {
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout);

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
    let mut builder = app_builder::AppConfigBuilder::new();

    let ev = std::env::var("IPFS_GATEWAY_ADDRESS");
    if let Ok(address) = ev {
        debug!("Succeed to read env IPFS_GATEWAY_ADDRESS: {:?}", address);
        builder = builder.ipfs_gateway_address(address.to_string());
    }

    let ev = std::env::var("IPFS_RPC_ADDRESS");
    if let Ok(address) = ev {
        debug!("Succeed to read env IPFS_RPC_ADDRESS: {:?}", address);
        builder = builder.ipfs_rpc_address(address.to_string());
    }

    // let ev = option_env!("IPFS_GATEWAY_ADDRESS");
    // if let Some(ev) = ev {
    //     let address = std::net::SocketAddr::from_str(ev);
    //     match address {
    //         Ok(addr) => {
    //             debug!("Succeed to read env IPFS_GATEWAY_ADDRESS: {:?}", address);
    //             builder = builder.ipfs_gateway_address(addr);
    //         }
    //         Err(e) => {
    //             error!("Env IPFS_GATEWAY_ADDRESS is not an invalid SocketAddr. Default value used. Value: {}, error msg: {}", ev, e);
    //         }
    //     }
    // }
    //
    // let ev = option_env!("IPFS_RPC_ADDRESS");
    // if let Some(ev) = ev {
    //     let address = std::net::SocketAddr::from_str(ev);
    //     match address {
    //         Ok(addr) => {
    //             debug!("Succeed to read env IPFS_RPC_ADDRESS: {:?}", address);
    //             builder = builder.ipfs_rpc_address(addr);
    //         }
    //         Err(e) => {
    //             error!("Env IPFS_RPC_ADDRESS is not an invalid SocketAddr. Default value used. Value: {}, error msg: {}", ev, e);
    //         }
    //     }
    // }

    builder.finish()
}
