use oblivion::exceptions::Exception as OblivionException;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Exception {
    #[error("账户标识[{identity}]使用密钥[{password}]身份验证失败")]
    AuthenticationFailed { identity: String, password: String },
    #[error("向服务端发起请求失败: {error:?}, 这可能是服务端遭到网络攻击")]
    ConnectionError { error: OblivionException },
    #[error(
        "捕获到了不符合预期格式的数据: {detail},  这可能是由于您遭到了网络劫持, 必要时建议紧急避难"
    )]
    BadResponse { detail: String },
    #[error("请求失败: {detail}")]
    RequestFailed { detail: String },
}
