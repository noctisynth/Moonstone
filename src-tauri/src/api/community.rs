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

pub async fn add(node: &str, session_key: &str, user_id: &str, community_id: &str) -> Result<()> {
    let mut res = api::post(
        format!("{node}/member/new").as_str(),
        json!({
            "node": node,
            "token": session_key,
            "user_id": user_id,
            "community_id": community_id,
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

    Ok(())
}
