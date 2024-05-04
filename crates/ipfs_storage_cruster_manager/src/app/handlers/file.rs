use axum::extract::{Query, State};
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
    let upload_res = services::file::add_file_to_ipfs(&state, req).await?;

    let new_pin_id = Uuid::new_v4().to_string();
    let new_pin = pin::ActiveModel {
        id: Set(new_pin_id.clone()),
        status: Set(sea_orm_active_enums::Status::Queued),
        cid: Set(upload_res.hash.clone()),
    };
    let add_pin_res = new_pin.insert(&state.db_conn).await
        .map_err(services::db::check_duplicate_key_error);
    // TODO insert users_pins
    if let Err(e) = add_pin_res {
        // throw db error
        let _ = e.map_err(services::db::handle_db_error)?;
        // no need to do anything when dup key
        info!("cid {} has been stored, skip it", upload_res.hash.clone());
    } else {
        // TODO here async
        // make decision and store
        let stored_node_list = services::file::store_file_to_cluster(&state, upload_res.hash.clone()).await?;

        // TODO modify pin's status.
        // store decision to database
        let node_models: Vec<_> = stored_node_list.into_iter()
            .map(|v| pins_stored_nodes::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                pin_id: Set(new_pin_id.clone()),
                node_id: Set(v.id),
            }).collect();
        PinsStoredNodes::insert_many(node_models)
            .exec(&state.db_conn).await
            .map_err(services::db::handle_db_error)?;
    }

    info!("Finish storing cid {}", upload_res.hash.clone());
    let res = dtos::UploadFileResponse {
        request_id: new_pin_id,
        file_metadata: upload_res,
    };
    Ok(res.into())
}

/// Get the advice that which Wrapper to download the file.
///
/// Return the url of target Wrapper (no scheme like "http://").
// #[axum_macros::debug_handler]
pub async fn download_file_advice(State(state): State<AppState>, Query(args): Query<dtos::DownloadFileAdviceArgs>) -> StandardApiResult<dtos::DownloadFileAdviceResponse> {
    let target_wrapper = state.file_download_decision_maker
        .decide_download_node(&args.cid, &state.db_conn, &state.reqwest_client).await?;
    let target_wrapper_pub_addr = target_wrapper.wrapper_public_address;
    let target_url = target_wrapper_pub_addr + "/api/" + &args.cid;
    info!("cid {} would be downloaded at target url: {}", args.cid, target_url);
    let res = dtos::DownloadFileAdviceResponse {
        url: target_url,
    };
    Ok(res.into())
}
