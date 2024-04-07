use axum::extract::State;
use sea_orm::ActiveValue::Set;
#[allow(unused_imports)]
use tracing::{info, debug, trace, warn, error};
use crate::imports::dao_imports::*;
use crate::app::AppState;
use crate::app::common::StandardApiResult;
use crate::app::dtos;
use crate::app::services;

/// Upload file.
/// Use [reverse-proxy](https://github.com/tokio-rs/axum/tree/main/examples/reverse-proxy)
/// to send stream data.
///
/// Seems no request size limitation.
// #[axum_macros::debug_handler]
pub async fn upload_file(State(state): State<AppState>, req: axum::extract::Request) -> StandardApiResult<dtos::UploadFileResponse> {
    let upload_res = services::ipfs::add_file_to_ipfs(&state, req).await?;

    let new_pin_id = Uuid::new_v4().to_string();
    let new_pin = pin::ActiveModel {
        id: Set(new_pin_id),
        status: Set(sea_orm_active_enums::Status::Queued),
        cid: Set(upload_res.hash.clone()),
    };
    let add_pin_res = new_pin.insert(&state.db_conn).await
        .map_err(services::db::check_duplicate_key_error);
    if let Err(e) = add_pin_res {
        // no need to do anything when dup key
        let _ = e.map_err(services::db::handle_db_error)?;
    } else {
        // make decision and store
        services::ipfs::store_file_to_cluster(&state, upload_res.hash.clone()).await?;
    }

    // TODO pin
    let res = dtos::UploadFileResponse {
        request_id: "todo request_id".to_string(),
        file_metadata: upload_res,
    };
    Ok(res.into())
}

