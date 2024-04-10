use anyhow::{Error, Result};
use oblivion::api;
use serde_json::json;

use crate::exceptions::Exception;

pub async fn alive(server: &str, session_key: &str) -> Result<bool> {
    let mut res = api::post(
        format!("{server}/session/alive").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await?;

    let json = res.json()?;

    match json["status"].as_bool() {
        Some(status) => Ok(status),
        None => {
            return Err(Error::from(Exception::BadResponse {
                detail: res.text().unwrap(),
            }));
        }
    }
}
