use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PinType {
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "indirect")]
    Indirect,
    #[serde(rename = "recursive")]
    Recursive,
}
