use ipfs_node_wrapper_client::admin::IpfsNodeWrapperAdminClient;

#[tokio::test]
#[ignore]
async fn test_basic() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .init();
    let span = tracing::info_span!("test_ipfs_client");
    let _guard = span.enter();

    let client = IpfsNodeWrapperAdminClient::new(
        "127.0.0.1:4000".to_string());
    let res = client.get_ipfs_node_info().await;
    println!("get_ipfs_node_info: {:?}", res);
}
