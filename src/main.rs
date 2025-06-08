use std::net::SocketAddr;

use crate::proxy::server::HttpServer;

pub mod proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "127.0.0.1:9000"
        .parse::<SocketAddr>()?;
    let server = HttpServer::new(addr).await?;
    server.run().await?;
    Ok(())
}

