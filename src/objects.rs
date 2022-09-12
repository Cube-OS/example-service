// Example for input objects that are defined on Service level
use example_api::*;
use cubeos_service::serde::*;
use std::mem::size_of;

#[derive(Serialize, Deserialize)]
pub struct ExampleObject {
    pub sub: ExampleInput,
    pub choice: ExampleEnum,
}
impl ExampleObject {
    pub fn print_length(&self) {
        println!("{:?}", size_of::<ExampleInput>());
        println!("{:?}", size_of::<ExampleEnum>());
    }
}
