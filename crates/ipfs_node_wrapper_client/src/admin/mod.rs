mod normal;
mod pin;
mod traffic;

pub use pin::*;
pub use traffic::*;

pub struct IpfsNodeWrapperAdminClient {
    client: reqwest::Client,
    url: String,
}

impl IpfsNodeWrapperAdminClient {
    pub fn new(url: String) -> Self {
        IpfsNodeWrapperAdminClient {
            client: reqwest::Client::new(),
            url,
        }
    }

    pub fn url(&self) -> &str{
        &self.url
    }
}
