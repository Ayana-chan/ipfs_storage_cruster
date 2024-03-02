//! Provide some helper functions about IPFS.
#[allow(unused_imports)]
use tracing::{error, debug, warn, info, trace};
use ipfs_node_wrapper_structs::admin::models;
use ipfs_node_wrapper_structs::ApiResult;
use crate::app::admin_app::AdminAppState;
use crate::error_convert;

static CACHE_PINS_INTERVAL_TIME_MS: u64 = 500;

/// Initial works when start contact IPFS node.
pub async fn init_ipfs_contact(state: &AdminAppState) {
    let state = state.clone();
    // Won't serve until first IPFS request succeed.
    cache_recursive_pins(&state).await;
}

/// Regularly try until get pins list successfully once.
#[tracing::instrument(skip_all)]
pub async fn cache_recursive_pins(state: &AdminAppState) {
    let pins;
    loop {
        let res = state.app_state.ipfs_client
            .list_recursive_pins_pinned(false).await;
        match res {
            Ok(res) => {
                pins = res.keys;
                break;
            }
            Err(_e) => {
                error!("Failed to cache recursive pins. Try again in {} ms ...", CACHE_PINS_INTERVAL_TIME_MS);
                tokio::time::sleep(tokio::time::Duration::from_millis(CACHE_PINS_INTERVAL_TIME_MS)).await;
            }
        };
    }

    info!("Cache {} pins that have been pinned in IPFS node.", pins.len());
    trace!("Cached pins: {:?}", pins);

    for pin_info in pins.into_keys() {
        let _ = state.add_pin_task_recorder
            .modify_to_success_before_work(pin_info).await;
    }
}

/// Check pin status in IPFS. If pinned, cache to `add_pin_task_recorder`, otherwise return `None`.
pub async fn check_pinned_and_cache(cid: String, state: &AdminAppState) -> ApiResult<Option<models::PinStatus>> {
    // query in IPFS
    let pin = state.app_state.ipfs_client.get_one_pin(&cid, false).await
        .map_err(error_convert::from_ipfs_client_error)?;
    if pin.is_some() {
        // TODO 无法进行cache，因为会导致不一致性：NotFound，然后直接查询，发现确实pin了，然而这时revoke迅速开始并完成，数据被删，但此处有可能又缓存为success。
        // 不会影响background pin的正确性，但依旧让其多进行了一次get_one_pin。
        // 如果每次success都会去进一步查询get_one_pin并确认的话，也许有效，也能称之为防止缓存击穿。看看redis的缓存一致性解决方案。
        // record to local (cache)
        // let _ = state.add_pin_task_recorder.modify_to_success_before_work(cid).await;
        Ok(Some(models::PinStatus::Pinned))
    } else {
        Ok(None)
    }
}
