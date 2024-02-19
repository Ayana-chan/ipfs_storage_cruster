
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PinFileArgs {
    /// cid of the target IPFS object
    pub cid: String,
    /// pin's name
    pub name: Option<String>,
    /// Default true. If be false, it wouldn't response until pin finishes.
    pub r#async: Option<bool>,
}


