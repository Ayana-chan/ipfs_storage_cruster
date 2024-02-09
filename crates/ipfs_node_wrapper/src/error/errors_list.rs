use crate::error::ResponseErrorStatic;

//TODO 宏
pub static IPFS_COMMUCATION_FAIL: ResponseErrorStatic = ResponseErrorStatic {
    code: 530,
    msg: "Fail to contact IPFS node",
};
pub static IPFS_UNKNOWN_ERROR: ResponseErrorStatic = ResponseErrorStatic {
    code: 531,
    msg: "IPFS node return an error",
};
pub static IPFS_DOWNLOAD_ERROR: ResponseErrorStatic = ResponseErrorStatic {
    code: 532,
    msg: "Fail to get data from IPFS node",
};
