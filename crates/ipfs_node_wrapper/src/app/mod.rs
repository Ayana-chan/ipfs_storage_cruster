

use tiny_ipfs_client::ReqwestIpfsClient;

pub mod admin_app;
pub mod public_app;

/// Public state among all apps.
/// Should never be Cloned.
#[derive(Default, Debug)]
pub struct AppState {
    /// Contact IPFS node.
    pub ipfs_client: ReqwestIpfsClient,
    /// Count the number of downloads of files. `cid -> count`.
    pub file_traffic_counter: scc::HashMap<String, usize>,
}


