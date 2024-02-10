use crate::error::ResponseErrorStatic;

macro_rules! define_static_error {
    ($name:ident, $code:expr, $message:expr) => {
        pub static $name: ResponseErrorStatic = ResponseErrorStatic {
            code: $code,
            message: $message,
        };
    };
}

define_static_error!(IPFS_COMMUCATION_FAIL, "C0601", "IPFS node respond an unknown error");
define_static_error!(IPFS_UNKNOWN_ERROR, "C0602", "Fail to contact IPFS node");
define_static_error!(IPFS_NOT_FOUND, "C0603", "IPFS node unreachable");
define_static_error!(IPFS_DOWNLOAD_ERROR, "C0604", "Fail to get data from IPFS node");


