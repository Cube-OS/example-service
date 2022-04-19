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
// generic/query/mutation: Command-Name => Function as defined in subsystem.rs; (GraphQLInput,GraphQLOutput)
// 
// GraphQLInput is only needed if Input is a struct that contains fields with types other than i32,f64,String or bool
// GraphQLOutput is only needed if the Output should be formatted in humanly readable way 
// (e.g. Payload telemetry returns a Vec<u8>, but resembles analog data like Voltage,Current,Temperature etc.)
// If GraphQLInput/Output are not needed then please set to Input and Output of function
service_macro!{
    generic: Ping => fn ping(&self,_g: Generic) -> Result<GenericResponse>; (Generic, GenericResponse),
    query: Get => fn get(&self, get: ExampleEnum) -> Result<ExampleOutput>; (ExampleEnum, GqlExampleOutput),
    mutation: Set => fn set(&self, set: ExampleObject) -> Result<GenericResponse>; (GqlExampleInput, GenericResponse),
}