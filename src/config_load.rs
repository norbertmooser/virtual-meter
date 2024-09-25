use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeterConfig {
    pub meter_data: MeterData,
    pub auth: Auth,
    pub write_registers: Vec<Register>,
    pub read_registers: Vec<Register>,
    pub debug: Debug,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeterData {
    pub ip: String,
    pub port: u16,
    pub meter_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auth {
    pub name: String,
    pub register: u16,
    pub pin: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Register {
    pub name: String,
    pub address: u16,
    pub value: i32,
    pub is_input: bool,  
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Debug {
    pub mgw_generic: String,
    pub statemachine_modbus: String,
    pub statemachine_read: String,
}

pub fn load_config(path: &str) -> Result<MeterConfig> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open config file: {}", path))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    
    let config: MeterConfig = serde_yaml::from_str(&contents)
        .with_context(|| format!("Failed to parse YAML in config file: {}", path))?;
    
    Ok(config)
}
