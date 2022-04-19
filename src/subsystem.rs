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
use cubeos_error::*;
use std::sync::{Arc,Mutex};
use std::convert::From;
use crate::objects::*;

#[derive(Clone)]
pub struct Subsystem {
    substruct: Arc<Mutex<ExampleStruct>>,
}
impl Subsystem {
    pub fn new(_bus: &str) -> ExampleResult<Self> {
        Ok(Self {
            substruct: Arc::new(Mutex::new(ExampleStruct::new())),
        })
    }

    pub fn ping(&self, _g: Generic) -> Result<GenericResponse> {
        Ok(GenericResponse::new())
    }

    pub fn get(&self, get: ExampleEnum) -> Result<ExampleOutput> {
        match self.substruct.lock().unwrap().get(get) {
            Ok(x) => Ok(x),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn set(&self, set: ExampleObject) -> Result<GenericResponse> {
        println!("Set");
        match self.substruct.lock().unwrap().set(set.sub,set.choice) {
            Ok(()) => Ok(GenericResponse::new()),
            Err(e) => Err(Error::from(e)),
        }
        
    }
}