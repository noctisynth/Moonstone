pub mod models;
pub mod sessions;

use anyhow::{Ok, Result};
use once_cell::sync::Lazy;
use sessions::update_community;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

use crate::sessions::{all_communities, new_community};

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub async fn init() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;
    Ok(())
}

pub async fn test() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;

    new_community(
        "oblivion://127.0.0.1/".to_owned(),
        "测试".to_owned(),
        Some("fuefh34uenj".to_owned()),
    )
    .await?;

    update_community(
        "zzzzzzzz".to_string(),
        "node".to_string(),
        "name".to_string(),
        None,
    )
    .await?;

    let all = all_communities().await?;

    for c in all {
        println!("{:?}", c);
    }
    Ok(())
}
