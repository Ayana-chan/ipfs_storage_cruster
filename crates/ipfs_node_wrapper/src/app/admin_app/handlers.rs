use axum::extract::State;
#[allow(unused_imports)]
use tracing::{info, trace, error};
use crate::app::admin_app::AdminAppState;
use crate::common::StandardApiResult;
use crate::error;

/// Pin file to IPFS node.
#[axum_macros::debug_handler]
#[tracing::instrument(skip_all)]
pub async fn pin_file(State(state): State<AdminAppState>) -> StandardApiResult<()> {
    Err(error::IPFS_UNKNOWN_ERROR.clone().into())
}