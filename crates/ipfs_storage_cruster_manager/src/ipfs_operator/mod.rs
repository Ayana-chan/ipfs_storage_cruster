#[allow(unused_imports)]
use tracing::{trace, debug, info};
use tiny_ipfs_client::{IpfsClientResult, ReqwestIpfsClient};

mod dtos;
mod models;
mod common;

pub struct IpfsOperator {
    ipfs_client: ReqwestIpfsClient,
}

impl IpfsOperator {
    /// Get IPFS node's information.
    pub async fn get_ipfs_node_info(&self) -> IpfsClientResult<String> {
        let peer_id_res = self.ipfs_client.get_id_info().await?;
        trace!("peer_id_res: {:?}", peer_id_res);
        Ok(peer_id_res.id)
    }
}
