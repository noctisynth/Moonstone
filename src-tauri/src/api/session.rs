use oblivion::api;
use serde_json::json;

use crate::exceptions::Exception;

pub async fn alive(server: &str, session_key: &str) -> Result<bool, Exception> {
    let mut res = match api::post(
        format!("{server}/session/alive").as_str(),
        json!({"session_key": session_key}),
        true,
    )
    .await
    {
        Ok(res) => res,
        Err(error) => {
            return Err(Exception::ConnectionError { error });
        }
    };
    let json = match res.json() {
        Ok(json) => json,
        Err(error) => {
            return Err(Exception::BadResponse {
                detail: error.to_string(),
            });
        }
    };
    match json["status"].as_bool() {
        Some(status) => Ok(status),
        None => {
            return Err(Exception::BadResponse {
                detail: res.text().unwrap(),
            });
        }
    }
}
