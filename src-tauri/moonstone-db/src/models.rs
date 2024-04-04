use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct Community {
    pub node: String,
    pub name: String,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CSession {
    pub id: Thing,
    pub node: String,
    pub name: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Tunnel {
    pub node: String,
    pub iden: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct TSession {
    pub id: Thing,
    pub node: String,
    pub iden: String,
    pub token: String,
}
