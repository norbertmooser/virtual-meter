use tokio_modbus::prelude::*;
use tokio_modbus::{Request, Response};
use std::{
    collections::HashMap,
    future,
    sync::{Arc, Mutex},
};
use log::{warn, info, debug};
use crate::config_load::MeterConfig; 

pub struct ModbusRegisterService {
    input_registers: Arc<Mutex<HashMap<u16, u16>>>,
    holding_registers: Arc<Mutex<HashMap<u16, u16>>>,
}

impl tokio_modbus::server::Service for ModbusRegisterService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = Exception;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let res = match req {
            Request::ReadInputRegisters(addr, cnt) => {
                self.bulk_read_registers(&self.input_registers, addr, cnt)
                    .map(Response::ReadInputRegisters)
            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                self.bulk_read_registers(&self.holding_registers, addr, cnt)
                    .map(Response::ReadHoldingRegisters)
            }
            Request::WriteMultipleRegisters(addr, values) => {
                register_write(&mut self.holding_registers.lock().unwrap(), addr, &values)
                    .map(|_| Response::WriteMultipleRegisters(addr, values.len() as u16))
            }
            Request::WriteSingleRegister(addr, value) => register_write(
                &mut self.holding_registers.lock().unwrap(),
                addr,
                std::slice::from_ref(&value),
            )
            .map(|_| Response::WriteSingleRegister(addr, value)),
            _ => {
                warn!("[SERVICE] Exception::IllegalFunction - Unimplemented function code in request: {req:?}");
                Err(Exception::IllegalFunction)
            }
        };
        future::ready(res)
    }
}

impl ModbusRegisterService {
    pub fn new() -> Self {
        Self {
            input_registers: Arc::new(Mutex::new(HashMap::new())),
            holding_registers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn initialize_registers(&self, config: &MeterConfig) {
        let mut input_registers = self.input_registers.lock().unwrap();
        let mut holding_registers = self.holding_registers.lock().unwrap();

        input_registers.clear();
        holding_registers.clear();

        for register in &config.read_registers {
            if register.is_input {
                input_registers.insert(register.address, register.value as u16);
                debug!("[SERVICE] Initialized input register: {} (address: {}, value: {})", 
                       register.name, register.address, register.value);
            } else {
                holding_registers.insert(register.address, register.value as u16);
                debug!("[SERVICE] Initialized holding register: {} (address: {}, value: {})", 
                       register.name, register.address, register.value);
            }
        }

        for register in &config.write_registers {
            holding_registers.insert(register.address, register.value as u16);
            debug!("[SERVICE] Initialized holding register: {} (address: {}, value: {})", 
                   register.name, register.address, register.value);
        }

        holding_registers.insert(config.auth.register, config.auth.pin);
        debug!("[SERVICE] Initialized auth register: address: {}, value: {}", 
               config.auth.register, config.auth.pin);

        info!("[SERVICE] Initialized {} input registers and {} holding registers", 
              input_registers.len(), holding_registers.len());
    }

    fn bulk_read_registers(&self, registers: &Arc<Mutex<HashMap<u16, u16>>>, start_address: u16, quantity: u16) -> Result<Vec<u16>, Exception> {
        if quantity == 0 || quantity > 125 {
            warn!("[SERVICE] Exception::IllegalDataValue - Invalid quantity: {}", quantity);
            return Err(Exception::IllegalDataValue);
        }

        let registers = registers.lock().map_err(|_| {
            warn!("[SERVICE] Exception::SlaveDeviceFailure - Failed to lock mutex");
            Exception::ServerDeviceFailure
        })?;

        let mut response = Vec::with_capacity(quantity as usize);

        for addr in start_address..start_address + quantity {
            match registers.get(&addr) {
                Some(&value) => response.push(value),
                None => {
                    warn!("[SERVICE] Exception::IllegalDataAddress - Address not found: {}", addr);
                    return Err(Exception::IllegalDataAddress);
                }
            }
        }

        Ok(response)
    }

    // Remove the async bulk_read method as it's no longer needed
}

/// Helper function implementing reading registers from a HashMap.
fn register_read(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, Exception> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = registers.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            warn!("[SERVICE] Exception::IllegalDataAddress");
            return Err(Exception::IllegalDataAddress);
        }
    }

    Ok(response_values)
}

/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.
fn register_write(
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: &[u16],
) -> Result<(), Exception> {
    for (i, value) in values.iter().enumerate() {
        let reg_addr = addr + i as u16;
        if let Some(r) = registers.get_mut(&reg_addr) {
            *r = *value;
        } else {
            warn!("[SERVICE] Exception::IllegalDataAddress");
            return Err(Exception::IllegalDataAddress);
        }
    }

    Ok(())
}
