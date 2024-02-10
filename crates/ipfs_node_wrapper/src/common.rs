#![allow(dead_code)]

use axum::{http, Json};
use axum::response::{IntoResponse, Response};
use crate::error;
use serde::Serialize;

pub type ApiResult<T> = Result<T, error::ResponseError>;
pub type ApiResponseResult = ApiResult<Response>;
pub type StandardApiResult<T> = Result<StandardApiJsonBody<T>, error::ResponseError>;

// TODO untested
#[derive(Clone, Debug, Serialize)]
pub struct StandardApiJsonBody<T: Serialize> {
    code: String,
    message: String,
    data: T,
}

impl<T: Serialize> IntoResponse for StandardApiJsonBody<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> From<T> for StandardApiJsonBody<T> {
    fn from(value: T) -> Self {
        StandardApiJsonBody {
            code: "00000".to_string(),
            message: "success".to_string(),
            data: value
        }
    }
}

/// convert u16 to http::StatusCode
pub fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}
