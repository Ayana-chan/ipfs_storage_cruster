use serde::{Deserialize, Serialize};
use crate::admin::models;

#[derive(Debug, Clone, Deserialize)]
pub struct AddPinArgs {
    /// cid of the target IPFS object
    pub cid: String,
    /// pin's name
    pub name: Option<String>,
    /// Default true. If be false, it wouldn't response until pin finishes.
    pub r#async: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckPinResponse {
    /// cid of the target IPFS object
    pub status: models::PinStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetIpfsNodeInfoResponse {
    /// peer ID
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListSucceededPinsResponse {
    /// Result pin cids
    pub cids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemovePinArgs {
    pub cid: String,
}

// TODO 换掉
#[derive(Debug, Clone, Serialize)]
pub struct GetDownloadTimeListResponse {
    pub list: scc::HashMap<String, usize>,
}

