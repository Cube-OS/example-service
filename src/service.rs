// The service.rs file is the core of each service
// It enables the communication via UDP or GraphQL (depending on --features flag during compilation)

use cubeos_service::*;
use example_api::*;
use crate::objects::*;

#[cfg(not(feature = "ground"))]
use crate::subsystem::*;

#[cfg(any(feature = "graphql", feature = "ground"))]
use crate::graphql::*;

// Macro to create UDP-handler function or GraphQL Queries and Mutations
// The layout follows the rules:
// query/mutation: Command-Name => Function as defined in subsystem.rs; in: GraphQLInput; out: GraphQLOutput;
// 
// GraphQLInput is only needed if Input is a struct that contains fields with types other than i32,f64,String or bool
// GraphQLOutput is only needed if the Output should be formatted in humanly readable way 
// (e.g. Payload telemetry returns a Vec<u8>, but resembles analog data like Voltage,Current,Temperature etc.)
// If GraphQLInput/Output are not needed then please set to Input and Output of function
service_macro!{
    query: Ping => fn ping(&self) -> Result<()>;
    query: Get => fn get_values(&self, get: ExampleEnum) -> Result<ExampleOutput>; in: ExampleEnum; out: GqlExampleOutput;
    query: GetI2c => fn get_i2c(&self) -> Result<Vec<u8>>; out: Vec<u8>;
    query: GetUart => fn get_uart(&self) -> Result<u8>; out: u8;
    mutation: Set => fn set_values(&self, sub: ExampleInput, choice: ExampleEnum) -> Result<()>; in: GqlExampleInput, ExampleEnum;
    mutation: SetI2c => fn set_i2c(&self, input: u8) -> Result<()>; in: i32;
    mutation: SetUart => fn set_uart(&self, input: u8) -> Result<()>; in: i32;
}