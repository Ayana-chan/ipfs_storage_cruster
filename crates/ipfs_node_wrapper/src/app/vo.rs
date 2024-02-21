use serde::{Serialize, Deserialize};
use crate::app::models;

#[derive(Debug, Clone, Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinFileArgs {
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


