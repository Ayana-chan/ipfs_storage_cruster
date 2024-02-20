use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PinStatus {
    #[serde(rename = "pinning")]
    Pinning,
    #[serde(rename = "pinned")]
    Pinned,
    #[serde(rename = "failed")]
    Failed,
}
