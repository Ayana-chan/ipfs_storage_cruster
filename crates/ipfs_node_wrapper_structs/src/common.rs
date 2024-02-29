#[cfg(feature = "server")]
use axum::{http, Json, response::{IntoResponse, Response}};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardApiJsonBody<T: Serialize> {
    code: String,
    message: String,
    data: T,
}

#[cfg(feature = "server")]
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
            data: value,
        }
    }
}
