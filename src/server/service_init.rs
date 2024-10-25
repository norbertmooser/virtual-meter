use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RegisterData {
    pub input_registers: Arc<Mutex<HashMap<u16, u16>>>,
    pub holding_registers: Arc<Mutex<HashMap<u16, u16>>>,
}

pub fn initialize_registers() -> RegisterData {
    let mut input_registers = HashMap::new();
    input_registers.insert(0, 1234);
    input_registers.insert(1, 5678);

    let mut holding_registers = HashMap::new();
    holding_registers.insert(0, 10);
    holding_registers.insert(1, 20);
    holding_registers.insert(2, 30);
    holding_registers.insert(3, 40);

    RegisterData {
        input_registers: Arc::new(Mutex::new(input_registers)),
        holding_registers: Arc::new(Mutex::new(holding_registers)),
    }
}

