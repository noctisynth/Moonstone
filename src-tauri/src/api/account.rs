use oblivion::api;
use serde_json::{json, Value};

use crate::exceptions::Exception;

pub async fn login(
    server: &str,
    identity: &str,
    password: &str,
    unique_id: &str,
) -> Result<String, Exception> {
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
                                    return Err(Exception::BadResponse {
                                        detail: res.text().unwrap(),
                                    });
                                }
                            }
                            None => {
                                return Err(Exception::BadResponse {
                                    detail: res.text().unwrap(),
                                });
                            }
                        };
                        match result["session_key"].as_str() {
                            Some(session_key) => Ok(session_key.to_string()),
                            None => {
                                return Err(Exception::BadResponse {
                                    detail: res.text().unwrap(),
                                });
                            }
                        }
                    }
                    Err(_) => Err(Exception::BadResponse {
                        detail: res.text().unwrap(),
                    }),
                }
            } else {
                Err(Exception::AuthenticationFailed {
                    identity: identity.to_string(),
                    password: password.to_string(),
                })
            }
        }
        Err(error) => Err(Exception::ConnectionError { error }),
    }
}

pub async fn account(server: &str, session_key: &str) -> Result<Value, Exception> {
    let mut res = match api::post(
        format!("{server}/account").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(Exception::ConnectionError { error: error });
        }
    };

    match res.json() {
        Ok(json) => Ok(json),
        Err(error) => Err(Exception::BadResponse {
            detail: error.to_string(),
        }),
    }
}
