#![allow(dead_code)]

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
pub use errors_list::*;
use serde::Serialize;

mod errors_list;

/// Can be handler's return type.
/// The http status code always be StatusCode::INTERNAL_SERVER_ERROR.
#[derive(Clone, Debug, Serialize)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
}

impl ResponseError {
    pub fn new(code: &str, msg: &str) -> Self {
        ResponseError {
            code: code.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn modify_msg(mut self, new_msg: &str) -> Self {
        self.message = new_msg.to_string();
        self
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

impl From<ResponseErrorStatic> for ResponseError {
    fn from(value: ResponseErrorStatic) -> Self {
        ResponseError {
            code: value.code.to_string(),
            message: value.msg.to_string(),
        }
    }
}

/// Use `into()` to convert to `ResponseError`, which impl `IntoResponse`.
#[derive(Clone, Debug)]
pub struct ResponseErrorStatic {
    pub code: &'static str,
    pub msg: &'static str,
}

impl ResponseErrorStatic {
    pub fn new(code: &'static str, msg: &'static str) -> Self {
        ResponseErrorStatic {
            code,
            msg,
        }
    }

    pub fn modify_msg(mut self, new_msg: &'static str) -> Self {
        self.msg = new_msg;
        self
    }
}



