#![allow(dead_code)]

use axum::response::{IntoResponse, Response};
pub use errors_list::*;
use crate::common::convert_status_code;

mod errors_list;

#[derive(Clone, Debug)]
pub struct ResponseError {
    pub code: u16,
    pub msg: String,
}

impl ResponseError {
    pub fn new(status: u16, msg: &str) -> Self {
        ResponseError {
            code: status,
            msg: msg.to_string(),
        }
    }

    pub fn modify_msg(mut self, new_msg: &str) -> Self {
        self.msg = new_msg.to_string();
        self
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        (convert_status_code(self.code), self.msg).into_response()
    }
}

impl From<ResponseErrorStatic> for ResponseError {
    fn from(value: ResponseErrorStatic) -> Self {
        ResponseError {
            code: value.code,
            msg: value.msg.to_string(),
        }
    }
}

/// Use `into()` to convert to `ResponseError`, which impl `IntoResponse`.
#[derive(Clone, Debug)]
pub struct ResponseErrorStatic {
    pub code: u16,
    pub msg: &'static str,
}

impl ResponseErrorStatic {
    pub fn new(status: u16, msg: &'static str) -> Self {
        ResponseErrorStatic {
            code: status,
            msg,
        }
    }

    pub fn modify_msg(mut self, new_msg: &'static str) -> Self {
        self.msg = new_msg;
        self
    }
}



