//! Provide some helper functions about IPFS.
#[allow(unused_imports)]
use tracing::{error, debug, warn, info, trace, info_span};
use crate::app::admin_app::AdminAppState;

static CACHE_PINS_INTERVAL_TIME_MS: u64 = 1000;

// TODO tracing span
/// Initial works when start contact IPFS node.
pub fn init_ipfs_contact(state: &AdminAppState) {
    let state = state.clone();
    tokio::spawn(async move {
        cache_recursive_pins(&state).await;
    });
}

/// Regularly try until get pins list successfully.
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
            },
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
