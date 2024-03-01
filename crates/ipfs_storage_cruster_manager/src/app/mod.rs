use std::sync::Arc;
use tiny_ipfs_client::{IpfsNodeMetadata, ReqwestIpfsClient};
use crate::app_builder::AppConfig;

pub mod handlers;

/// State of app. Should be cheap and safe to clone.
#[derive(Default, Debug)]
pub struct AppState {
    /// Contact IPFS node.
    pub ipfs_client: Arc<ReqwestIpfsClient>,
}

impl AppState {
    pub fn from_app_config(app_config: AppConfig) -> AppState {
        let ipfs_metadata = IpfsNodeMetadata {
            gateway_address: "".to_string(),
            rpc_address: app_config.ipfs_rpc_address.to_string(),
        };
        AppState {
            ipfs_client: ReqwestIpfsClient::new(ipfs_metadata).into(),
        }
    }
}
