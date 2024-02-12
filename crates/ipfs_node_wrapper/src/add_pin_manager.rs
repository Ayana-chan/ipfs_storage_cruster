use std::collections::HashMap;
use std::future::Future;
use crate::common::ApiResult;

#[derive(Debug, Clone)]
pub enum AddPinState {
    Queued,
    Pinning,
    Pinned,
    Failed,
}

#[derive(Default, Debug)]
pub struct AddPinManager {
    /// cid -> state
    state_map: scc::HashMap<String, AddPinState>,
}

impl AddPinManager {
    pub fn new() -> Self {
        AddPinManager {
            state_map: scc::HashMap::new(),
        }
    }

    pub async fn launch(&self, task: impl Future<Output=ApiResult<reqwest::Response>>) {

    }
}

