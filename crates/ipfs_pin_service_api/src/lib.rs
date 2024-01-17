use async_trait::async_trait;
use axum::http;
use crate::models::PinStatus;

mod errors;
mod models;

#[async_trait]
pub trait IpfsPinServiceApi {
    /// List pin objects
    async fn get_pins(
        &self,
        cid: Option<&Vec<String>>,
        name: Option<String>,
        r#match: Option<models::TextMatchingStrategy>,
        status: Option<&Vec<models::Status>>,
        before: Option<chrono::DateTime::<chrono::Utc>>,
        after: Option<chrono::DateTime::<chrono::Utc>>,
        limit: Option<i32>,
        meta: Option<std::collections::HashMap<String, String>>,
    ) -> Result<models::PinResults, errors::ResponseError>;

    /// Add pin object
    async fn add_pin(
        &self,
        pin: models::Pin,
    ) -> Result<models::PinStatus, errors::ResponseError>;

    /// Get pin object
    async fn get_pin_by_request_id(
        &self,
        requestid: String,
    ) -> Result<models::PinStatus, errors::ResponseError>;

    /// Replace pin object
    async fn replace_pin_by_request_id(
        &self,
        requestid: String,
        pin: models::Pin,
    ) -> Result<models::PinStatus, errors::ResponseError>;

    /// Remove pin object
    async fn delete_pin_by_request_id(
        &self,
        requestid: String,
    ) -> Result<(), errors::ResponseError>;
}

fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}
