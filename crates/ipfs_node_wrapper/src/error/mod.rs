#![allow(dead_code)]

use std::error::Error;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
pub use errors_list::*;
use serde::Serialize;
use tracing::error;

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
            message: value.message.to_string(),
        }
    }
}

/// Use `into()` to convert to `ResponseError`, which impl `IntoResponse`.
#[derive(Clone, Debug)]
pub struct ResponseErrorStatic {
    pub code: &'static str,
    pub message: &'static str,
}

impl ResponseErrorStatic {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        ResponseErrorStatic {
            code,
            message,
        }
    }

    pub fn modify_msg(mut self, new_msg: &'static str) -> Self {
        self.message = new_msg;
        self
    }

    /// Convert `ResponseErrorStatic` to `Err(ResponseError)`.
    pub fn clone_to_error(&self) -> ResponseError {
        self.clone().into()
    }

    /// Convert `ResponseErrorStatic` to `Err(ResponseError)`,
    /// and output an error log of `message`.
    pub fn clone_to_error_with_log(&self) -> ResponseError {
        error!(self.message);
        self.clone().into()
    }

    /// Convert `ResponseErrorStatic` to `Err(ResponseError)`,
    /// and output an error log of `message` and `Error`.
    pub fn clone_to_error_with_log_error(&self, err: impl Error) -> ResponseError {
        let err_log = self.message.to_string() + ": " + &err.to_string();
        error!(err_log);
        self.clone().into()
    }
}



