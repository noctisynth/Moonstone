use crate::models::{CSession, Community, TSession, Tunnel};
use crate::DB;
use anyhow::{Ok, Result};

pub async fn new_community(
    node: String,
    name: String,
    token: Option<String>,
) -> Result<Vec<CSession>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let new_community_session: Vec<CSession> = DB
        .create("community")
        .content(Community { node, name, token })
        .await?;

    Ok(new_community_session)
}

pub async fn update_community(
    id: String,
    node: String,
    name: String,
    token: Option<String>,
) -> Result<Option<CSession>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let new_community_session: Option<CSession> = DB
        .update(("community", id))
        .content(Community { node, name, token })
        .await?;

    Ok(new_community_session)
}

pub async fn all_communities() -> Result<Vec<CSession>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let all_communities: Vec<CSession> = DB.select("community").await?;
    Ok(all_communities)
}

pub async fn new_tunnel(node: String, token: String, iden: String) -> Result<Vec<TSession>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let new_tunel_session: Vec<TSession> = DB
        .create("tunnel")
        .content(Tunnel { node, token, iden })
        .await?;

    Ok(new_tunel_session)
}
