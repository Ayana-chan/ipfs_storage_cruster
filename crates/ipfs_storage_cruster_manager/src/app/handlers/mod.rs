use axum::extract::State;
use axum::http;
use axum::response::IntoResponse;
#[allow(unused_imports)]
use tracing::{info, debug, trace, warn, error};
use crate::app::AppState;
use crate::utils::move_entry_between_header_map;

// TODO 返回值
/// Add file.
/// Use [reverse-proxy](https://github.com/tokio-rs/axum/tree/main/examples/reverse-proxy)
/// to send stream data.
///
/// Seems no size limitation.
#[axum_macros::debug_handler]
pub async fn add_file(State(state): State<AppState>, mut req: axum::extract::Request) -> Result<axum::response::Response, http::StatusCode> {
    // log
    let file_size = req.headers().get(http::header::CONTENT_LENGTH);
    if file_size.is_none() {
        warn!("Add file without content length in headers");
    } else if let Some(file_size) = file_size {
        info!("Add file. Content size: {:?}", file_size);
    }

    // handle url
    let url = format!("http://{}/api/v0/add", state.ipfs_client.rpc_address);
    *req.uri_mut() = http::uri::Uri::try_from(url).unwrap();

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

    let res = state.raw_hyper_client
        .request(req)
        .await.map_err(|_| http::StatusCode::BAD_REQUEST)?;
    let res = res.into_response();
    debug!("res into_response: {:?}", res);

    Ok(res
        .into_response())
}

