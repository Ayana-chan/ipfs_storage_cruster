use axum::http;

/// convert u16 to http::StatusCode
pub(crate) fn convert_status_code(code: u16) -> http::StatusCode {
    http::StatusCode::from_u16(code).expect("Fatal: Invalid Http Status Code.")
}