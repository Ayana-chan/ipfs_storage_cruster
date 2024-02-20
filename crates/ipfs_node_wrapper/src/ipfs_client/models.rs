use std::sync::Arc;
use reqwest::Client;
use serde::Deserialize;

#[derive(Default, Clone, Debug)]
pub struct IpfsNodeMetadata {
    pub gateway_address: String,
    pub rpc_address: String,
}

/// Safe to clone.
#[derive(Default, Debug, Clone)]
pub struct ReqwestIpfsClient {
    pub client: Client,
    pub ipfs_node_metadata: Arc<parking_lot::RwLock<IpfsNodeMetadata>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub public_key: String,
    pub addresses: Vec<String>,
    pub agent_version: String,
    pub protocols: Vec<String>,
}