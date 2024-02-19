use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
#[allow(unused_imports)]
use tracing::{info, trace, error};
use crate::app::admin_app::AdminAppState;
use crate::common::{StandardApiResult, StandardApiResultStatus};
use crate::models;

/// Pin file to IPFS node.
/// Return after pin completed.
#[axum_macros::debug_handler]
pub async fn add_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<models::PinFileArgs>)
    -> StandardApiResult<()> {
    if args.async == Some(false) {
        add_pin_sync(state, args).await
    } else {
        add_pin_async(state, args).await
    }
}

async fn add_pin_sync(state: AdminAppState, args: models::PinFileArgs) -> StandardApiResult<()> {
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
pub async fn add_pin_async(state: AdminAppState, args: models::PinFileArgs) -> StandardApiResultStatus<()> {
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

/// Check whether pin succeed.
/// Just query local recorder, so maybe return `Failed` when not found.
#[axum_macros::debug_handler]
pub async fn check_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<models::PinFileArgs>)
    -> StandardApiResultStatus<()> {
    info!("Check Pin cid: {}", args.cid);
    // TODO Path para
    Ok(().into())
}
