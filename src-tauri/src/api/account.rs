use oblivion::api;
use serde_json::{json, Value};

use crate::exceptions::QuantumixException;

// pub async fn login(username: &str, tuta_mail: &str, password: &str, nickname: &str) {}

pub async fn login(
    server: &str,
    identity: &str,
    password: &str,
    unique_id: &str,
) -> Result<String, QuantumixException> {
    match api::post(
        format!("{server}/login").as_str(),
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
                                    return Err(QuantumixException::BadResponse {
                                        detail: res.text().unwrap(),
                                    });
                                }
                            }
                            None => {
                                return Err(QuantumixException::BadResponse {
                                    detail: res.text().unwrap(),
                                });
                            }
                        };
                        match result["session_key"].as_str() {
                            Some(session_key) => Ok(session_key.to_string()),
                            None => {
                                return Err(QuantumixException::BadResponse {
                                    detail: res.text().unwrap(),
                                });
                            }
                        }
                    }
                    Err(_) => Err(QuantumixException::BadResponse {
                        detail: res.text().unwrap(),
                    }),
                }
            } else {
                Err(QuantumixException::AuthenticationFailed {
                    identity: identity.to_string(),
                    password: password.to_string(),
                })
            }
        }
        Err(error) => Err(QuantumixException::ConnectionError { error }),
    }
}

pub async fn session(server: &str, session_key: &str) -> Result<bool, QuantumixException> {
    let mut res = match api::post(
        format!("{server}/session").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(QuantumixException::ConnectionError { error: error });
        }
    };
    let json = match res.json() {
        Ok(json) => json,
        Err(error) => {
            return Err(QuantumixException::BadResponse {
                detail: error.to_string(),
            });
        }
    };
    match json["status"].as_bool() {
        Some(status) => Ok(status),
        None => {
            return Err(QuantumixException::BadResponse {
                detail: res.text().unwrap(),
            });
        }
    }
}

pub async fn account(server: &str, session_key: &str) -> Result<Value, QuantumixException> {
    let mut res = match api::post(
        format!("{server}/account").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(QuantumixException::ConnectionError { error: error });
        }
    };

    match res.json() {
        Ok(json) => Ok(json),
        Err(error) => Err(QuantumixException::BadResponse {
            detail: error.to_string(),
        }),
    }
}
