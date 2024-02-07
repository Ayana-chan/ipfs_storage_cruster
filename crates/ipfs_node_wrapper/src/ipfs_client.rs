use reqwest::Response;

#[derive(Default, Clone, Debug)]
pub struct IpfsNodeMetadata {
    pub gateway_address: String,
    pub rpc_address: String,
}

// TODO 建立struct来访问ipfs，然后把结构体存在state中

#[tracing::instrument(skip_all)]
pub async fn ipfs_get_file(cid: &str, ipfs_node_metadata: &parking_lot::RwLock<IpfsNodeMetadata>) -> Result<Response, String> {
    let url = "http://".to_string() +
        &ipfs_node_metadata.read().gateway_address +
        "/ipfs/" + cid;
    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await.map_err(|e| e.to_string())?
        .error_for_status().map_err(|e| e.to_string())?;

    Ok(res)
}
