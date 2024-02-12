
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PinFileArgs {
    pub cid: String,
    /// pin's name
    pub name: Option<String>,
}


