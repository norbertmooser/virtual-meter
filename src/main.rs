// SPDX-FileCopyrightText: Copyright (c) 2017-2024 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Modbus TCP Server and Client Example
//!
//! This module implements a Modbus TCP server and client, demonstrating:
//! - Server setup and basic register read/write operations
//! - Client connection and interaction with the server
//! - Configuration loading and logging
//!
//! ## Features
//! - Asynchronous Modbus TCP server
//! - Modbus TCP client for testing
//! - Configuration management
//! - Logging and error handling
//!
//! ## Usage
//! Run the program to start the Modbus TCP server and perform client operations.
//!
//! ## License
//! This code is licensed under either of
//! - MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
//! - Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//! at your option.

mod config_load;
mod client;
mod server;
mod service;
mod logger;

// Standard library imports
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

// External crate imports
use anyhow::Result;
use log::{info, error};
use tokio::time::sleep;
use tokio_modbus::client::tcp::connect;
use tokio_modbus::prelude::*;

// Internal module imports
use crate::config_load::{load_config, MeterConfig};
use client::test_modbus_server;
use logger::init_logger;
use server::run_modbus_server;
use service::ModbusRegisterService;

async fn perform_bulk_read(socket_addr: SocketAddr, config: &MeterConfig) -> Result<()> {
    let mut ctx = connect(socket_addr).await?;

    // Determine the start address and quantity for bulk read
    let start_address = config.read_registers.iter().map(|r| r.address).min().unwrap_or(0);
    let end_address = config.read_registers.iter().map(|r| r.address).max().unwrap_or(0);
    let quantity = end_address - start_address + 1;

    info!("[MAIN] Performing bulk read from address {} to {}", start_address, end_address);

    // Perform bulk read
    let result = ctx.read_holding_registers(start_address, quantity).await?;

    match result {
        Ok(values) => {
            info!("[MAIN] Bulk read successful. Read {} values.", values.len());
            for (i, value) in values.iter().enumerate() {
                let address = start_address + i as u16;
                if let Some(register) = config.read_registers.iter().find(|r| r.address == address) {
                    info!("[MAIN] Register {} (address {}): {}", register.name, address, value);
                }
            }
        },
        Err(e) => {
            error!("[MAIN] Bulk read failed: {:?}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    info!("[MAIN] Starting application");
    let config = load_config("config/meter_config.yaml")?;
    let socket_addr: SocketAddr = format!("{}:{}", config.meter_data.ip, config.meter_data.port).parse()?;

    let service = Arc::new(ModbusRegisterService::new());

    let server_handle = tokio::spawn(async move {
        if let Err(e) = run_modbus_server(socket_addr).await {
            error!("Server error: {}", e);
        }
    });

    // Wait for the server to start
    sleep(Duration::from_secs(1)).await;

    // Initialize registers
    info!("[MAIN] Initializing registers");
    service.initialize_registers(&config);
    info!("[MAIN] Registers initialized");

    // Perform bulk read
    if let Err(e) = perform_bulk_read(socket_addr, &config).await {
        error!("[MAIN] Bulk read error: {}", e);
    }

    // Uncomment if you want to run tests
    // test_modbus_server(socket_addr, &config).await;

    // Keep the main task running
    server_handle.await?;

    info!("[MAIN] Application finished");
    Ok(())
}