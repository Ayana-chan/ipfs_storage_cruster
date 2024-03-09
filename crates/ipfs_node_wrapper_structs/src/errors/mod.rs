#![allow(dead_code)]

#[cfg(feature = "server")]
use axum::{http::StatusCode,
           Json,
           response::{IntoResponse, Response}
};
use tracing::error;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

mod errors_list;

pub use errors_list::*;

/// Can be handler's return type.
/// The http status code always be StatusCode::INTERNAL_SERVER_ERROR.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn log(self) -> Self {
        error!(self.message);
        self
    }
}

#[cfg(feature = "server")]
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
#[derive(Clone, Debug, Serialize)]
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
    pub fn clone_to_error_with_log_with_content(&self, err: impl Debug) -> ResponseError {
        let err_log = format!("{}: {:?}", self.message, err);
        error!(err_log);
        self.clone().into()
    }
}



