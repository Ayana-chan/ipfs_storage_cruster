use serde::{Serialize, Deserialize};
use crate::models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinFileArgs {
    /// cid of the target IPFS object
    pub cid: String,
    /// pin's name
    pub name: Option<String>,
    /// Default true. If be false, it wouldn't response until pin finishes.
    pub r#async: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPinResponse {
    /// cid of the target IPFS object
    pub status: models::PinStatus,
}

