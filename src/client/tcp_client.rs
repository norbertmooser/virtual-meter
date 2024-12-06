use std::net::SocketAddr;
use tokio_modbus::prelude::*;
use std::time::Duration;
use crate::meter_config::MeterConfig;

pub async fn client_context(socket_addr: SocketAddr, config_path: &str) {
    
    bulk_read_configured_registers(socket_addr, &config_path).await;


    // Call the test routine
    // run_client_tests(socket_addr).await;

    println!("CLIENT: Done.");
}

async fn bulk_read_configured_registers(socket_addr: SocketAddr, config_path: &str) {
    let config = MeterConfig::from_file(config_path).expect("Failed to load config");
    tokio::join!(
        async {
            // Give the server some time for starting up
            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("CLIENT: Connecting client...");
            let mut ctx = tcp::connect(socket_addr).await.unwrap();

            // Collect all register addresses to read
            let addresses: Vec<u16> = config.read_registers.iter()
                .map(|register| register.address as u16)
                .collect();

            // Find the minimum and maximum register addresses
            let min_address = *addresses.iter().min().expect("No registers found");
            let max_address = *addresses.iter().max().expect("No registers found");

            // Read all registers in bulk from min to max
            let response = ctx.read_holding_registers(min_address, (max_address - min_address + 1) as u16).await.unwrap();
            match response {
                Ok(values) => {
                    for (i, register) in config.read_registers.iter().enumerate() {
                        if addresses[i] >= min_address && addresses[i] <= max_address {
                            println!("CLIENT: The value of '{}' is: {}", register.name, values[(addresses[i] - min_address) as usize]);
                        }
                    }
                }
                Err(err) => {
                    println!("CLIENT: Error reading registers: {:?}", err);
                }
            }

            println!("CLIENT: Done.")
        },
        tokio::time::sleep(Duration::from_secs(5))
    );
}





#[allow(dead_code)]
async fn read_configured_registers(socket_addr: SocketAddr, config_path: &str) {
    let config = MeterConfig::from_file(config_path).expect("Failed to load config");
    tokio::join!(
        async {
            // Give the server some time for starting up
            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("CLIENT: Connecting client...");
            let mut ctx = tcp::connect(socket_addr).await.unwrap();

            // Read all read_registers one by one
            for register in &config.read_registers {
                println!("CLIENT: Reading register '{}' at address {}...", register.name, register.address);
                let response = ctx.read_holding_registers(register.address as u16, 1).await.unwrap();
                match response {
                    Ok(values) => {
                        println!("CLIENT: The value of '{}' is: {}", register.name, values[0]);
                    }
                    Err(err) => {
                        println!("CLIENT: Error reading register '{}': {:?}", register.name, err);
                    }
                }
            }

            println!("CLIENT: Done.")
        },
        tokio::time::sleep(Duration::from_secs(5))
    );
}






#[allow(dead_code)]
async fn run_client_tests(socket_addr: SocketAddr) {
    // Give the server some time for starting up
    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("CLIENT: Connecting client...");
    let mut ctx = tcp::connect(socket_addr).await.unwrap();

    println!("CLIENT: Reading 2 input registers...");
    let response = ctx.read_input_registers(0x00, 2).await.unwrap();
    println!("CLIENT: The result is '{response:?}'");
    assert_eq!(response.unwrap(), vec![1234, 5678]);

    println!("CLIENT: Writing 2 holding registers...");
    ctx.write_multiple_registers(0x01, &[7777, 8888])
        .await
        .unwrap()
        .unwrap();

    // Read back a block including the two registers we wrote.
    println!("CLIENT: Reading 4 holding registers...");
    let response = ctx.read_holding_registers(0x00, 4).await.unwrap();
    println!("CLIENT: The result is '{response:?}'");
    assert_eq!(response.unwrap(), vec![10, 7777, 8888, 40]);

    // Now we try to read with an invalid register address.
    // This should return a Modbus exception response with the code
    // IllegalDataAddress.
    println!("CLIENT: Reading nonexistent holding register address... (should return IllegalDataAddress)");
    let response = ctx.read_holding_registers(0x100, 1).await.unwrap();
    println!("CLIENT: The result is '{response:?}'");
    assert!(matches!(response, Err(ExceptionCode::IllegalDataAddress)));
}

