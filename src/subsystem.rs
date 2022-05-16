// In this file the subsystem that contains all the functions to interact with the API is defined
// 
// IMPORTANT: Any function here has EXACTLY 1 INPUT
// 
// The Generic type is used for functions that usually wouldn't need an input
// 
// For any function with more than 1 input, those inputs need to be put in a struct 
// and the struct is used as the 1 input (see ExampleInput)

use example_api::*;
use cubeos_service::*;
use cubeos_error::{Error,Result};
use std::sync::{Arc,Mutex,RwLock};
use std::convert::From;
use std::time::Duration;
use crate::service::*;

#[derive(Clone)]
pub struct Subsystem {
    example: Arc<Mutex<ExampleStruct>>,
    pub last_cmd: Arc<RwLock<Vec<u8>>>,
    pub last_err: Arc<RwLock<Error>>,
}
impl Subsystem {
    pub fn new(
        i2c_path: String,
        i2c_addr: u16,
        uart_path: String,
        uart_setting: serial::PortSettings,
        uart_timeout: Duration,
    ) -> ExampleResult<Self> {
        Ok(Self {
            example: Arc::new(Mutex::new(ExampleStruct::new(i2c_path,i2c_addr,uart_path,uart_setting,uart_timeout)?)),
            last_cmd: Arc::new(RwLock::new(Vec::new())),
            last_err: Arc::new(RwLock::new(Error::None)),
        })
    }

    pub fn ping(&self) -> Result<()> {
        Ok(())
    }

    pub fn get_values(&self, get: ExampleEnum) -> Result<ExampleOutput> {
        Ok(self.example.lock().unwrap().get_values(get)?)
    }

    pub fn set_values(&self, sub: ExampleInput, choice: ExampleEnum) -> Result<()> {
        Ok(self.example.lock().unwrap().set_values(sub,choice)?)
    }

    pub fn get_i2c(&self) -> Result<Vec<u8>> {
        Ok(self.example.lock().unwrap().get_i2c()?)
    }

    pub fn set_i2c(&self, input: u8) -> Result<()> {
        Ok(self.example.lock().unwrap().set_i2c(input)?)
    }

    pub fn get_uart(&self) -> Result<Vec<u8>> {
        Ok(self.example.lock().unwrap().get_uart()?)
    }

    pub fn set_uart(&self, input: u8) -> Result<()> {
        Ok(self.example.lock().unwrap().set_i2c(input)?)
    }
}