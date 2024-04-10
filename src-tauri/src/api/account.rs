use anyhow::{Error, Result};
use oblivion::api;
use serde_json::{json, Value};

use crate::exceptions::Exception;

pub async fn login(
    server: &str,
    identity: &str,
    password: &str,
    unique_id: &str,
) -> Result<String> {
    match api::post(
        format!("{server}/session/new").as_str(),
        json!({"identity": identity, "password": password, "unique_id": unique_id}),
        true,
    )
    .await
    {
        Ok(mut res) => {
            if res.ok() {
                match res.json() {
                    Ok(result) => {
                        match result["status"].as_bool() {
                            Some(status) => {
                                if !status {
                                    return Err(Error::from(Exception::BadResponse {
                                        detail: res.text().unwrap(),
                                    }));
                                }
                            }
                            None => {
                                return Err(Error::from(Exception::BadResponse {
                                    detail: res.text().unwrap(),
                                }));
                            }
                        };
                        match result["session_key"].as_str() {
                            Some(session_key) => Ok(session_key.to_string()),
                            None => {
                                return Err(Error::from(Exception::BadResponse {
                                    detail: res.text().unwrap(),
                                }));
                            }
                        }
                    }
                    Err(_) => Err(Error::from(Exception::BadResponse {
                        detail: res.text().unwrap(),
                    })),
                }
            } else {
                Err(Error::from(Exception::AuthenticationFailed {
                    identity: identity.to_string(),
                    password: password.to_string(),
                }))
            }
        }
        Err(error) => Err(Error::from(Exception::ConnectionError { error })),
    }
}

pub async fn profile(node: &str, session_key: &str) -> Result<Value> {
    let mut res = match api::post(
        format!("{node}/account/profile").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(Error::from(Exception::ConnectionError { error }));
        }
    };

    match res.json() {
        Ok(json) => Ok(json),
        Err(error) => Err(Error::from(Exception::BadResponse {
            detail: error.to_string(),
        })),
    }
}

pub(crate) async fn register(
    node: &str,
    tuta_mail: &str,
    username: &str,
    nickname: &str,
    password: &str,
) -> Result<Value> {
    let mut res = match api::post(
        format!("{node}/account/new").as_str(),
        json!({"tuta_mail": tuta_mail, "username": username, "nickname": nickname, "password": password}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(Error::from(Exception::ConnectionError { error }));
        }
    };

    match res.json() {
        Ok(json) => {
            let status = if json["status"].as_bool().is_none() {
                false
            } else {
                json["status"].as_bool().unwrap()
            };
            if status {
                Ok(json)
            } else {
                Err(Error::from(Exception::RequestFailed {
                    detail: json["msg"].as_str().unwrap().to_string(),
                }))
            }
        }
        Err(error) => Err(Error::from(Exception::BadResponse {
            detail: error.to_string(),
        })),
    }
}
