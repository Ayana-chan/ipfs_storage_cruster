use serde::{Serialize, Deserialize};
use ipfs_storage_cruster_manager_entity::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct IpfsAddFileResponse {
    pub name: String,
    pub hash: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    pub request_id: String,
    pub file_metadata: IpfsAddFileResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListIpfsNodesResponse {
    pub list: Vec<node::Model>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddIpfsNodeArgs {
    pub rpc_address: String,
    pub wrapper_public_address: String,
    pub wrapper_admin_address: String,
}

