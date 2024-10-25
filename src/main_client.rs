use std::net::SocketAddr;

// Import the client_context function from tcp_server module
mod client;
use client::client_context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = "127.0.0.1:5502".parse()?;
    
    println!("Starting client...");
    client_context(socket_addr).await;
    println!("Client finished.");

    Ok(())
}

