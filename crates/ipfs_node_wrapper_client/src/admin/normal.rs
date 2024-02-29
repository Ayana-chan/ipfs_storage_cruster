use ipfs_node_wrapper_structs::admin::dtos;
use crate::admin::IpfsNodeWrapperAdminClient;
use crate::common::StandardClientResult;
use crate::client_tools::handle_client_response;

impl IpfsNodeWrapperAdminClient {
    pub async fn get_ipfs_node_info(&self) -> StandardClientResult<dtos::GetIpfsNodeInfoResponse> {
        let url = format!("http://{base_url}/api/info", base_url = self.base_url);
        let res = self.client.get(url)
            .send().await;
        handle_client_response(res).await
    }
}
