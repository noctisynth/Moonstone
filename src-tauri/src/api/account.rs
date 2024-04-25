use anyhow::{Error, Result};
use oblivion::models::client::Client;
use serde_json::{json, Value};

use crate::exceptions::Exception;

pub async fn login(
    server: &str,
    identity: &str,
    password: &str,
    unique_id: &str,
) -> Result<String> {
    let client = Client::connect(format!("{server}/session/new").as_str()).await?;
    client
        .send_json(
            json!({"identity": identity, "password": password, "unique_id": unique_id}),
            200,
        )
        .await?;
    let res = client.recv().await?;

    if res.ok() {
        let result = res.json()?;
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
    } else {
        Err(Error::from(Exception::AuthenticationFailed {
            identity: identity.to_string(),
            password: password.to_string(),
        }))
    }
}

pub async fn profile(node: &str, session_key: &str) -> Result<Value> {
    let client = Client::connect( format!("{node}/account/profile").as_str()).await?;
    client
        .send_json(
            json!({"session_key": session_key}),
            200,
        )
        .await?;
    let res = client.recv().await?;

    Ok(res.json()?)
}

pub(crate) async fn register(
    node: &str,
    tuta_mail: &str,
    username: &str,
    nickname: &str,
    password: &str,
) -> Result<Value> {
    let client = Client::connect( format!("{node}/account/new").as_str()).await?;
    client
        .send_json(
            json!({"tuta_mail": tuta_mail, "username": username, "nickname": nickname, "password": password}),
            200,
        )
        .await?;
    let res = client.recv().await?;

    let json = res.json()?;

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
