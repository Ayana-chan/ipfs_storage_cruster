mod normal;
mod pin;
mod traffic;

pub use pin::*;
pub use traffic::*;

pub struct IpfsNodeWrapperAdminClient {
    client: reqwest::Client,
}

impl IpfsNodeWrapperAdminClient {
    pub fn new() -> Self {
        IpfsNodeWrapperAdminClient {
            client: reqwest::Client::new(),
        }
    }
}
