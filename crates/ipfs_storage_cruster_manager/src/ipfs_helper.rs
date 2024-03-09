#[allow(unused_imports)]
use tracing::{error, debug, warn, info, trace};
use tiny_ipfs_client::ReqwestIpfsClient;

static CACHE_PINS_INTERVAL_TIME_MS: u64 = 500;

/// Regularly try until get peer id.
#[tracing::instrument(skip_all)]
pub async fn get_peer_id_until_success(ipfs_client: &ReqwestIpfsClient) -> String {
    loop {
        let res = ipfs_client.get_id_info().await;
        match res {
            Ok(res) => {
                return res.id;
            }
            Err(_e) => {
                error!("Failed to cache recursive pins. Try again in {} ms. msg: {:?}", CACHE_PINS_INTERVAL_TIME_MS, _e);
                tokio::time::sleep(tokio::time::Duration::from_millis(CACHE_PINS_INTERVAL_TIME_MS)).await;
            }
        };
    }
}
