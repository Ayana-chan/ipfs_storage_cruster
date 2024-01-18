//! `api::ApiResponse<T>` is more recommended to use, which is declared as `Result<Json<T>, ResponseError>`.

use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::common::convert_status_code;

pub enum ResponseErrorType {
    BadRequest,
    InsufficientFunds,
}

pub struct ResponseError {
    err_type: ResponseErrorType,
    detail: Option<String>,
}

impl ResponseError {
    pub fn new(err_type: ResponseErrorType) -> Self {
        Self {
            err_type,
            detail: None,
        }
    }

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


