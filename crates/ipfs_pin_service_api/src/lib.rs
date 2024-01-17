use async_trait::async_trait;

pub mod error;

#[async_trait]
pub trait ApiNoContext {

    /// Add pin object
    async fn add_pin(
        &self,
        pin: models::Pin,
    ) -> Result<AddPinResponse, ApiError>;

    /// Remove pin object
    async fn delete_pin_by_request_id(
        &self,
        requestid: String,
    ) -> Result<DeletePinByRequestIdResponse, ApiError>;

    /// Get pin object
    async fn get_pin_by_request_id(
        &self,
        requestid: String,
    ) -> Result<GetPinByRequestIdResponse, ApiError>;

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
    ) -> Result<GetPinsResponse, ApiError>;

    /// Replace pin object
    async fn replace_pin_by_request_id(
        &self,
        requestid: String,
        pin: models::Pin,
    ) -> Result<ReplacePinByRequestIdResponse, ApiError>;
}
