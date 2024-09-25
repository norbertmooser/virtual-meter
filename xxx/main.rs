// SPDX-FileCopyrightText: Copyright (c) 2017-2024 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # TCP server example
//!
//! This example shows how to start a server and implement basic register
//! read/write operations.
mod server;
mod client;
mod service;
mod config_load;

use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

use server::run_modbus_server;
use client::test_modbus_server;
use config_load::{load_config, MeterConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration
    let config: MeterConfig = load_config("config/meter_config.yaml")?;
    println!("Loaded configuration: {:?}", config);

    let socket_addr: SocketAddr = format!("{}:{}", config.meter_data.ip, config.meter_data.port).parse()?;
    println!("Using socket address: {}", socket_addr);

    // Spawn the server task
    let server_handle = tokio::spawn(async move {
        match run_modbus_server(socket_addr).await {
            Ok(_) => println!("Server finished successfully"),
            Err(e) => eprintln!("Server error: {}", e),
        }
    });

    // Give the server some time to start up
    sleep(Duration::from_secs(1)).await;

    // Run the client
    test_modbus_server(socket_addr).await;

    // Wait for the server to finish or timeout
    tokio::select! {
        _ = server_handle => {
            println!("Server task completed");
        },
        _ = sleep(Duration::from_secs(3600)) => {
            println!("Timeout reached. Exiting...");
        }
    }

    println!("Main function exiting");
    Ok(())
}