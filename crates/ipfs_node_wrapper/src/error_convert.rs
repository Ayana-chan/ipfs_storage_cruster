use ipfs_node_wrapper_structs::errors::*;
use tiny_ipfs_client::IpfsClientError;

pub fn from_ipfs_client_error(value: IpfsClientError) -> ResponseError {
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

#[allow(dead_code)]
pub fn from_ipfs_client_error_with_log(value: IpfsClientError) -> ResponseError {
    from_ipfs_client_error(value).log()
}
