use std::collections::HashMap;
#[allow(unused_imports)]
use tracing::{trace, debug, info};
use ipfs_node_wrapper_client::IpfsNodeWrapperClient;
use tiny_ipfs_client::{IpfsClientResult, ReqwestIpfsClient};

mod dtos;
mod models;
mod common;

/// TODO 没啥用
pub struct IpfsOperator {
    pub ipfs_client: ReqwestIpfsClient,
    pub ipfs_node_wrapper_client: IpfsNodeWrapperClient,
}

impl IpfsOperator {
    pub fn new(rpc_address: String, wrapper_address: String, client: reqwest::Client) -> Self {
        IpfsOperator {
            ipfs_client: ReqwestIpfsClient::new_with_reqwest_client(rpc_address, client.clone()),
            ipfs_node_wrapper_client: IpfsNodeWrapperClient::new_with_reqwest_client(wrapper_address, client),
        }
    }

    /// Get IPFS node's id.
    pub async fn get_ipfs_node_id(&self) -> IpfsClientResult<String> {
        let peer_id_res = self.ipfs_client.get_id_info().await?;
        trace!("peer_id_res: {:?}", peer_id_res);
        Ok(peer_id_res.id)
    }

    // TODO 本地统计流量就行了，分点统计还可能导致计算麻烦。不过依然可以留着当做示例
    /// Get a list of the number of times files has been downloaded.
    ///
    /// Ignore error type.
    pub async fn get_download_time_list(&self) -> Result<HashMap<String, usize>, ()> {
        let res = self.ipfs_node_wrapper_client.get_download_time_list().await;
        res.map_or(Err(()), |v| Ok(v.list))
    }

    /// List all recursive pins that is pinned in IPFS node.
    pub async fn list_succeeded_pins(&self) -> IpfsClientResult<Vec<String>> {
        let list_res = self.ipfs_client
            .list_recursive_pins_pinned(false).await?;
        let cids = list_res.keys.into_keys().collect();
        Ok(cids)
    }
}
