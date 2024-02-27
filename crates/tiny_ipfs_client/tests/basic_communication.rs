use tiny_ipfs_client::ReqwestIpfsClient;

#[tokio::test]
#[ignore]
async fn test_pin() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let span = tracing::info_span!("test_ipfs_client");
    let _guard = span.enter();

    let ipfs_client = ReqwestIpfsClient::default();

    let ans = ipfs_client
        .list_recursive_pins_pinned(true).await.unwrap();
    println!("list_recursive_pins_pinned: {:#?}", ans);

    let cid = "QmRTV3h1jLcACW4FRfdisokkQAk4E4qDhUzGpgdrd4JAFy";

    ipfs_client
        .add_pin_recursive(cid, None).await.unwrap();
    println!("add_pin_recursive: {}", cid);

    let ans = ipfs_client
        .list_recursive_pins_pinned(true).await.unwrap();
    println!("list_recursive_pins_pinned: {:#?}", ans);
    assert!(ans.keys.contains_key(cid));

    ipfs_client
        .remove_pin_recursive(cid).await.unwrap();
    println!("remove_pin_recursive: {}", cid);

    let ans = ipfs_client
        .list_recursive_pins_pinned(true).await.unwrap();
    println!("list_recursive_pins_pinned: {:#?}", ans);
    assert!(!ans.keys.contains_key(cid));
}

#[tokio::test]
#[ignore]
async fn test_get_one_pin() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let span = tracing::info_span!("test_ipfs_client");
    let _guard = span.enter();

    let ipfs_client = ReqwestIpfsClient::default();

    let cid = "QmWeoysRLxatACwJQNmZLbBefTrFfdJoYcCQb3FoAZ2kt4";
    let res = ipfs_client.get_one_pin(cid, true).await;
    println!("get_one_pin {} res: {:?}", cid, res);
}
