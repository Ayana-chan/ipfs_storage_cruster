use tracing::trace;

// TODO 变成可变
pub static IPFS_NODE_ADDRESS: &str = "127.0.0.1";
pub static IPFS_NODE_GATEWAY_PORT: u16 = 8080;
pub static IPFS_NODE_RPC_PORT: u16 = 5001;

pub async fn ipfs_get_file(cid: &str) -> Result<String, String> {
    let url = "http://".to_string() +
        IPFS_NODE_ADDRESS + ":" + &IPFS_NODE_GATEWAY_PORT.to_string() +
        "/ipfs/" + cid;
    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map(|v| v.error_for_status());
    if let Err(e) = res {
        return Err(e.to_string());
    }
    let res = res.unwrap();
    if let Err(e) = res {
        return Err(e.to_string());
    }
    let res = res.unwrap();

    let content = res.text().await;
    if let Err(e) = content {
        return Err(e.to_string());
    }
    let content = content.unwrap();
    trace!("text: {:?}", content);
    Ok(content)
}
