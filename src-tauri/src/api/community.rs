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
) -> Result<i64, Exception> {
    let mut res = match api::post(
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
        Some(_) => {},
        None => {
            return Err(Exception::BadResponse {
                detail: res.text().unwrap(),
            });
        }
    };

    Ok(json["community_id"].as_i64().unwrap())
}
