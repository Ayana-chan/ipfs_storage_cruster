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
    pub fn new_from_config(app_config: &AppConfig) -> Self {
        IpfsClient {
            client: reqwest::Client::new(),
            ipfs_node_metadata: parking_lot::RwLock::new(IpfsNodeMetadata {
                gateway_address: app_config.ipfs_gateway_address.to_string(),
                rpc_address: app_config.ipfs_rpc_address.to_string(),
            }),
        }
    }

    pub async fn ipfs_get_file(&self, cid: &str, file_name: Option<&str>) -> ApiResult<Response> {
        let url = format!("http://{addr}/ipfs/{cid}?filename={file_name}&download=true",
                          addr = &self.ipfs_node_metadata.read().gateway_address,
                          cid = cid,
                          file_name = file_name.unwrap_or(cid)
        );

        let res = self.client
            .get(url)
            .send()
            .await.map_err(|_e| {
            error!("Fail to contact IPFS node: {:?}", _e);
            error::IPFS_COMMUCATION_FAIL.clone_to_error()
        }
        )?;

        let status = res.status();
        return match status {
            _ if status.is_success() => {
                debug!("Success contact IPFS node");
                Ok(res)
            }
            StatusCode::NOT_FOUND => {
                error!("IPFS node unreachable");
                Err(error::IPFS_NOT_FOUND.clone_to_error())
            }
            _ => {
                error!("IPFS node respond an unknown status code: {}", status.to_string());
                Err(error::IPFS_UNKNOWN_ERROR.clone_to_error())
            }
        };
    }
}
