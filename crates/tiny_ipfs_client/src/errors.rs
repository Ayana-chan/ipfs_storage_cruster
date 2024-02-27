
#[derive(Debug, Eq, PartialEq)]
pub enum IpfsClientError {
    /// `reqwest` returns an error when send a request.
    SendRequestFailed,
    /// Respond an unknown status code.
    UnknownStatusCode,
    /// 404 not found.
    NotFound,
    /// RPC request error.
    RpcReject,
    /// RPC respond 500.
    RpcInternalServerError,
    /// Failed to deserialize response body.
    UnexpectedResponseBody,
}
