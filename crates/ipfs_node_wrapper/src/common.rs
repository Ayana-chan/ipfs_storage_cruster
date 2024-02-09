use axum::http;
use crate::error;

pub type ApiResponse<T> = Result<T, error::ResponseError>;

/// convert u16 to http::StatusCode
pub fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}
