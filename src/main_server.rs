mod server;
use crate::server::server_context;
// use crate::server::service_init::{RegisterData, initialize_registers};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = "127.0.0.1:5502".parse()?;

    println!("Starting Modbus TCP server");
    
    // Initialize registers before starting the server
    // let registers: RegisterData = initialize_registers();
    
    // Pass the initialized registers to server_context
    server_context(socket_addr).await?;

    Ok(())
}
