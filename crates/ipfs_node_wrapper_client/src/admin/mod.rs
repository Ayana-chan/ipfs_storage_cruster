mod normal;
mod pin;
mod traffic;

pub use pin::*;
pub use traffic::*;

pub struct IpfsNodeWrapperAdminClient {
    client: reqwest::Client,
    pub base_url: String,
}

// TODO 使用更多数据结构来初始化； 默认值
impl IpfsNodeWrapperAdminClient {
    /// Arg example: "127.0.0.1:4000"
    pub fn new(address: String) -> Self {
        IpfsNodeWrapperAdminClient {
            client: reqwest::Client::new(),
            base_url: address,
        }
    }
}

/// private tools
impl IpfsNodeWrapperAdminClient {
    /// Generate `http://{base_url}{url_content}`
    fn generate_url(&self, url_content: &str) -> String {
        format!("http://{base_url}{url_content}",
                base_url = self.base_url,
                url_content = url_content)
    }
}
