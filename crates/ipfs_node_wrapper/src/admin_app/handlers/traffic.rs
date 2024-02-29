use tracing::{info, debug, trace};
use std::collections::HashMap;
use axum::extract::State;
use ipfs_node_wrapper_structs::admin::dtos;
use crate::admin_app::AdminAppState;
use ipfs_node_wrapper_structs::StandardApiResult;

// TODO 也许一般需要的是过去的一个时间段内的下载量（或者，为了防止节点太新，使用平均下载量增加速度）
/// Get a list of the number of times files has been downloaded.
#[axum_macros::debug_handler]
pub async fn get_download_time_list(State(state): State<AdminAppState>) -> StandardApiResult<dtos::GetDownloadTimeListResponse> {
    info!("Get download time list");
    let target_hashmap_ref = &state.app_state.file_traffic_counter;
    let mut list = HashMap::with_capacity(target_hashmap_ref.capacity());
    target_hashmap_ref.scan_async(|k, v| {
        let _ = list.insert(k.clone(), v.clone());
    }).await;
    debug!("Finish get download time list. {} items in total.", list.len());
    trace!("Download time list: {:?}", list);

    let res = dtos::GetDownloadTimeListResponse {
        list,
    };
    Ok(res.into())
}
