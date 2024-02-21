#[allow(unused_imports)]
use tracing::{error, debug, warn};
use crate::app::AppConfig;
use crate::error;
use crate::common::ApiResult;
pub use models::*;

mod models;

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
    pub async fn get_file_by_gateway(&self, cid: &str, file_name: Option<&str>) -> ApiResult<reqwest::Response> {
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
            reqwest::StatusCode::NOT_FOUND => {
                Err(error::IPFS_GATEWAY_NOT_FOUND.clone_to_error_with_log())
            }
            _ => {
                warn!("IPFS gateway responded unhandled status code: {}", status);
                Err(error::IPFS_UNKNOWN_ERROR.clone_to_error_with_log())
            }
        }
    }

    /// Send `/pin/add` RPC to add a recursive pin object.
    pub async fn add_pin_recursive(&self, cid: &str, pin_name: Option<&str>) -> ApiResult<reqwest::Response> {
        let pin_name = pin_name.unwrap_or("untitled");

        let url_content = format!("/pin/add?arg={cid}&name={pin_name}",
                          cid = cid,
                          pin_name = pin_name,
        );
        let res = self.ipfs_rpc_request(url_content).await?;

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
    pub async fn get_id_info(&self) -> ApiResult<IdResponse> {
        // TODO format arg无效
        let res = self.ipfs_rpc_request("/id".to_string()).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let peer_id = res.json().await.map_err(|_e| {
                    error::IPFS_FAIL.clone_to_error_with_log_error(_e)
                })?;
                debug!("Success get peer id");
                Ok(peer_id)
            }
            err => Err(handle_rpc_error(err))
        }
    }

    /// Request's url is `"http://{addr}/api/v0{url_content}"`.
    async fn ipfs_rpc_request(&self, url_content: String) -> ApiResult<reqwest::Response> {
        let url = format!("http://{addr}/api/v0{url_content}",
                          addr = &self.ipfs_node_metadata.read().rpc_address,
                          url_content = url_content,
        );
        debug!("IPFS RPC url: {}", url);

        self.client
            .post(url)
            .send()
            .await.map_err(|_e| {
            error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
        }
        )
    }
}

/// Convert RPC status error into `ResponseError`,
/// and output log.
fn handle_rpc_error(status: reqwest::StatusCode) -> error::ResponseError {
    match status {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => error::IPFS_RPC_INTERNAL_ERROR.clone_to_error_with_log(),
        reqwest::StatusCode::BAD_REQUEST => {
            error!("IPFS Bad Request: malformed RPC, argument type error, etc");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        reqwest::StatusCode::FORBIDDEN => {
            error!("IPFS RPC call forbidden");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        reqwest::StatusCode::NOT_FOUND => error::IPFS_RPC_NOT_FOUND.clone_to_error_with_log(),
        reqwest::StatusCode::METHOD_NOT_ALLOWED => {
            error!("IPFS RPC method not allowed");
            error::IPFS_RPC_REJECT.clone_to_error()
        }
        reqwest::StatusCode::BAD_GATEWAY => {
            error!("IPFS RPC server responded Bad Gateway");
            error::IPFS_RPC_INTERNAL_ERROR.clone_to_error()
        }
        status => {
            warn!("IPFS RPC responded unhandled status code: {}", status);
            error::IPFS_UNKNOWN_ERROR.clone_to_error_with_log()
        },
    }
}




