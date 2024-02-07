use tracing::trace;

#[derive(Default, Clone, Debug)]
pub struct IpfsNodeMetadata {
    pub gateway_address: String,
    pub rpc_address: String,
}

#[tracing::instrument]
pub async fn ipfs_get_file(cid: &str, ipfs_node_metadata: &parking_lot::RwLock<IpfsNodeMetadata>) -> Result<String, String> {
    let url = "http://".to_string() +
        &ipfs_node_metadata.read().gateway_address +
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
