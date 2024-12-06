// SPDX-FileCopyrightText: Copyright (c) 2017-2024 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # TCP server example
//!
//! This example shows how to start a server and implement basic register
//! read/write operations.

use crate::server::service_init::{RegisterData, initialize_registers};
use crate::meter_config::MeterConfig; // Import MeterConfig

use std::{
    collections::HashMap,
    future,
    net::SocketAddr,
};

use tokio::net::TcpListener;
use tokio_modbus::{
    prelude::*,
    server::tcp::{accept_tcp_connection, Server},
    ExceptionCode,
};



struct ModbusTcpService {
    register_data: RegisterData,
}

impl tokio_modbus::server::Service for ModbusTcpService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let res = match req {
            Request::ReadInputRegisters(addr, cnt) => {
                register_read_with_default(&self.register_data.input_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadInputRegisters)
            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                register_read_with_default(&self.register_data.holding_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadHoldingRegisters)
            }
            Request::WriteMultipleRegisters(addr, values) => {
                register_write(&mut self.register_data.holding_registers.lock().unwrap(), addr, &values)
                    .map(|_| Response::WriteMultipleRegisters(addr, values.len() as u16))
            }
            Request::WriteSingleRegister(addr, value) => {
                register_write(
                    &mut self.register_data.holding_registers.lock().unwrap(),
                    addr,
                    std::slice::from_ref(&value),
                )
                .map(|_| Response::WriteSingleRegister(addr, value))
            }
            _ => {
                println!("SERVER: Exception::IllegalFunction - Unimplemented function code in request: {req:?}");
                Err(ExceptionCode::IllegalFunction)
            }
        };
        future::ready(res)
    }
}

impl ModbusTcpService {
    fn new(config: &MeterConfig) -> Self {
        println!("Initializing ModbusTcpService with provided MeterConfig."); // Debug printout for initialization start

        let mut service = Self {
            register_data: initialize_registers(),
        };

        println!("Register data initialized."); // Debug printout after initializing register_data

        initialize_read_registers(&mut service.register_data, config);
        println!("Read registers initialized with config: {:?}", config); // Debug printout after initializing read registers

        service
    }
}

/// Helper function implementing reading registers from a HashMap.
#[allow(dead_code)]
fn register_read(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, ExceptionCode> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = registers.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            println!("SERVER: Exception::IllegalDataAddress: Read");
            return Err(ExceptionCode::IllegalDataAddress);
        }
    }
    Ok(response_values)
}

/// Helper function implementing reading registers from a HashMap.
/// Returns zero for uninitialized registers instead of throwing an exception.
#[allow(dead_code)]
fn register_read_with_default(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, ExceptionCode> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        // If the register is found, use its value; otherwise, default to 0
        response_values[i as usize] = *registers.get(&reg_addr).unwrap_or(&0);
    }
    Ok(response_values)
}



/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.
fn register_write(
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: &[u16],
) -> Result<(), ExceptionCode> {
    for (i, value) in values.iter().enumerate() {
        let reg_addr = addr + i as u16;
        if let Some(r) = registers.get_mut(&reg_addr) {
            *r = *value;
        } else {
            println!("SERVER: Exception::IllegalDataAddress: Write");
            return Err(ExceptionCode::IllegalDataAddress);
        }
    }
    Ok(())
}

pub fn initialize_read_registers(register_data: &mut RegisterData, config: &MeterConfig) {
    for register in &config.read_registers {
        register_data.holding_registers.lock().unwrap().insert(register.address as u16, register.value as u16);
        println!("Inserted register: address = {}, value = {}", register.address, register.value); // Printout for each inserted register
    }
}

pub async fn server_context(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = MeterConfig::from_file(config_path)?; // Load the config
    
    // Create SocketAddr from config values
    let socket_addr: SocketAddr = format!("{}:{}", config.meter_data.ip, config.meter_data.port)
        .parse()
        .expect("Failed to parse socket address from config");
    
    println!("Starting up server on {socket_addr}");
    let listener: TcpListener = TcpListener::bind(socket_addr).await?;
    let server: Server = Server::new(listener);
    
    let new_service = move |_socket_addr| {
        let service: ModbusTcpService = ModbusTcpService::new(&config);
        Ok(Some(service))
    };
    
    let on_connected = |stream, socket_addr| {
        let new_service = new_service.clone();
        async move {
            accept_tcp_connection(stream, socket_addr, new_service)
        }
    };
    
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
