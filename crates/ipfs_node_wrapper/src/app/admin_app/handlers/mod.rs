use axum::extract::State;
#[allow(unused_imports)]
use tracing::{info, trace, error, warn, debug};
use ipfs_node_wrapper_app_structs::admin::dtos;
use crate::app::admin_app::AdminAppState;
use crate::common::StandardApiResult;
use crate::error_convert;

mod pin;
mod traffic;

pub use pin::*;
pub use traffic::*;

/// Get IPFS node's information.
#[axum_macros::debug_handler]
pub async fn get_ipfs_node_info(State(state): State<AdminAppState>) -> StandardApiResult<dtos::GetIpfsNodeInfoResponse> {
    let peer_id_res = state.app_state.ipfs_client.get_id_info().await
        .map_err(error_convert::from_ipfs_client_error)?;
    trace!("peer_id_res: {:?}", peer_id_res);

    let res = dtos::GetIpfsNodeInfoResponse {
        id: peer_id_res.id,
    };
    Ok(res.into())
}

