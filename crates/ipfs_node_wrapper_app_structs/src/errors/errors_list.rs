use crate::errors::ResponseErrorStatic;

macro_rules! define_static_error {
    ($name:ident, $code:expr, $message:expr) => {
        pub static $name: ResponseErrorStatic = ResponseErrorStatic {
            code: $code,
            message: $message,
        };
    };
}

define_static_error!(IPFS_FAIL, "C0600", "Error about IPFS");
define_static_error!(IPFS_CLIENT_ERROR, "C0601", "Error when send request to IPFS node");
define_static_error!(IPFS_RPC_NOT_FOUND, "C0602", "IPFS RPC endpoint doesn't exist");
define_static_error!(IPFS_REQUEST_ERROR, "C0603", "IPFS node rejects the request");
define_static_error!(IPFS_RESPOND_ERROR, "C0604", "IPFS node responds an error");

