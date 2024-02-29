#[allow(unused_imports)]
use tracing::{error, debug, warn};
use std::sync::Arc;
use crate::{IpfsClientError, dtos};

pub type IpfsClientResult<T> = Result<T, IpfsClientError>;

#[derive(Default, Clone, Debug)]
pub struct IpfsNodeMetadata {
    pub gateway_address: String,
    pub rpc_address: String,
}

/// An IPFS client depend on Reqwest.
/// Safe to clone.
#[derive(Debug, Clone)]
pub struct ReqwestIpfsClient {
    pub client: reqwest::Client,
    /// Mutable.
    pub ipfs_node_metadata: Arc<parking_lot::RwLock<IpfsNodeMetadata>>,
}

impl ReqwestIpfsClient {
    pub fn new(metadata: IpfsNodeMetadata) -> Self {
        ReqwestIpfsClient {
            client: reqwest::Client::new(),
            ipfs_node_metadata: parking_lot::RwLock::new(metadata).into(),
        }
    }

    /// Get file from IPFS gateway.
    pub async fn get_file_by_gateway(&self, cid: &str, file_name: Option<&str>) -> IpfsClientResult<reqwest::Response> {
        let url = format!("http://{addr}/ipfs/{cid}?filename={file_name}&download=true",
                          addr = &self.ipfs_node_metadata.read().gateway_address,
                          cid = cid,
                          file_name = file_name.unwrap_or(cid)
        );

        let res = self.client
            .get(url)
            .send()
            .await.map_err(|_e| {
            error!("Send gateway request failed. msg: {:?}", _e);
            IpfsClientError::SendRequestFailed
        })?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                debug!("Success get file");
                Ok(res)
            }
            reqwest::StatusCode::NOT_FOUND => {
                error!("IPFS gateway not found");
                Err(IpfsClientError::NotFound)
            }
            _ => {
                warn!("IPFS gateway responds unhandled status code: {}", status);
                Err(IpfsClientError::UnknownStatusCode)
            }
        }
    }

    /// Get IPFS node's basic information.
    pub async fn get_id_info(&self) -> IpfsClientResult<dtos::IdResponse> {
        // TODO format arg无效
        let url_content = "/id";
        let res = self.ipfs_rpc_request(url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let peer_id = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;
                debug!("Success get id info");
                Ok(peer_id)
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// List all recursive pins that is pinned
    pub async fn list_recursive_pins_pinned(&self, with_pin_name: bool) -> IpfsClientResult<dtos::ListPinsResponse> {
        let url_content = if with_pin_name {
            "/pin/ls?type=recursive&names=true"
        } else {
            "/pin/ls?type=recursive"
        };
        let res = self.ipfs_rpc_request(url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                let pins = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;
                debug!("Success list recursive pins that is pinned");
                Ok(pins)
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Get a pin.
    pub async fn get_one_pin(&self, cid: &str, with_pin_name: bool) -> IpfsClientResult<Option<dtos::PinsInfoInList>> {
        let url_content = format!("/pin/ls?arg={cid}&names={with_pin_name}",
                                  cid = cid, with_pin_name = with_pin_name);
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                // pinned
                let pins: dtos::ListPinsResponse = res.json().await.map_err(|_e| {
                    error!("Unexpected response body. msg: {:?}", _e);
                    IpfsClientError::UnexpectedResponseBody
                })?;

                if pins.keys.len() != 1 {
                    error!("Unexpected response body. Should only respond one pin");
                    return Err(IpfsClientError::UnexpectedResponseBody);
                }

                debug!("Success get one pin");
                let pin_info = pins.keys.into_values().next().unwrap();
                Ok(Some(pin_info))
            }
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                // might not pinned
                let error_response: dtos::ErrorResponse = res.json().await.map_err(|_e| {
                    Self::handle_rpc_status_code_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                })?;

                if error_response.message.contains("is not pinned") {
                    // really not pinned
                    Ok(None)
                } else {
                    Err(Self::handle_rpc_status_code_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Add a recursive pin.
    pub async fn add_pin_recursive(&self, cid: &str, pin_name: Option<&str>) -> IpfsClientResult<()> {
        let pin_name = pin_name.unwrap_or("untitled");

        let url_content = format!("/pin/add?arg={cid}&name={pin_name}",
                                  cid = cid,
                                  pin_name = pin_name,
        );
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                debug!("Success add pin. cid: {}, pin_name: {}", cid, pin_name);
                Ok(())
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }

    /// Remove a recursive pin.
    pub async fn remove_pin_recursive(&self, cid: &str) -> IpfsClientResult<()> {
        let url_content = format!("/pin/rm?arg={cid}",
                                  cid = cid,
        );
        let res = self.ipfs_rpc_request(&url_content).await?;

        let status = res.status();
        match status {
            _ if status.is_success() => {
                debug!("Success remove pin. cid: {}", cid);
                Ok(())
            }
            err => Err(Self::handle_rpc_status_code_error(err))
        }
    }
}

impl Default for ReqwestIpfsClient {
    fn default() -> Self {
        let default_metadata = IpfsNodeMetadata {
            gateway_address: "127.0.0.1:8080".to_string(),
            rpc_address: "127.0.0.1:5001".to_string(),
        };
        ReqwestIpfsClient::new(default_metadata)
    }
}

/// Private tools
impl ReqwestIpfsClient {
    /// Request's url is `"http://{addr}/api/v0{url_content}"`.
    async fn ipfs_rpc_request(&self, url_content: &str) -> IpfsClientResult<reqwest::Response> {
        let url = format!("http://{addr}/api/v0{url_content}",
                          addr = &self.ipfs_node_metadata.read().rpc_address,
                          url_content = url_content,
        );
        debug!("IPFS RPC url: {}", url);

        self.client
            .post(url)
            .send()
            .await.map_err(|_e| {
            error!("Send RPC request failed. msg: {:?}", _e);
            IpfsClientError::SendRequestFailed
        })
    }

    /// Convert RPC status error into `ResponseError`,
    /// and output log.
    fn handle_rpc_status_code_error(status: reqwest::StatusCode) -> IpfsClientError {
        match status {
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                error!("IPFS RPC respond unexpected internal server error");
                IpfsClientError::RpcInternalServerError
            }
            reqwest::StatusCode::BAD_REQUEST => {
                error!("IPFS RPC bad request: malformed RPC, argument type error, etc");
                IpfsClientError::RpcReject
            }
            reqwest::StatusCode::FORBIDDEN => {
                error!("IPFS RPC call forbidden");
                IpfsClientError::RpcReject
            }
            reqwest::StatusCode::NOT_FOUND => {
                error!("IPFS RPC endpoint not found");
                IpfsClientError::NotFound
            }
            reqwest::StatusCode::METHOD_NOT_ALLOWED => {
                error!("IPFS RPC method not allowed");
                IpfsClientError::RpcReject
            }
            reqwest::StatusCode::BAD_GATEWAY => {
                error!("IPFS RPC server responded bad gateway");
                IpfsClientError::NotFound
            }
            status => {
                warn!("IPFS RPC responded unhandled status code: {}", status);
                IpfsClientError::UnknownStatusCode
            }
        }
    }
}
