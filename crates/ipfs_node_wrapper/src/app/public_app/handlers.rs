#[allow(unused_imports)]
use tracing::{info, trace, error};
use axum::extract::{Path, Query, State};
use axum::body::Body;
use axum::response::IntoResponse;
use crate::app::public_app::PublicAppState;
use crate::app::vo;
use crate::utils::HttpHeaderPorterFromReqwest;
use crate::common::ApiResponseResult;

/// Get file from IPFS node's gateway.
#[axum_macros::debug_handler]
pub async fn get_file(
    State(state): State<PublicAppState>,
    Path(cid): Path<String>,
    Query(query): Query<vo::GetFileArgs>)
    -> ApiResponseResult {
    info!("Get File cid: {}", cid);
    let ipfs_res = state.app_state.ipfs_client
        .get_file_by_gateway(
            &cid,
            query.filename.as_deref(),
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

