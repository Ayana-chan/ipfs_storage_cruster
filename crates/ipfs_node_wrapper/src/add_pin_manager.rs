use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AddPinState {
    Queued,
    Pinning,
    Pinned,
    Failed,
}

#[derive(Debug)]
pub struct AddPinManager {
    /// cid -> state
    state_map: scc::HashMap<String, AddPinState>,
}
