use std::net::SocketAddr;
mod client;
mod meter_config;
use client::client_context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = "127.0.0.1:5502".parse()?;
    
    println!("Starting client...");
    client_context(socket_addr, "./config/meter_config.yaml").await;
    println!("Client finished.");

    Ok(())
}

