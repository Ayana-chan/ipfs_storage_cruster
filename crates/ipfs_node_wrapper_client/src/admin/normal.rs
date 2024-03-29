use ipfs_node_wrapper_structs::admin::dtos;
use crate::admin::IpfsNodeWrapperAdminClient;
use crate::common::StandardClientResult;
use crate::client_tools::handle_client_response;

impl IpfsNodeWrapperAdminClient {
    /// Get IPFS node's information.
    pub async fn get_ipfs_node_info(&self) -> StandardClientResult<dtos::GetIpfsNodeInfoResponse> {
        let url = self.generate_url("/api/info");
        let res = self.client.get(url)
            .send().await;
        handle_client_response(res).await
    }
}
