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

#[derive(Default, Debug)]
pub struct IpfsClient {
    pub client: Client,
    pub ipfs_node_metadata: parking_lot::RwLock<IpfsNodeMetadata>,
}

impl IpfsClient {
    /// Create ipfs client by `AppConfig`
    pub fn new_from_config(app_config: &AppConfig) -> Self {
        IpfsClient {
            client: reqwest::Client::new(),
            ipfs_node_metadata: parking_lot::RwLock::new(IpfsNodeMetadata {
                gateway_address: app_config.ipfs_gateway_address.to_string(),
                rpc_address: app_config.ipfs_rpc_address.to_string(),
            }),
        }
    }

    /// Get file from IPFS gateway.
    pub async fn get_file_gateway(&self, cid: &str, file_name: Option<&str>) -> ApiResult<Response> {
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
        return match status {
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
        };
    }

    /// Send `/pin/add` RPC to add a recursive pin object.
    pub async fn add_pin_recursive(&self, cid: &str, pin_name: Option<&str>) -> ApiResult<Response> {
        let pin_name = pin_name.unwrap_or("untitled");

        let url = format!("http://{addr}/api/v0/pin/add?arg={cid}&name={pin_name}",
                          addr = &self.ipfs_node_metadata.read().gateway_address,
                          cid = cid,
                          pin_name = pin_name,
        ); //TODO progress

        let res = self.client
            .post(url)
            .send()
            .await.map_err(|_e| {
            error::IPFS_COMMUCATION_FAIL.clone_to_error_with_log_error(_e)
        }
        )?;

        let status = res.status();
        return match status {
            _ if status.is_success() => {
                debug!("Success add pin");
                Ok(res)
            }
            err => Err(handle_rpc_error(err))
        };
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

