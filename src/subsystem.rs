//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
//
// In this file the subsystem that contains all the functions to interact with the API is defined
//
// Comments generated in parts with GPT-3 (see disclaimer in README)

use cubeos_error::{Error, Result};
use cubeos_service::*;
use example_api::*;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

#[derive(Clone)]
pub struct Subsystem {
    example: Arc<Mutex<ExampleStruct>>,
    pub last_cmd: Arc<RwLock<Vec<u8>>>,
    pub last_err: Arc<RwLock<Error>>,
}
impl Subsystem {
    /// Initialisation of the Subsystem
    ///
    /// # Arguments
    ///
    /// * `i2c_path` - A string that represents the path to the i2c device.
    /// * `i2c_addr` - A u16 that represents the address of the i2c device.
    /// * `uart_path` - A string that represents the path to the uart device.
    /// * `uart_setting` - A serial::PortSettings that represents the settings of the uart device.
    /// * `uart_timeout` - A Duration that represents the timeout of the uart device.
    ///
    /// # Output
    ///
    /// * `ExampleResult<Self>` - Returns `Self` or ExampleError.
    pub fn new(
        i2c_path: String,
        i2c_addr: u16,
        uart_path: String,
        uart_setting: serial::PortSettings,
        uart_timeout: Duration,
        udp_path: String,
        udp_to: String,
    ) -> ExampleResult<Self> {
        Ok(Self {
            example: Arc::new(Mutex::new(ExampleStruct::new(
                i2c_path,
                i2c_addr,
                uart_path,
                uart_setting,
                uart_timeout,
                udp_path,
                udp_to,
            )?)),
            last_cmd: Arc::new(RwLock::new(Vec::new())),
            last_err: Arc::new(RwLock::new(Error::None)),
        })
    }

    /// This function is used to get values from the underlying API's struct.
    ///
    /// # Arguments
    ///
    /// * `get` - Enum that specifies which values to get.
    ///
    /// # Output
    ///
    /// * `Result<ExampleOutput>` - Returns Struct containing the requested values or ExampleError
    pub fn get_values(&self, get: ExampleEnum) -> Result<ExampleOutput> {
        Ok(self.example.lock().unwrap().get_values(get)?)
    }

    /// This function is used to set values of the underlying API's struct.
    ///
    /// # Arguments
    ///
    /// * `sub` - Struct containing the values to set.
    /// * `choice` - Enum that specifies which values to set.
    ///
    /// # Output
    ///
    /// * `Result<()>` - Returns () if successful, ExampleError otherwise
    ///
    pub fn set_values(&self, sub: ExampleInput, choice: ExampleEnum) -> Result<()> {
        Ok(self.example.lock().unwrap().set_values(sub, choice)?)
    }

    /// These functions are examples how to use the underlying GET and SET functions
    /// for I2C, UART and UDP payloads
    ///
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
        Ok(self.example.lock().unwrap().set_uart(input)?)
    }

    pub fn get_udp(&self, command: Vec<u8>, rx_len: usize) -> Result<Vec<u8>> {
        Ok(self.example.lock().unwrap().get_udp(command, rx_len)?)
    }

    pub fn set_udp(&self, input: Vec<u8>) -> Result<()> {
        Ok(self.example.lock().unwrap().set_udp(input)?)
    }
}
