use std::net::SocketAddr;
use std::time::Duration;
use tokio_modbus::prelude::*;
use log::{info, error, debug, warn};
use crate::config_load::MeterConfig;

pub async fn test_modbus_server(socket_addr: SocketAddr, config: &MeterConfig) {
    tokio::join!(
        async {
            // Give the server some time for starting up
            tokio::time::sleep(Duration::from_secs(1)).await;

            info!("[TEST   ] Connecting client...");
            let mut ctx = match tcp::connect(socket_addr).await {
                Ok(ctx) => ctx,
                Err(e) => {
                    error!("[TEST   ] Failed to connect to server: {}", e);
                    return;
                }
            };

            // Read registers
            for register in &config.read_registers {
                debug!("[TEST   ] Reading register {} ({})...", register.name, register.address);
                let result = if register.is_input {
                    ctx.read_input_registers(register.address, 1).await
                } else {
                    ctx.read_holding_registers(register.address, 1).await
                };

                match result {
                    Ok(response) => {
                        match response {
                            Ok(values) => {
                                if let Some(value) = values.get(0) {
                                    info!("[TEST   ] {} = {}", register.name, value);
                                } else {
                                    warn!("[TEST   ] No value returned for {}", register.name);
                                }
                            },
                            Err(e) => warn!("[TEST   ] Modbus exception for {}: {:?}", register.name, e),
                        }
                    },
                    Err(e) => error!("[TEST   ] Failed to read {}: {}", register.name, e),
                }
            }

            // Write registers
            for register in &config.write_registers {
                debug!("[TEST   ] Writing register {} ({}) = {}...", register.name, register.address, register.value);
                match ctx.write_single_register(register.address, register.value as u16).await {
                    Ok(_) => info!("[TEST   ] Successfully wrote {} = {}", register.name, register.value),
                    Err(e) => error!("[TEST   ] Failed to write {}: {}", register.name, e),
                }
            }

            // Read back written registers
            for register in &config.write_registers {
                debug!("[TEST   ] Reading back written register {} ({})...", register.name, register.address);
                match ctx.read_holding_registers(register.address, 1).await {
                    Ok(response) => {
                        match response {
                            Ok(values) => {
                                if let Some(value) = values.get(0) {
                                    info!("[TEST   ] {} = {}", register.name, value);
                                } else {
                                    warn!("[TEST   ] No value returned for {}", register.name);
                                }
                            },
                            Err(e) => warn!("[TEST   ] Modbus exception for {}: {:?}", register.name, e),
                        }
                    },
                    Err(e) => error!("[TEST   ] Failed to read back {}: {}", register.name, e),
                }
            }

            info!("[TEST   ] All tests completed.");
        },
        tokio::time::sleep(Duration::from_secs(5))
    );
}
