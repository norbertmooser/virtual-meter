use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_modbus::server::tcp::{accept_tcp_connection, Server};
use anyhow::Result;
use log::{info, error};

use crate::service::ModbusRegisterService;

pub async fn run_modbus_server(socket_addr: SocketAddr) -> Result<()> {
    info!("[SERVER] Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    info!("[SERVER] Listener bound successfully");
    let server = Server::new(listener);
    info!("[SERVER] Server created");
    
    // Create a shared instance of ModbusRegisterService
    let service = Arc::new(ModbusRegisterService::new());
    
    let new_service = move |socket_addr| {
        info!("[SERVER] New service requested for {socket_addr}");
        let service_clone = Arc::clone(&service);
        Ok(Some(service_clone))
    };
    
    let on_connected = |stream: TcpStream, socket_addr: SocketAddr| {
        let new_service = new_service.clone();
        async move {
            info!("[SERVER] New connection from {socket_addr}");
            accept_tcp_connection(stream, socket_addr, new_service)
        }
    };
    
    let on_process_error = |err| {
        error!("[SERVER] Process error: {err}");
    };
    
    info!("[SERVER] Starting server...");
    server.serve(&on_connected, on_process_error).await?;
    info!("[SERVER] Server stopped");
    Ok(())
}
