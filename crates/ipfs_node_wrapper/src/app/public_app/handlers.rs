#[allow(unused_imports)]
use tracing::{info, trace};
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use bytes::Bytes;
use crate::app::public_app::PublicAppState;
use crate::models;
use crate::ipfs_client;
use crate::utils::HttpHeaderPorterFromReqwest;
use crate::error;
use crate::common::ApiResponse;

#[axum_macros::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn get_file(
    State(state): State<PublicAppState>,
    Path(cid): Path<String>,
    Query(query): Query<models::GetFileArgs>)
    -> ApiResponse<(HeaderMap, Bytes)> {
    info!("Get File cid: {}", cid);
    let ipfs_res = ipfs_client::ipfs_get_file(
        &cid,
        query.filename.as_deref(),
        &state.app_state.ipfs_node_metadata,
    ).await?;

    // construct header
    let ipfs_res_header = ipfs_res.headers();
    trace!("Header: {:#?}", ipfs_res_header);
    let header = HttpHeaderPorterFromReqwest::new(ipfs_res_header)
        .transfer_when_exist_with_static_key("content-type")
        .transfer_when_exist_with_static_key("content-disposition")
        .finish();

    // read file
    let content = ipfs_res.bytes().await
        .map_err(|_e| error::IPFS_DOWNLOAD_ERROR.clone())?;
    // trace!("File content: {:?}", content);

    Ok((header, content))
}


