use axum::extract::State;
use axum::Json;
#[allow(unused_imports)]
use tracing::{info, trace, error};
use crate::app::admin_app::AdminAppState;
use crate::common::StandardApiResult;
use crate::models;

/// Pin file to IPFS node.
#[axum_macros::debug_handler]
pub async fn add_pin(
    State(state): State<AdminAppState>,
    Json(args): Json<models::PinFileArgs>)
    -> StandardApiResult<()> {
    info!("Add Pin cid: {}", args.cid);
    let ipfs_res = state.app_state.ipfs_client
        .add_pin_recursive(
            &args.cid,
            args.name.as_deref()
        ).await?;

    Ok(().into())
}



