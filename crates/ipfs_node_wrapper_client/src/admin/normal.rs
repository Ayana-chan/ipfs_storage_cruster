use ipfs_node_wrapper_structs::admin::dtos;
use crate::admin::IpfsNodeWrapperAdminClient;
use crate::common::StandardClientResult;
use crate::client_tools::handle_client_response;

impl IpfsNodeWrapperAdminClient {
    pub async fn get_ipfs_node_info(&self) -> StandardClientResult<dtos::GetIpfsNodeInfoResponse> {
        let res = self.client.get(self.url.clone())
            .send().await;
        handle_client_response(res).await
    }
}
