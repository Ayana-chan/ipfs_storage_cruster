use std::collections::HashMap;
use serde::Deserialize;
use crate::models;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponse {
    pub message: String,
    pub code: i32,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub public_key: String,
    pub addresses: Vec<String>,
    pub agent_version: String,
    pub protocols: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPinsResponse {
    pub keys: HashMap<String, PinsInfoInList>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinsInfoInList {
    pub name: Option<String>,
    pub r#type: models::PinType,
}



