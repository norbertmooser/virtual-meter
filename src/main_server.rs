mod server;
mod meter_config;
use crate::server::server_context;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = "127.0.0.1:5502".parse()?;
    println!("Starting Modbus TCP server");
    server_context(socket_addr, "./config/meter_config.yaml").await?;
    Ok(())
}
