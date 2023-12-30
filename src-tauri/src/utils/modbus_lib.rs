use std::sync::{Arc, Mutex};
use tokio_modbus::prelude::*;
use lazy_static::lazy_static;

pub struct ModbusConnection {
    pub connection: Option<client::sync::Context>,
}

#[allow(dead_code)]
impl ModbusConnection {
    pub fn new() -> Self {
        ModbusConnection { connection: None }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn connect(&mut self, ip: &str) {
        self.disconnect();
        let socket_addr = ip.parse().unwrap();
        match client::sync::tcp::connect(socket_addr) {
            Ok(ctx) => {
                self.connection = Some(ctx);
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                // You can return from the function or continue to the next iteration of a loop here.
            }
        }
    }

    pub fn disconnect(&mut self) {
        // 判断连接是否存在，存在的还释放连接
        if !self.is_connected() {
            return;
        }
        std::mem::drop(self.connection.take());
        self.connection = None;
    }

    pub fn read_input_registers(&mut self, addr: u16, nb: u16) -> Vec<u16> {
        match self.connection.as_mut() {
            Some(conn) => {
                match conn.read_input_registers(addr, nb) {
                    Ok(data) => return data,
                    Err(e) => println!("Error reading input registers: {}", e),
                }
            },
            None => println!("No connection available"),
        }
        vec![]
    }
    
    pub fn read_registers(&mut self, addr: u16, nb: u16) -> Vec<u16> {
        match self.connection.as_mut() {
            Some(conn) => {
                match conn.read_holding_registers(addr, nb) {
                    Ok(data) => return data,
                    Err(e) => println!("Error reading holding registers: {}", e),
                }
            },
            None => println!("No connection available"),
        }
        vec![]
    }

    // pub fn write_registers(&mut self, addr: u16, src: &[u16]) {
    //     self.connection
    //         .as_mut()
    //         .unwrap()
    //         .write_multiple_registers(addr, src.to_vec())
    //         .unwrap();
    // }

    pub fn reconnect(&mut self, ip: &str) {
        self.disconnect();
        self.connect(ip);
    }
}

lazy_static! {
    pub static ref MODBUS_CONN: Arc<Mutex<ModbusConnection>> =
        Arc::new(Mutex::new(ModbusConnection::new()));
}

pub fn get_modbus_conn() -> Arc<Mutex<ModbusConnection>> {
    MODBUS_CONN.clone()
}
