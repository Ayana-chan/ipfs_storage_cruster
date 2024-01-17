use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub enum ResponseErrorType {
    BadRequest,
    InsufficientFunds,
}

pub struct ResponseError{
    err_type: ResponseErrorType,
    detail: Option<String>,
}

impl ResponseError {
    pub fn new(err_type: ResponseErrorType) -> Self{
        Self {
            err_type,
            detail: None,
        }
    }

    pub fn detail(mut self, detail: &str) -> Self{
        self.detail = Some(detail.into());
        self
    }
}

impl IntoResponse for ResponseError{
    fn into_response(self) -> Response {
        let error_with_status = match self.err_type {
            ResponseErrorType::BadRequest => {
                (400,
                 GenericResponseError::new_json(
                     "BAD_REQUEST",
                     &self.detail.unwrap_or("Bad Request".into())
                 ))
            },
            ResponseErrorType::InsufficientFunds => {
                (409,
                 GenericResponseError::new_json(
                     "INSUFFICIENT_FUNDS",
                     "Unable to process request due to the lack of funds"
                 ))
            },
        };
        error_with_status.into_response()
    }
}

#[derive(Serialize)]
struct ErrorContent {
    reason: String,
    #[serde(skip_serializing_if="Option::is_none")]
    details: String,
}

#[derive(Serialize)]
struct GenericResponseError {
    error: ErrorContent
}

impl GenericResponseError {
    pub fn new_json(reason: &str, detail: &str) -> Json<Self> {
        let err = ErrorContent{
            reason: reason.into(),
            details: detail.into()
        };
        let ret = Self{
            error: err,
        };
        Json(ret)
    }
}

