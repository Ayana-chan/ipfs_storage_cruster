use async_tasks_state_map::{TaskState, RevokeFailReason};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
#[allow(unused_imports)]
use tracing::{info, trace, error, warn, debug};
use crate::app::admin_app::AdminAppState;
use crate::app::vo;
use crate::common::{StandardApiResult, StandardApiResultStatus};
use crate::app::models;
use crate::app::admin_app::ipfs_helper;

/// Check status of adding pin.
/// Just query local recorder, so maybe return `Failed` when not found.
#[axum_macros::debug_handler]
pub async fn check_pin(
    State(state): State<AdminAppState>,
    Path(cid): Path<String>)
    -> StandardApiResult<vo::CheckPinResponse> {
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

    let res = vo::CheckPinResponse {
        status
    };
    Ok(res.into())
}

/// List all recursive pins that is pinned in IPFS node.
#[axum_macros::debug_handler]
pub async fn list_succeeded_pins(State(state): State<AdminAppState>) -> StandardApiResult<vo::ListSucceededPinsResponse> {
    info!("List Pins.");
    let list_res = state.app_state.ipfs_client
        .list_recursive_pins_pinned(false).await?;
    let cids = list_res.keys.into_keys().collect();
    let res = vo::ListSucceededPinsResponse {
        cids,
    };
    Ok(res.into())
}

/// Add a pin to IPFS node.
#[axum_macros::debug_handler]
pub async fn add_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<vo::AddPinArgs>)
    -> Response {
    if args.r#async == Some(false) {
        add_pin_sync(state, args).await.into_response()
    } else {
        add_pin_async(state, args).await.into_response()
    }
}

/// Pin file to IPFS node.
#[axum_macros::debug_handler]
pub async fn rm_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<vo::RemovePinArgs>)
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
    if let Err(RevokeFailReason::RevokeTaskError(e)) = revoke_res {
        debug!("Failed to remove pin for IPFS error. cid: {}, ", args.cid);
        return Err(e.into());
    }

    // Return ok even the removing pin didn't actually occurred/ finished. TODO 急需单飞
    Ok(().into())
}

// --------------------------------------------------------------------------------

/// Add a pin to IPFS node.
/// Return until pin finishes.
async fn add_pin_sync(state: AdminAppState, args: vo::AddPinArgs) -> StandardApiResult<()> {
    info!("Add Pin. cid: {}", args.cid);
    state.app_state.ipfs_client
        .add_pin_recursive(
            &args.cid,
            args.name.as_deref(),
        ).await?;

    // cache
    let _ = state.add_pin_task_recorder
        .modify_to_success_before_work(args.cid).await;

    // trace!("add pin res: {}", _ipfs_res.text().await.unwrap_or_default());
    Ok(().into())
}

/// Add a pin to IPFS node.
/// Return immediately.
async fn add_pin_async(state: AdminAppState, args: vo::AddPinArgs) -> StandardApiResultStatus<()> {
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
