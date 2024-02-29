use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::admin::models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPinArgs {
    /// cid of the target IPFS object
    pub cid: String,
    /// pin's name
    pub name: Option<String>,
    /// Default true. If be false, it wouldn't response until pin finishes.
    pub background: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPinResponse {
    /// cid of the target IPFS object
    pub status: models::PinStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIpfsNodeInfoResponse {
    /// peer ID
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSucceededPinsResponse {
    /// Result pin cids
    pub cids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePinArgs {
    pub cid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDownloadTimeListResponse {
    pub list: HashMap<String, usize>,
}

