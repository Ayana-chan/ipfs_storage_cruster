
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}
