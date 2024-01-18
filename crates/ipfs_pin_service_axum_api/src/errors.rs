//! Errors for API's response. \
//! Use [`api::ApiResponse<T>`](crate::api::ApiResponse) as return type is more convenient,
//! which is declared as `Result<Json<T>, ResponseError>`. \
//! Look at [`ResponseError`] for more usage.

use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::common::convert_status_code;

/// Error types supported by `ResponseError`.
pub enum ResponseErrorType {
    BadRequest,
    Unauthorized,
    NotFound,
    InsufficientFunds,
}
// TODO 定制error

/// Structure to generate error object to response (by [`IntoResponse`] trait). \
/// Just returning `ResponseError` in handler is ok. For example: \
/// ```no_run
/// # use ipfs_pin_service_axum_api::errors::{ResponseError, ResponseErrorType};
/// async fn handle_404() -> ResponseError {
///     ResponseError::new(ResponseErrorType::NotFound)
/// }
/// ```
///
/// ## Low Reusability
/// All [error types](ResponseErrorType) and their status code and default content are hard-coded. \
/// Only support one response format: \
/// ```json
/// # "reason" is required and "details" is optional
/// {
///     "error": {
///         "reason": "NOT_FOUND",
///         "details": "The specified resource was not found"
///     }
/// }
/// ```
/// So think twice before reusing this module in **other parts** of your code.
pub struct ResponseError {
    err_type: ResponseErrorType,
    detail: Option<String>,
}

impl ResponseError {
    /// Create new error with certain error type.
    pub fn new(err_type: ResponseErrorType) -> Self {
        Self {
            err_type,
            detail: None,
        }
    }

    /// Customize `"detail"`
    pub fn detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.into());
        self
    }

    fn obtain_detail(&mut self, default_detail: Option<&str>) -> Option<&str> {
        if self.detail == None{
            self.detail = default_detail.map(String::from);
        }
        self.detail.as_deref()
    }
}

impl IntoResponse for ResponseError {
    fn into_response(mut self) -> Response {
        let error_with_status = match self.err_type {
            ResponseErrorType::BadRequest => {
                (
                    convert_status_code(400),
                    GenericResponseError::new_json(
                        "BAD_REQUEST",
                        self.obtain_detail(None),
                    )
                )
            }
            ResponseErrorType::Unauthorized => {
                (
                    convert_status_code(401),
                    GenericResponseError::new_json(
                        "UNAUTHORIZED",
                        self.obtain_detail(Some("Access token is missing or invalid")),
                    )
                )
            }
            ResponseErrorType::NotFound => {
                (
                    convert_status_code(404),
                    GenericResponseError::new_json(
                        "NOT_FOUND",
                        self.obtain_detail(Some(
                            "The specified resource was not found"
                        )),
                    )
                )
            }
            ResponseErrorType::InsufficientFunds => {
                (
                    convert_status_code(409),
                    GenericResponseError::new_json(
                        "INSUFFICIENT_FUNDS",
                        self.obtain_detail(Some(
                            "Unable to process request due to the lack of funds"
                        )),
                    )
                )
            }
        };
        error_with_status.into_response()
    }
}

#[derive(Serialize)]
struct ErrorContent {
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

#[derive(Serialize)]
struct GenericResponseError {
    error: ErrorContent,
}

impl GenericResponseError {
    pub fn new_json(reason: &str, detail: Option<&str>) -> Json<Self> {
        let err = ErrorContent {
            reason: reason.into(),
            details: detail.map(String::from),
        };
        let ret = Self {
            error: err,
        };
        Json(ret)
    }
}


