use axum::extract::State;
use axum::http;
#[allow(unused_imports)]
use tracing::{info, debug, trace, warn, error};
use crate::app::AppState;
use crate::utils::move_entry_between_header_map;
use http_body_util::BodyExt;
use crate::app::common::{ApiResult, StandardApiResult};
use crate::app::{dtos, errors};

/// Add file.
/// Use [reverse-proxy](https://github.com/tokio-rs/axum/tree/main/examples/reverse-proxy)
/// to send stream data.
///
/// Seems no size limitation.
#[axum_macros::debug_handler]
pub async fn upload_file(State(state): State<AppState>, req: axum::extract::Request) -> StandardApiResult<dtos::UploadFileResponse> {
    let upload_res = add_file_to_ipfs(&state, req).await?;
    // TODO pin
    let res = dtos::UploadFileResponse {
        request_id: "todo request_id".to_string(),
        file_metadata: upload_res,
    };
    Ok(res.into())
}

// ----------------------------------------------------------------

/// Add a file to ipfs, return the message of the added file.
async fn add_file_to_ipfs(state: &AppState, mut req: axum::extract::Request) -> ApiResult<dtos::IpfsAddFileResponse> {
    // log
    let file_size = req.headers().get(http::header::CONTENT_LENGTH);
    if file_size.is_none() {
        warn!("Add file without content length in headers");
    } else if let Some(file_size) = file_size {
        info!("Add file. Content size: {:?}", file_size);
    }

    // handle url
    let url = format!("http://{}/api/v0/add", state.ipfs_client.rpc_address);
    *req.uri_mut() = http::uri::Uri::try_from(url).expect("Impossible fail to parse url");

    // handle headers
    let old_hm_ref = req.headers();
    let mut hm = http::header::HeaderMap::new();
    hm.reserve(5);
    move_entry_between_header_map(http::header::HOST, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONNECTION, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONTENT_LENGTH, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::ACCEPT, old_hm_ref, &mut hm);
    move_entry_between_header_map(http::header::CONTENT_TYPE, old_hm_ref, &mut hm);
    *req.headers_mut() = hm;
    trace!("add req: {:?}", req);

    // read body
    let res = state.raw_hyper_client
        .request(req)
        .await
        .map_err(|_e|
        errors::IPFS_REQUEST_ERROR.clone_to_error_with_log_with_content(_e)
    )?;
    if !res.status().is_success() {
        error!("Failed to add file to IPFS. Status code: {}", res.status());
        return Err(errors::IPFS_RESPOND_ERROR.clone_to_error());
    }
    let body = res.into_body().collect();
    let body = body.await
        .map_err(|_e| {
        error!("Failed to receive IPFS response when add file");
        errors::IPFS_FAIL.clone_to_error()
    })?;
    let body = body.to_bytes();
    let body: dtos::IpfsAddFileResponse = serde_json::from_slice(body.as_ref())
        .map_err(|_e| {
        error!("Unexpected IPFS response when add file");
        errors::IPFS_FAIL.clone_to_error()
    })?;
    info!("Add file succeed. {:?}", body);

    Ok(body)
}
