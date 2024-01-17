// #![warn(missing_docs)]

use async_trait::async_trait;
use axum::{http, Router};
use axum::routing::{get, post, delete};

pub mod errors;
pub mod models;

#[async_trait]
pub trait IpfsPinServiceApi {
    /// List pin objects
    async fn get_pins(
        &self,
        get_pins_args: models::GetPinsArgs
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

pub fn generate_router<T>() -> Router
where T: IpfsPinServiceApi{
    let ipfs_pin_service_app = Router::new()
        .route("/", get(T::get_pins))
        .route("/", post(T::add_pin))
        .route("/:requestid", get(T::get_pin_by_request_id))
        .route("/:requestid", post(T::get_pins))
        .route("/:requestid", delete(T::get_pins));
}

/// convert u16 to http::StatusCode
fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}
