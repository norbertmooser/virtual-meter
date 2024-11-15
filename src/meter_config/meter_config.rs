// meter_config.rs
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct MeterConfig {
    pub meter_data: MeterData,
    pub auth: Auth,
    pub write_registers: Vec<Register>,
    pub read_registers: Vec<Register>,
    pub debug: DebugConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MeterData {
    pub ip: String,
    pub port: u16,
    pub meter_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub name: String,
    pub register: u32,
    pub pin: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Register {
    pub name: String,
    pub address: u32,
    pub value: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DebugConfig {
    pub mgw_generic: String,
    pub statemachine_modbus: String,
    pub statemachine_read: String,
}

impl MeterConfig {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file_path)?;
        let config: MeterConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    // Function to display all read configurations
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("Meter Data:");
        println!("  IP: {}", self.meter_data.ip);
        println!("  Port: {}", self.meter_data.port);
        println!("  Meter Type: {}", self.meter_data.meter_type);
        
        println!("\nAuth:");
        println!("  Name: {}", self.auth.name);
        println!("  Register: {}", self.auth.register);
        println!("  PIN: {}", self.auth.pin);
        
        println!("\nWrite Registers:");
        for register in &self.write_registers {
            println!("  - Name: {}, Address: {}, Value: {}", register.name, register.address, register.value);
        }
        
        println!("\nRead Registers:");
        for register in &self.read_registers {
            println!("  - Name: {}, Address: {}, Value: {}", register.name, register.address, register.value);
        }
        
        println!("\nDebug Config:");
        println!("  MGW Generic: {}", self.debug.mgw_generic);
        println!("  State Machine Modbus: {}", self.debug.statemachine_modbus);
        println!("  State Machine Read: {}", self.debug.statemachine_read);
    }
}

