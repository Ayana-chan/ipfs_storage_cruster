#![allow(dead_code)]

use axum::http;
use axum::response::Response;
use ipfs_node_wrapper_structs::{errors, StandardApiJsonBody};

pub type ApiResult<T> = Result<T, errors::ResponseError>;
pub type ApiResponseResult = ApiResult<Response>;
pub type StandardApiResult<T> = ApiResult<StandardApiJsonBody<T>>;
pub type StandardApiResultStatus<T> = ApiResult<(http::StatusCode, StandardApiJsonBody<T>)>;

