use axum::extract::State;
use ipfs_node_wrapper_app_structs::admin::dtos;
use crate::app::admin_app::AdminAppState;
use crate::common::StandardApiResult;

/// Get a list of the number of times files has been downloaded.
#[axum_macros::debug_handler]
pub async fn get_download_time_list(State(state): State<AdminAppState>) -> StandardApiResult<dtos::GetDownloadTimeListResponse> {
    let list = state.app_state.file_traffic_counter.clone();
    let res = dtos::GetDownloadTimeListResponse {
        list,
    };
    Ok(res.into())
}
