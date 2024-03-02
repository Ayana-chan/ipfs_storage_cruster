use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDownloadTimeListResponse {
    pub list: HashMap<String, usize>,
}

