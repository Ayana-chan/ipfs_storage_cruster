use serde::de::DeserializeOwned;
use tracing::error;
use ipfs_node_wrapper_structs::errors;
use crate::common::{ClientResult, ClientErrorType, CommunicationErrorType};

pub async fn handle_client_response<T>(response: Result<reqwest::Response, reqwest::Error>) -> ClientResult<T>
where T: DeserializeOwned{
    let response = response.map_err(handle_request_error::<T>)?;
    let status = response.status();
    match status {
        _ if status.is_success() => {
            let content: T = response.json().await.map_err(|_e| {
                error!("Unexpected response body. msg: {:?}", _e);
                CommunicationErrorType::UnexpectedResponseBody
            })?;
            Ok(content)
        },
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            let content: errors::ResponseError = response.json().await.map_err(|_e| {
                error!("Unexpected internal server error. msg: {:?}", _e);
                CommunicationErrorType::InternalServerError
            })?;
            Err(ClientErrorType::ServerExplictError(content))
        },
        reqwest::StatusCode::NOT_FOUND => {
            error!("Server not found.");
            Err(CommunicationErrorType::NotFound.into())
        }
        _ => {
            error!("Unknown error. Status code: {}", status);
            Err(CommunicationErrorType::UnknownError.into())
        }
    }
}

/// Handle the error returned by `send()`.
pub fn handle_request_error<T>(err: reqwest::Error) -> ClientErrorType {
    error!("There was an error while sending request, \
    redirect loop was detected or redirect limit was exhausted.\
    msg: {}", err);
    CommunicationErrorType::RequestError.into()
}
