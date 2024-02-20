use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
#[allow(unused_imports)]
use tracing::{info, trace, error};
use async_tasks_recorder::TaskState;
use crate::app::admin_app::AdminAppState;
use crate::app::vo;
use crate::common::{StandardApiResult, StandardApiResultStatus};
use crate::models;

/// Get IPFS node's information.
#[axum_macros::debug_handler]
pub async fn get_ipfs_node_info(State(state): State<AdminAppState>) -> StandardApiResult<vo::GetIpfsNodeInfoResponse> {
    let peer_id_res = state.app_state.ipfs_client.get_id().await?;
    trace!("peer_id_res: {:?}", peer_id_res);

    let res = vo::GetIpfsNodeInfoResponse {
        id: peer_id_res.id,
    };
    Ok(res.into())
}

/// Pin file to IPFS node.
/// Return after pin completed.
#[axum_macros::debug_handler]
pub async fn add_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<vo::PinFileArgs>)
    -> Response {
    if args.r#async == Some(false) {
        add_pin_sync(state, args).await.into_response()
    } else {
        add_pin_async(state, args).await.into_response()
    }
}

/// Check status of adding pin.
/// Just query local recorder, so maybe return `Failed` when not found.
#[axum_macros::debug_handler]
pub async fn check_pin(
    State(state): State<AdminAppState>,
    Path(cid): Path<String>)
    -> StandardApiResult<vo::CheckPinResponse> {
    info!("Check Pin cid: {}", cid);
    let task_state = state.add_pin_recorder.query_task_state(&cid.into()).await;
    let status = match task_state {
        TaskState::Success => models::PinStatus::Pinned,
        TaskState::Working => models::PinStatus::Pinning,
        TaskState::Failed => models::PinStatus::Failed,
        TaskState::NotFound => models::PinStatus::NotFound,
    };
    let res = vo::CheckPinResponse {
        status,
    };
    Ok(res.into())
}

// TODO verify pin，可以供NotFound时深入检查

// --------------------------------------------------------------------------------

/// Pin file to IPFS node.
/// Return until pin finishes.
async fn add_pin_sync(state: AdminAppState, args: vo::PinFileArgs) -> StandardApiResult<()> {
    info!("Add Pin cid: {}", args.cid);
    let _ipfs_res = state.app_state.ipfs_client
        .add_pin_recursive(
            &args.cid,
            args.name.as_deref(),
        ).await?;

    // trace!("add pin res: {}", _ipfs_res.text().await.unwrap_or_default());
    Ok(().into())
}

/// Pin file to IPFS node.
/// Return immediately.
async fn add_pin_async(state: AdminAppState, args: vo::PinFileArgs) -> StandardApiResultStatus<()> {
    info!("Add Pin Async cid: {}", args.cid);
    let app_state = state.app_state.clone();
    let cid_backup = args.cid.clone();
    let task = async move {
        let res = app_state.ipfs_client.add_pin_recursive(&cid_backup, args.name.as_deref()).await;
        // ignore specific error types
        match res {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                Err(())
            }
        }
    };

    state.add_pin_recorder.launch(
        args.cid.into(),
        task,
    ).await;

    Ok((StatusCode::ACCEPTED, ().into()))
}
