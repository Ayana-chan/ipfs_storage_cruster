//! Args and correct Responses of API. \
//! - Each response is specified which a fixed status code. \
//! - Each response impls `IntoResponse` trait.

use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::{common, models};

/// Args to get pin list. For pagination and filtering.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct GetPinsArgs {
    #[serde(deserialize_with = "option_form_vec_deserialize", default)]
    pub cid: Option<Vec<String>>,
    pub name: Option<String>,
    pub r#match: Option<models::TextMatchingStrategy>,
    #[serde(deserialize_with = "option_form_vec_deserialize", default)]
    pub status: Option<Vec<models::Status>>,
    pub before: Option<chrono::DateTime::<chrono::Utc>>,
    pub after: Option<chrono::DateTime::<chrono::Utc>>,
    pub limit: Option<i32>,
    pub meta: Option<std::collections::HashMap<String, String>>,
}

pub struct GetPinsResponse {
    pin_results: models::PinResults,
}

impl GetPinsResponse {
    pub fn new(pin_results: models::PinResults) -> Self {
        Self {
            pin_results
        }
    }
}

impl IntoResponse for GetPinsResponse {
    fn into_response(self) -> Response {
        (common::convert_status_code(200), Json(self.pin_results)).into_response()
    }
}

impl From<models::PinResults> for GetPinsResponse {
    fn from(value: models::PinResults) -> Self {
        Self {
            pin_results: value,
        }
    }
}

pub struct AddPinResponse {
    pin_status: models::PinStatus,
}

impl AddPinResponse {
    pub fn new(pin_status: models::PinStatus) -> Self {
        Self {
            pin_status
        }
    }
}

impl IntoResponse for AddPinResponse {
    fn into_response(self) -> Response {
        (common::convert_status_code(202), Json(self.pin_status)).into_response()
    }
}

impl From<models::PinStatus> for AddPinResponse {
    fn from(value: models::PinStatus) -> Self {
        Self {
            pin_status: value,
        }
    }
}

pub struct GetPinByRequestIdResponse {
    pin_status: models::PinStatus,
}

impl GetPinByRequestIdResponse {
    pub fn new(pin_status: models::PinStatus) -> Self {
        Self {
            pin_status
        }
    }
}

impl IntoResponse for GetPinByRequestIdResponse {
    fn into_response(self) -> Response {
        (common::convert_status_code(200), Json(self.pin_status)).into_response()
    }
}

impl From<models::PinStatus> for GetPinByRequestIdResponse {
    fn from(value: models::PinStatus) -> Self {
        Self {
            pin_status: value,
        }
    }
}

/// Empty body.
pub struct DeletePinByRequestIdResponse {}

impl DeletePinByRequestIdResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl IntoResponse for DeletePinByRequestIdResponse {
    fn into_response(self) -> Response {
        (common::convert_status_code(202), ()).into_response()
    }
}

