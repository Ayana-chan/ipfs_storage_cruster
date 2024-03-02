pub mod admin;
pub mod common;
mod client_tools;
mod client;

pub use ipfs_node_wrapper_structs;

pub struct IpfsNodeWrapperClient {
    pub base_url: String,
    client: reqwest::Client,
}

impl IpfsNodeWrapperClient {
    /// Arg example: "127.0.0.1:4000"
    pub fn new(address: String) -> Self {
        IpfsNodeWrapperClient {
            base_url: address,
            client: reqwest::Client::new(),
        }
    }
}

/// private tools
impl IpfsNodeWrapperClient {
    /// Generate `http://{base_url}{url_content}`
    fn generate_url(&self, url_content: &str) -> String {
        format!("http://{base_url}{url_content}",
                base_url = self.base_url,
                url_content = url_content)
    }
}

