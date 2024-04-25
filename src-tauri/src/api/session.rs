use anyhow::{Error, Result};
use oblivion::models::client::Client;
use serde_json::json;

use crate::exceptions::Exception;

pub async fn alive(server: &str, session_key: &str) -> Result<bool> {
    let client = Client::connect(format!("{server}/session/alive").as_str()).await?;
    client
        .send_json(
            json!({"session_key": session_key}),
            200,
        )
        .await?;
    let res = client.recv().await?;

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
