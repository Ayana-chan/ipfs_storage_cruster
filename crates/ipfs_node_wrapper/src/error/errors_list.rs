use crate::error::ResponseErrorStatic;

macro_rules! define_static_error {
    ($name:ident, $code:expr, $message:expr) => {
        pub static $name: ResponseErrorStatic = ResponseErrorStatic {
            code: $code,
            message: $message,
        };
    };
}

define_static_error!(IPFS_FAIL, "C0600", "Fail about IPFS");
define_static_error!(IPFS_COMMUCATION_FAIL, "C0601", "Fail to contact IPFS node");
define_static_error!(IPFS_UNKNOWN_ERROR, "C0602", "IPFS node respond an unknown error");
define_static_error!(IPFS_GATEWAY_NOT_FOUND, "C0603", "IPFS gateway not found");
define_static_error!(IPFS_DOWNLOAD_ERROR, "C0604", "Fail to get data from IPFS node");
define_static_error!(IPFS_RPC_NOT_FOUND, "C0605", "RPC endpoint doesn't exist");
define_static_error!(IPFS_RPC_REJECT, "C0606", "IPFS Node reject request");
define_static_error!(IPFS_RPC_INTERNAL_ERROR, "C0607", "IPFS node has internal error");

