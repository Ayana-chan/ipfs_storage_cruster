use async_tasks_state_map::TaskState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
#[allow(unused_imports)]
use tracing::{info, trace, error, warn, debug};
use ipfs_node_wrapper_structs::admin::{dtos, models};
use ipfs_node_wrapper_structs::{StandardApiResult, StandardApiResultStatus};
use crate::admin_app::AdminAppState;
use crate::admin_app::ipfs_helper;
use crate::error_convert;

/// Check status of adding pin.
/// Just query local recorder, so maybe return `Failed` when not found.
#[axum_macros::debug_handler]
pub async fn check_pin(
    State(state): State<AdminAppState>,
    Path(cid): Path<String>)
    -> StandardApiResult<dtos::CheckPinResponse> {
    info!("Check Pin. cid: {}", cid);
    let task_state = state.add_pin_task_recorder.query_task_state(&cid).await;
    let status = match task_state {
        TaskState::Success | TaskState::Revoking => models::PinStatus::Pinned,
        TaskState::Working => models::PinStatus::Pinning,
        TaskState::Failed => ipfs_helper::check_pinned_and_cache(cid, &state).await?
            .unwrap_or(models::PinStatus::Failed),
        TaskState::NotFound => ipfs_helper::check_pinned_and_cache(cid, &state).await?
            .unwrap_or(models::PinStatus::NotFound),
    };

    let res = dtos::CheckPinResponse {
        status
    };
    Ok(res.into())
}

/// List all recursive pins that is pinned in IPFS node.
#[axum_macros::debug_handler]
pub async fn list_succeeded_pins(State(state): State<AdminAppState>) -> StandardApiResult<dtos::ListSucceededPinsResponse> {
    info!("List Pins.");
    let list_res = state.app_state.ipfs_client
        .list_recursive_pins_pinned(false).await
        .map_err(error_convert::from_ipfs_client_error)?;
    let cids = list_res.keys.into_keys().collect();
    let res = dtos::ListSucceededPinsResponse {
        cids,
    };
    Ok(res.into())
}

/// Add a pin to IPFS node.
#[axum_macros::debug_handler]
pub async fn add_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<dtos::AddPinArgs>)
    -> Response {
    if args.background == Some(false) {
        add_pin_sync(state, args).await.into_response()
    } else {
        add_pin_background(state, args).await.into_response()
    }
}

/// Remove a pin.
#[axum_macros::debug_handler]
pub async fn rm_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<dtos::RemovePinArgs>)
    -> StandardApiResult<()> {
    info!("Remove Pin. cid: {}", args.cid);
    let app_state = state.app_state.clone();
    let cid_backup = args.cid.clone();
    let task = async move {
        app_state.ipfs_client
            .remove_pin_recursive(&cid_backup)
            .await
    };

    let revoke_res = state.add_pin_task_recorder
        .revoke_task_block(&args.cid, task).await;
    // IPFS err
    if let Ok(Err(e)) = revoke_res {
        debug!("Failed to remove pin for IPFS error. cid: {}, ", args.cid);
        return Err(error_convert::from_ipfs_client_error(e));
    }

    // Return ok even the removing pin didn't actually occurred / finished. TODO 急需单飞
    Ok(().into())
}

// --------------------------------------------------------------------------------

/// Add a pin to IPFS node.
/// Wouldn't return until pin finishes.
/// Wouldn't be recorded into memory.
async fn add_pin_sync(state: AdminAppState, args: dtos::AddPinArgs) -> StandardApiResult<()> {
    info!("Add Pin. cid: {}", args.cid);
    state.app_state.ipfs_client
        .add_pin_recursive(
            &args.cid,
            args.name.as_deref(),
        ).await
        .map_err(error_convert::from_ipfs_client_error)?;

    // cache might break the consistency
    // let _ = state.add_pin_task_recorder
    //     .modify_to_success_before_work(args.cid).await;

    // trace!("add pin res: {}", _ipfs_res.text().await.unwrap_or_default());
    Ok(().into())
}

/// Add a pin to IPFS node.
/// Return immediately.
async fn add_pin_background(state: AdminAppState, args: dtos::AddPinArgs) -> StandardApiResultStatus<()> {
    info!("Add Pin Async. cid: {}", args.cid);
    let app_state = state.app_state.clone();
    let cid_backup = args.cid.clone();
    let task = async move {
        app_state.ipfs_client
            .add_pin_recursive(
                &cid_backup,
                args.name.as_deref())
            .await
    };

    let _ = state.add_pin_task_recorder
        .launch(args.cid, task).await;

    Ok((StatusCode::ACCEPTED, ().into()))
}
