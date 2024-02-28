use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetFileArgs {
    pub filename: Option<String>,
}

