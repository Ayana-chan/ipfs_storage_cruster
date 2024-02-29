use ipfs_node_wrapper_structs::admin::dtos;
use crate::admin::IpfsNodeWrapperAdminClient;
use crate::client_tools::handle_client_response;
use crate::common::StandardClientResult;

impl IpfsNodeWrapperAdminClient {
    /// List all recursive pins that is pinned in IPFS node.
    pub async fn list_succeeded_pins(&self) -> StandardClientResult<dtos::ListSucceededPinsResponse> {
        let url = self.generate_url("/api/pin");
        let res = self.client.get(url)
            .send().await;
        handle_client_response(res).await
    }

    /// Check status of adding pin.
    /// Just query local recorder, so maybe return `Failed` when not found.
    pub async fn check_pin(&self, cid: &str) -> StandardClientResult<dtos::CheckPinResponse> {
        let url_content = format!("/api/pin/{}", cid);
        let url = self.generate_url(&url_content);
        let res = self.client.get(url)
            .send().await;
        handle_client_response(res).await
    }

    /// Add a pin to IPFS node.
    ///
    /// Return immediately.
    pub async fn add_pin_background(&self, cid: String, name: Option<String>) -> StandardClientResult<()> {
        let args = dtos::AddPinArgs {
            cid,
            name,
            background: None,
        };
        let url = self.generate_url("/api/pin");
        let res = self.client.post(url)
            .json(&args)
            .send().await;
        handle_client_response(res).await
    }

    /// Add a pin to IPFS node.
    ///
    /// Wouldn't return until pin finishes.
    pub async fn add_pin_sync(&self, cid: String, name: Option<String>) -> StandardClientResult<()> {
        let args = dtos::AddPinArgs {
            cid,
            name,
            background: Some(false),
        };
        let url = self.generate_url("/api/pin");
        let res = self.client.post(url)
            .json(&args)
            .send().await;
        handle_client_response(res).await
    }

    /// Remove a pin.
    ///
    /// Return immediately. Possible failure.
    pub async fn rm_pin(&self, cid: String) -> StandardClientResult<()> {
        let args = dtos::RemovePinArgs {
            cid,
        };
        let url = self.generate_url("/api/pin");
        let res = self.client.delete(url)
            .json(&args)
            .send().await;
        handle_client_response(res).await
    }
}