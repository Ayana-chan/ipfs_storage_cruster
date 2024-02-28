#![allow(dead_code)]

use serde::{Serialize, Deserialize};

mod errors_list;

pub use errors_list::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
}



