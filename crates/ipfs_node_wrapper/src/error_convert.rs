use ipfs_node_wrapper_app_structs::errors::*;
use tiny_ipfs_client::IpfsClientError;

pub fn from_ipfs_client_error(value: IpfsClientError) -> ResponseError {
    match value {
        IpfsClientError::SendRequestFailed => IPFS_CLIENT_ERROR.clone_to_error(),
        IpfsClientError::UnknownStatusCode => IPFS_FAIL.clone_to_error(),
        IpfsClientError::NotFound => IPFS_RPC_NOT_FOUND.clone_to_error(),
        IpfsClientError::RpcReject => IPFS_REQUEST_ERROR.clone_to_error(),
        IpfsClientError::RpcInternalServerError => IPFS_RESPOND_ERROR.clone_to_error(),
        IpfsClientError::UnexpectedResponseBody => IPFS_FAIL.clone_to_error(),
    }
}
