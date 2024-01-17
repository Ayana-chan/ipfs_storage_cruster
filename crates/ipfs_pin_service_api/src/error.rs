use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorContent {
    reason: String,
    details: String,
}

#[derive(Serialize)]
struct GenericResponseError {
    error: ErrorContent
}

impl GenericResponseError {
    pub fn new(reason: &str, detail: &str) -> Self{
        let err = ErrorContent{
            reason: reason.into(),
            details: detail.into()
        };
        Self{
            error: err,
        }
    }
}

pub enum ResponseErrorType {
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
            ResponseError::InsufficientFunds => {
                (409,
                 GenericResponseError::new(
                     "INSUFFICIENT_FUNDS",
                     &self.detail.unwrap_or("Unable to process request due to the lack of funds".into())
                 ))
            }
        };
        error_with_status.into_response()
    }
}



