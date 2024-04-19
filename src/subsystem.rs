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

use cubeos_service::*;
use dandelions_api::*;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use log::{error, info};


#[derive(Clone)]
pub struct Subsystem {
    dandelions: Arc<Mutex<DandelionsStruct>>,
    // pub last_cmd: Arc<RwLock<Vec<u8>>>,
    // pub last_err: Arc<RwLock<Error>>,
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
    /// * `DandelionsResult<Self>` - Returns `Self` or DandelionsError.
    pub fn new(
        uart_path: String,
        uart_setting: serial::PortSettings,
        uart_timeout: Duration,
    ) -> DandelionsResult<Self> {
        Ok(Self {
            dandelions: Arc::new(Mutex::new(DandelionsStruct::new(
                uart_path,
                uart_setting,
                uart_timeout,
            )?)),
        })
    }

    /// These functions are Dandelionss how to use the underlying GET and SET functions
    /// 
    pub fn get_uart(&self) -> Result<()> {
        Ok(self.dandelions.lock().unwrap().get_uart()?)
    }

    pub fn set_uart(&self) -> Result<()> {
        Ok(self.dandelions.lock().unwrap().set_uart()?)
    }
}
