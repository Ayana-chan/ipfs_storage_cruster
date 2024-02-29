use crate::admin::IpfsNodeWrapperAdminClient;
use crate::client_tools::handle_client_response;
use crate::common::StandardClientResult;
use crate::ipfs_node_wrapper_structs::admin::dtos;

impl IpfsNodeWrapperAdminClient {
    /// Get a list of the number of times files has been downloaded.
    pub async fn get_download_time_list(&self) -> StandardClientResult<dtos::GetDownloadTimeListResponse> {
        let url = self.generate_url("/api/traffic");
        let res = self.client.get(url)
            .send().await;
        handle_client_response(res).await
    }
}