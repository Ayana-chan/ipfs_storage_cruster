#[allow(unused_imports)]
use tracing::{info, trace, error};
use axum::extract::{Path, Query, State};
use axum::body::Body;
use axum::response::IntoResponse;
use crate::app::public_app::PublicAppState;
use crate::models;
use crate::ipfs_client;
use crate::utils::HttpHeaderPorterFromReqwest;
use crate::common::ApiResponseResult;

/// Get file from IPFS node's gateway.
#[axum_macros::debug_handler]
pub async fn get_file(
    State(state): State<PublicAppState>,
    Path(cid): Path<String>,
    Query(query): Query<models::GetFileArgs>)
    -> ApiResponseResult {
    info!("Get File cid: {}", cid);
    let ipfs_res = ipfs_client::ipfs_get_file(
        &cid,
        query.filename.as_deref(),
        &state.app_state.ipfs_node_metadata,
    ).await?;

    // construct header
    let ipfs_res_header = ipfs_res.headers();
    // trace!("Header: {:#?}", ipfs_res_header);
    let header = HttpHeaderPorterFromReqwest::new(ipfs_res_header)
        .transfer_when_exist_with_static_key("content-type")
        .transfer_when_exist_with_static_key("content-disposition")
        .finish();

    // read file
    let body = Body::from_stream(ipfs_res.bytes_stream());

    Ok((header, body).into_response())
}

