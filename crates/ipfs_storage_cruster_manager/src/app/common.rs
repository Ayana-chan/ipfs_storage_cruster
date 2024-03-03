use axum::{http, Json, response::{IntoResponse, Response}};
use serde::{Serialize, Deserialize};
use crate::app::errors;

pub type ApiResult<T> = Result<T, errors::ResponseError>;
pub type ApiResponseResult = ApiResult<Response>;
pub type StandardApiResult<T> = ApiResult<StandardApiJsonBody<T>>;
pub type StandardApiResultStatus<T> = ApiResult<(http::StatusCode, StandardApiJsonBody<T>)>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardApiJsonBody<T: Serialize> {
    pub code: String,
    pub message: String,
    pub data: T,
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
            data: value,
        }
    }
}

