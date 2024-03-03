use crate::app::errors::ResponseErrorStatic;

macro_rules! define_static_error {
    ($name:ident, $code:expr, $message:expr) => {
        pub static $name: ResponseErrorStatic = ResponseErrorStatic {
            code: $code,
            message: $message,
        };
    };
}

define_static_error!(IPFS_FAIL, "C0600", "Error about IPFS");

