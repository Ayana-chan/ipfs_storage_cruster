use std::collections::HashMap;
#[allow(unused_imports)]
use tracing::{trace, debug, info};
use ipfs_node_wrapper_client::ipfs_node_wrapper_structs::StandardApiResult;
use ipfs_node_wrapper_client::IpfsNodeWrapperClient;
use tiny_ipfs_client::{IpfsClientResult, ReqwestIpfsClient};

mod dtos;
mod models;
mod common;

pub struct IpfsOperator {
    pub ipfs_client: ReqwestIpfsClient,
    pub ipfs_node_wrapper_client: IpfsNodeWrapperClient,
}

impl IpfsOperator {
    /// Get IPFS node's id.
    pub async fn get_ipfs_node_id(&self) -> IpfsClientResult<String> {
        let peer_id_res = self.ipfs_client.get_id_info().await?;
        trace!("peer_id_res: {:?}", peer_id_res);
        Ok(peer_id_res.id)
    }

    /// Get a list of the number of times files has been downloaded.
    ///
    /// Ignore error type.
    pub async fn get_download_time_list(&self) -> Result<HashMap<String, usize>, ()> {
        let res = self.ipfs_node_wrapper_client.get_download_time_list().await;
        res.map_or(Err(()), |v| Ok(v.list))
    }
}
