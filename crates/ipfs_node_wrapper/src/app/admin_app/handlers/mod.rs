use axum::extract::State;
#[allow(unused_imports)]
use tracing::{info, trace, error, warn, debug};
use crate::app::admin_app::AdminAppState;
use crate::app::dto;
use crate::common::StandardApiResult;

mod pin;
mod traffic;

pub use pin::*;
pub use traffic::*;

/// Get IPFS node's information.
#[axum_macros::debug_handler]
pub async fn get_ipfs_node_info(State(state): State<AdminAppState>) -> StandardApiResult<dto::GetIpfsNodeInfoResponse> {
    let peer_id_res = state.app_state.ipfs_client.get_id_info().await?;
    trace!("peer_id_res: {:?}", peer_id_res);

    let res = dto::GetIpfsNodeInfoResponse {
        id: peer_id_res.id,
    };
    Ok(res.into())
}

