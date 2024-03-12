use tiny_ipfs_client::IpfsClientError;
use crate::app::errors::*;

impl From<IpfsClientError> for ResponseError {
    fn from(value: IpfsClientError) -> Self {
        match value {
            IpfsClientError::SendRequestFailed => IPFS_CLIENT_ERROR.clone_to_error(),
            IpfsClientError::UnknownStatusCode => IPFS_FAIL.clone_to_error(),
            IpfsClientError::BadRequest => IPFS_REQUEST_ERROR.clone_to_error(),
            IpfsClientError::NotFound => IPFS_NOT_FOUND.clone_to_error(),
            IpfsClientError::RpcReject => IPFS_REQUEST_ERROR.clone_to_error(),
            IpfsClientError::RpcInternalServerError => IPFS_RESPOND_ERROR.clone_to_error(),
            IpfsClientError::UnexpectedResponseBody => IPFS_FAIL.clone_to_error(),
        }
    }
}
