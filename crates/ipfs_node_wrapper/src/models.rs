
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFileArgs {
    pub cid: String,
    pub filename: Option<String>,
}
