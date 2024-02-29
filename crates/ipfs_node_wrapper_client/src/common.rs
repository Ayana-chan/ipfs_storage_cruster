use ipfs_node_wrapper_structs::{errors, StandardApiJsonBody};

pub type ClientResult<T> = Result<T, ClientErrorType>;
pub type StandardClientResult<T> = ClientResult<StandardApiJsonBody<T>>;

#[derive(Debug)]
pub enum ClientErrorType {
    ServerExplictError(errors::ResponseError),
    CommunicationError(CommunicationErrorType),
}

impl From<CommunicationErrorType> for ClientErrorType {
    fn from(value: CommunicationErrorType) -> Self {
        ClientErrorType::CommunicationError(value)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CommunicationErrorType {
    RequestError,
    NotFound,
    /// Unexpected 500
    InternalServerError,
    UnexpectedResponseBody,
    UnknownError,
}


