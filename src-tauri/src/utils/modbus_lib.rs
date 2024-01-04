use std::sync::{Arc, Mutex};
use tokio_modbus::prelude::*;
use lazy_static::lazy_static;
use tokio::time::{timeout, Duration};
use std::io::Error;
use tokio::runtime::Runtime;
use futures::executor::block_on;

pub struct ModbusConnection {
    pub connection: Option<tokio_modbus::client::Context>,
}

#[allow(dead_code)]
impl ModbusConnection {
    pub fn new() -> Self {
        ModbusConnection { connection: None }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub async fn connect_async(&mut self, ip: &str) -> Result<(), Error> {
        let socket_addr = ip.parse().unwrap();
        let timeout_duration = Duration::from_millis(300);

        let result = timeout(timeout_duration, tcp::connect(socket_addr)).await;

        match result {
            Ok(Ok(ctx)) => {
                self.connection = Some(ctx);
                Ok(())
            },
            Ok(Err(e)) => Err(e),
            Err(_) => Err(Error::new(std::io::ErrorKind::TimedOut, "timeout")),
        }
    }

    pub fn connect(&mut self, ip: &str, rt: &mut Runtime) {
        self.disconnect();
        rt.block_on(async {
            let result = self.connect_async(ip).await;

            match result {
                Ok(_) => println!("连接成功"),
                Err(_) => println!("连接超时"),
            }
        });
    }

    pub fn disconnect(&mut self) {
        // 判断连接是否存在，存在的还释放连接
        if !self.is_connected() {
            return;
        }
        std::mem::drop(self.connection.take());
        self.connection = None;
    }

    pub fn read_input_registers(&mut self, addr: u16, nb: u16) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
        match self.connection.as_mut() {
            Some(conn) => {
                let result = block_on(async {
                    conn.read_input_registers(addr, nb).await
                });
                match result {
                    Ok(data) => {
                        // println!("Data: {:?}", data);
                        Ok(data)
                    },
                    Err(e) => {
                        println!("Err: {:?}", e);
                        Err(Box::new(e))
                    },
                }
            },
            None => {
                println!("No connection available");
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "No connection available")))
            }
        }
    }
    
    pub fn read_registers(&mut self, addr: u16, nb: u16) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
        match self.connection.as_mut() {
            Some(conn) => {
                let result = block_on(async {
                    conn.read_holding_registers(addr, nb).await
                });
                match result {
                    Ok(data) => {
                        // println!("Data: {:?}", data);
                        Ok(data)
                    },
                    Err(e) => {
                        println!("Err: {:?}", e);
                        Err(Box::new(e))
                    },
                }
            },
            None => {
                println!("No connection available");
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotConnected, "No connection available")))
            }
        }
    }

    // pub fn write_registers(&mut self, addr: u16, src: &[u16]) {
    //     self.connection
    //         .as_mut()
    //         .unwrap()
    //         .write_multiple_registers(addr, src.to_vec())
    //         .unwrap();
    // }

    pub fn reconnect(&mut self, ip: &str, rt: &mut Runtime) {
        self.connect(ip, rt);
    }
}

lazy_static! {
    pub static ref MODBUS_CONN: Arc<Mutex<ModbusConnection>> =
        Arc::new(Mutex::new(ModbusConnection::new()));
}

pub fn get_modbus_conn() -> Arc<Mutex<ModbusConnection>> {
    MODBUS_CONN.clone()
}
