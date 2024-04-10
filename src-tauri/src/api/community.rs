use anyhow::{Error, Result};
use oblivion::api;
use serde_json::json;

use crate::exceptions::Exception;

pub async fn new(
    node: &str,
    session_key: &str,
    name: &str,
    security_level: i32,
    cross_origin: bool,
    token: Option<String>,
) -> Result<String> {
    let mut res = api::post(
        format!("{node}/community/new").as_str(),
        json!({
            "session_key": session_key,
            "name": name,
            "security_level": security_level,
            "cross_origin": cross_origin,
            "token": token
        }),
        true,
    )
    .await?;

    let json = res.json()?;
    match json["status"].as_bool() {
        Some(_) => {}
        None => {
            return Err(Error::from(Exception::BadResponse {
                detail: res.text().unwrap(),
            }));
        }
    };

    Ok(json["community_id"].as_str().unwrap().to_string())
}
