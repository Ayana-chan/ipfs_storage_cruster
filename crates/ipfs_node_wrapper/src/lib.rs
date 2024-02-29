pub mod app;
pub mod admin_app;
pub mod public_app;
mod utils;
mod error_convert;

// TODO 鉴权参考oss，可能是：用户请求中央服务器，中央服务器生成token并发送到对应节点服务器中，
// 之后用户访问对应服务器时消耗token。这样可以防止用户频繁请求压垮主服务器。token无需加密，也可以是任意数据结构。带上requestid即可保证唯一性。

