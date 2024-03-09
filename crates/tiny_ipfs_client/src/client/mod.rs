#[allow(unused_imports)]
use tracing::{error, debug, warn};
use crate::IpfsClientError;

mod apis;

pub type IpfsClientResult<T> = Result<T, IpfsClientError>;

/// An IPFS client depend on `reqwest`.
#[derive(Debug, Clone)]
pub struct ReqwestIpfsClient {
    #[cfg(not(feature = "no_gateway"))]
    pub gateway_address: String,
    pub rpc_address: String,
    client: reqwest::Client,
}

impl ReqwestIpfsClient {
    /// New with a new reqwest client.
    pub fn new(
        #[cfg(not(feature = "no_gateway"))] gateway_address: String,
        rpc_address: String) -> Self {
        ReqwestIpfsClient {
            #[cfg(not(feature = "no_gateway"))]
            gateway_address,
            rpc_address,
            client: reqwest::Client::new(),
        }
    }

    /// Relatively cheap to create (only address changed).
    pub fn new_with_reqwest_client(
        #[cfg(not(feature = "no_gateway"))] gateway_address: String,
        rpc_address: String, client: reqwest::Client) -> Self {
        ReqwestIpfsClient {
            #[cfg(not(feature = "no_gateway"))]
            gateway_address,
            rpc_address,
            client,
        }
    }
}

impl Default for ReqwestIpfsClient {
    fn default() -> Self {
        #[cfg(not(feature = "no_gateway"))]
            {
                ReqwestIpfsClient::new("127.0.0.1:8080".to_string(), "127.0.0.1:5001".to_string())
            }
        #[cfg(feature = "no_gateway")]
            {
                ReqwestIpfsClient::new("127.0.0.1:5001".to_string())
            }
    }
}

/// Private tools
impl ReqwestIpfsClient {
    /// Request's url is `"http://{addr}/api/v0{url_content}"`.
    async fn ipfs_rpc_request(&self, url_content: &str) -> IpfsClientResult<reqwest::Response> {
        let url = format!("http://{addr}/api/v0{url_content}",
                          addr = &self.rpc_address,
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
                error!("IPFS bad request: invalid cid, malformed RPC, argument type error, etc");
                IpfsClientError::BadRequest
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
