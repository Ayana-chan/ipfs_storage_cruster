use ipfs_node_wrapper_structs::StandardApiJsonBody;
use crate::client_tools::handle_client_response;
use crate::common::ClientResult;
use crate::ipfs_node_wrapper_structs::models;
use crate::IpfsNodeWrapperClient;

impl IpfsNodeWrapperClient {
    /// Get a list of the number of times files has been downloaded.
    pub async fn get_download_time_list(&self) -> ClientResult<models::GetDownloadTimeListResponse> {
        let url = self.generate_url("/api/traffic");
        let res = self.client.get(url)
            .send().await;
        let result: StandardApiJsonBody<models::GetDownloadTimeListResponse> = handle_client_response(res).await?;
        Ok(result.data)
    }
}