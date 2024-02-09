use tracing::{error, debug};
use reqwest::{Response, StatusCode};
use crate::error;
use crate::common::ApiResult;

#[derive(Default, Clone, Debug)]
pub struct IpfsNodeMetadata {
    pub gateway_address: String,
    pub rpc_address: String,
}

// TODO 建立struct来访问ipfs，然后把结构体存在state中

#[tracing::instrument(skip_all)]
pub async fn ipfs_get_file(cid: &str, file_name: Option<&str>, ipfs_node_metadata: &parking_lot::RwLock<IpfsNodeMetadata>) -> ApiResult<Response> {
    let url = format!("http://{addr}/ipfs/{cid}?filename={file_name}&download=true",
                      addr = &ipfs_node_metadata.read().gateway_address,
                      cid = cid,
                      file_name = file_name.unwrap_or(cid)
    );

    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await.map_err(|_e| {
        error!("Fail to contact IPFS node: {:?}", _e);
        error::IPFS_COMMUCATION_FAIL.clone()
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
            Err(error::IPFS_NOT_FOUND.clone().into())
        }
        _ => {
            error!("IPFS node respond an unknown status code: {}", status.to_string());
            Err(error::IPFS_UNKNOWN_ERROR.clone().into())
        }
    };
}
