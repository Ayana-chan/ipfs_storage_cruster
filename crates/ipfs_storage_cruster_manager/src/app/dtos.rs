use serde::{Serialize, Deserialize};

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
