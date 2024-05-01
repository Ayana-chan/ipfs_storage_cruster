use crate::app::errors::ResponseErrorStatic;

macro_rules! define_static_error {
    ($name:ident, $code:expr, $message:expr) => {
        pub static $name: ResponseErrorStatic = ResponseErrorStatic {
            code: $code,
            message: $message,
        };
    };
}

define_static_error!(DB_DATA_FAIL, "A1100", "Error about data in database");
define_static_error!(DB_TARGET_DATA_NOT_EXIST, "A1101", "Target data doesn't exist in database");

define_static_error!(SYSTEM_EXECUTION_ERROR, "B0001", "Error in system");

define_static_error!(DB_FAIL, "C0300", "Error about database");

define_static_error!(IPFS_FAIL, "C0600", "Error about IPFS");
define_static_error!(IPFS_CLIENT_ERROR, "C0601", "Error when send request to IPFS node");
define_static_error!(IPFS_NOT_FOUND, "C0602", "IPFS not found");
define_static_error!(IPFS_REQUEST_ERROR, "C0603", "IPFS node rejects the request");
define_static_error!(IPFS_RESPOND_ERROR, "C0604", "IPFS node responds an error");
define_static_error!(IPFS_NODE_CLUSTER_ERROR, "C0650", "Error about IPFS node cluster");
define_static_error!(IPFS_NODE_CLUSTER_UNHEALTHY, "C0650", "IPFS node cluster is too unhealthy to finish the task");
