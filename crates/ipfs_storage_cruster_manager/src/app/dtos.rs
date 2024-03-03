use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsAddFileResponse {
    pub name: String,
    pub hash: String,
    pub size: String,
}
