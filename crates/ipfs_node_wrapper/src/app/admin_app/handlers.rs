use axum::extract::State;
use axum::Json;
#[allow(unused_imports)]
use tracing::{info, trace, error};
use crate::app::admin_app::AdminAppState;
use crate::common::StandardApiResult;
use crate::error;
use crate::models;

/// Pin file to IPFS node.
#[axum_macros::debug_handler]
pub async fn pin_file(
    State(state): State<AdminAppState>,
    Json(args): Json<models::PinFileArgs>)
    -> StandardApiResult<()> {

    Err(error::IPFS_UNKNOWN_ERROR.clone_to_error())
}



