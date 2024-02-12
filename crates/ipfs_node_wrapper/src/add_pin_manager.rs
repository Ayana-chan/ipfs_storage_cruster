use std::collections::HashMap;
use std::future::Future;
use crate::common::ApiResult;

#[derive(Debug, Clone)]
pub enum AddPinWorkingState {
    Queued,
    Pinning,
}

#[derive(Default, Debug)]
pub struct AddPinManager {
    /// cid -> state
    working_tasks: scc::HashMap<String, AddPinWorkingState>,
    success_tasks: scc::HashSet<String>,
    failed_tasks: scc::HashSet<String>,
}

impl AddPinManager {
    pub fn new() -> Self {
        AddPinManager {
            working_tasks: scc::HashMap::new(),
            success_tasks: scc::HashSet::new(),
            failed_tasks: scc::HashSet::new(),
        }
    }

    pub async fn launch(&self, task: impl Future<Output=ApiResult<reqwest::Response>>) {
        // TODO 先插入success再移除working，查询的时候先查success，这样就可以在无原子性的时候也能正常工作
    }
}

