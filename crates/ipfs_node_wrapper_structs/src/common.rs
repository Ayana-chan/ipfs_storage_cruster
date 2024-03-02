#[cfg(feature = "server")]
use axum::{http, Json, response::{IntoResponse, Response}};
use serde::{Serialize, Deserialize};
use crate::errors;

pub type ApiResult<T> = Result<T, errors::ResponseError>;
#[cfg(feature = "server")]
pub type ApiResponseResult = ApiResult<Response>;
pub type StandardApiResult<T> = ApiResult<StandardApiJsonBody<T>>;
#[cfg(feature = "server")]
pub type StandardApiResultStatus<T> = ApiResult<(http::StatusCode, StandardApiJsonBody<T>)>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardApiJsonBody<T: Serialize> {
    pub code: String,
    pub message: String,
    pub data: T,
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
