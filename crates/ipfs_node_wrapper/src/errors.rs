
pub type ErrorContentStruct = (u16, &'static str);

pub static IPFS_COMMUCATION_FAIL: ErrorContentStruct = (530, "Something error in communication to IPFS node");
pub static IPFS_UNKNOWN_ERROR: ErrorContentStruct = (531, "Something error returned from IPFS node");
pub static IPFS_DOWNLOAD_ERROR: ErrorContentStruct = (532, "Something error when download bytes from IPFS node");

