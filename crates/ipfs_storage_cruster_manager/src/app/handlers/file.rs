use axum::extract::State;
#[allow(unused_imports)]
use tracing::{info, debug, trace, warn, error};
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::dtos;
use crate::app::services;

/// Add file.
/// Use [reverse-proxy](https://github.com/tokio-rs/axum/tree/main/examples/reverse-proxy)
/// to send stream data.
///
/// Seems no request size limitation.
#[axum_macros::debug_handler]
pub async fn upload_file(State(state): State<AppState>, req: axum::extract::Request) -> StandardApiResult<dtos::UploadFileResponse> {
    let upload_res = services::ipfs::add_file_to_ipfs(&state, req).await?;
    // TODO pin
    let res = dtos::UploadFileResponse {
        request_id: "todo request_id".to_string(),
        file_metadata: upload_res,
    };
    Ok(res.into())
}

