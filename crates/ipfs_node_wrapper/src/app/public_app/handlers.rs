#[allow(unused_imports)]
use tracing::{info, trace};
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use bytes::Bytes;
use crate::app::public_app::PublicAppState;
use crate::models;
use crate::ipfs_client;
use crate::utils::HttpHeaderPorterFromReqwest;

//TODO 文件名
#[axum_macros::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn get_file(
    State(state): State<PublicAppState>,
    Query(query): Query<models::GetFileArgs>)
    -> Result<(HeaderMap, Bytes), String> {
    info!("Get File cid: {}", query.cid);
    let ipfs_res = ipfs_client::ipfs_get_file(
        &query.cid,
        &state.app_state.ipfs_node_metadata,
    ).await;
    if let Err(e) = ipfs_res {
        return Err(e); //TODO 定制error
    }
    let ipfs_res = ipfs_res.unwrap();

    // construct header
    let ipfs_res_header = ipfs_res.headers();
    // trace!("Header: {:#?}", ipfs_res_header);
    let header = HttpHeaderPorterFromReqwest::new(ipfs_res_header)
        .transfer_when_exist_with_static_key("content-type")
        .finish();

    // read file
    let content = ipfs_res.bytes().await;
    if let Err(e) = content {
        return Err(e.to_string());
    }
    let content = content.unwrap();
    // trace!("File content: {:?}", content);

    Ok((header, content))
}


