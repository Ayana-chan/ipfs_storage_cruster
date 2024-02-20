use std::sync::Arc;
#[allow(unused_imports)]
use tracing::{error, debug};
use reqwest::{Client, Response, StatusCode};
use crate::app::AppConfig;
use crate::error;
use crate::common::ApiResult;

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

impl ReqwestIpfsClient {
    /// Create ipfs client by `AppConfig`
    pub fn new_from_config(app_config: &AppConfig) -> Self {
        ReqwestIpfsClient {
            client: reqwest::Client::new(),
            ipfs_node_metadata: parking_lot::RwLock::new(IpfsNodeMetadata {
                gateway_address: app_config.ipfs_gateway_address.to_string(),
                rpc_address: app_config.ipfs_rpc_address.to_string(),
            }).into(),
        }
    }

    /// Get file from IPFS gateway.
    pub async fn get_file_by_gateway(&self, cid: &str, file_name: Option<&str>) -> ApiResult<Response> {
        let url = format!("http://{addr}/ipfs/{cid}?filename={file_name}&download=true",
                          addr = &self.ipfs_node_metadata.read().gateway_address,
                          cid = cid,
                          file_name = file_name.unwrap_or(cid)
        );

        let res = self.client
            .get(url)
            .send()
            .await.map_err(|_e| {
            error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
        }
        )?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                debug!("Success get file");
                Ok(res)
            }
            StatusCode::NOT_FOUND => {
                Err(error::IPFS_GATEWAY_NOT_FOUND.clone_to_error_with_log())
            }
            _ => {
                Err(error::IPFS_UNKNOWN_ERROR.clone_to_error_with_log())
            }
        }
    }

    /// Send `/pin/add` RPC to add a recursive pin object.
    pub async fn add_pin_recursive(&self, cid: &str, pin_name: Option<&str>) -> ApiResult<Response> {
        let pin_name = pin_name.unwrap_or("untitled");

        let url = format!("http://{addr}/api/v0/pin/add?arg={cid}&name={pin_name}",
                          addr = &self.ipfs_node_metadata.read().rpc_address,
                          cid = cid,
                          pin_name = pin_name,
        );
        // debug!("add pin url: {}", url);

        let res = self.client
            .post(url)
            .send()
            .await.map_err(|_e| {
            error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
        }
        )?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                debug!("Success add pin. cid: {}, pin_name: {}", cid, pin_name);
                Ok(res)
            }
            err => Err(handle_rpc_error(err))
        }
    }

    /// Get IPFS node's peer ID.
    pub async fn get_id(&self) -> ApiResult<String> {
        let url = format!("http://{addr}/api/v0/id?format=\"<id>\\n\"",
                          addr = &self.ipfs_node_metadata.read().rpc_address,
        );

        let res = self.client
            .post(url)
            .send()
            .await.map_err(|_e| {
            error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
        }
        )?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let peer_id = res.text().await.map_err(|_e| {
                    error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
                })?;
                debug!("Success get peer id");
                Ok(peer_id)
            }
            err => Err(handle_rpc_error(err))
        }
    }
}

/// Convert RPC status error into `ResponseError`,
/// and output log.
fn handle_rpc_error(status: StatusCode) -> error::ResponseError {
    match status {
        StatusCode::INTERNAL_SERVER_ERROR => error::IPFS_RPC_INTERNAL_ERROR.clone_to_error_with_log(),
        StatusCode::BAD_REQUEST => {
            error!("IPFS Bad Request: malformed RPC, argument type error, etc");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        StatusCode::FORBIDDEN => {
            error!("IPFS RPC call forbidden");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        StatusCode::NOT_FOUND => error::IPFS_RPC_NOT_FOUND.clone_to_error_with_log(),
        StatusCode::METHOD_NOT_ALLOWED => {
            error!("IPFS RPC method not allowed");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        _ => error::IPFS_UNKNOWN_ERROR.clone_to_error_with_log(),
    }
}

