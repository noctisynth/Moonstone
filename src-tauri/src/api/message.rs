use anyhow::{anyhow, Result};
use oblivion::api;
use serde_json::{json, Value};

pub(crate) async fn new(
    node: &str,
    token: &str,
    community_id: &str,
    message_id: &str,
    text: &str,
) -> Result<bool> {
    let mut res = api::post(
        format!("{node}/message/new").as_str(),
        json!({
            "token": token,
            "node": node,
            "community_id": community_id,
            "message_id": message_id,
            "text": text,
        }),
        true,
    )
    .await?;
    let json = res.json()?;
    match json["status"].as_bool() {
        Some(status) => Ok(status),
        None => Err(anyhow!("数据格式错误！")),
    }
}

pub(crate) async fn get_all(node: &str, token: &str) -> Result<Value> {
    let mut res = api::post(
        format!("{node}/message/get").as_str(),
        json!({"token": token}),
        true,
    )
    .await?;
    let json = res.json()?;
    match json.get("messages") {
        Some(messages) => Ok(messages.clone()),
        None => Err(anyhow!("数据格式错误！")),
    }
}
