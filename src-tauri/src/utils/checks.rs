use anyhow::Result;
use oblivion::models::client::Client;
use std::net::TcpStream;

pub(crate) fn internet() -> bool {
    match TcpStream::connect("example.com:443") {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) fn system() -> bool {
    true
}

pub(crate) fn security() -> bool {
    match TcpStream::connect("google.com:443") {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) async fn node_status(node: &str) -> Result<()> {
    let client = Client::connect(node).await?;
    match client.recv().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
