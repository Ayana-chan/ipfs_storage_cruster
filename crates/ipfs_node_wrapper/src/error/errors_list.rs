use crate::error::ResponseErrorStatic;

//TODO 宏
pub static IPFS_COMMUCATION_FAIL: ResponseErrorStatic = ResponseErrorStatic {
    code: "C0601",
    message: "Fail to contact IPFS node",
};
pub static IPFS_UNKNOWN_ERROR: ResponseErrorStatic = ResponseErrorStatic {
    code: "C0602",
    message: "IPFS node respond an unknown error",
};
pub static IPFS_NOT_FOUND: ResponseErrorStatic = ResponseErrorStatic {
    code: "C0603",
    message: "IPFS node unreachable",
};
pub static IPFS_DOWNLOAD_ERROR: ResponseErrorStatic = ResponseErrorStatic {
    code: "C0604",
    message: "Fail to get data from IPFS node",
};
