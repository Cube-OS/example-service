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
// The service.rs file is the core of each service
// It enables the communication via UDP or GraphQL (depending on --features flag during compilation)

use cubeos_service::*;
use example_api::*;

// #[cfg(not(feature = "ground"))]
// use crate::subsystem::*;

#[cfg(feature = "ground")]
use crate::graphql::*;

// Macro to create UDP-handler function or GraphQL Queries and Mutations
// The layout follows the rules:
// query/mutation: Command-Name => Function as defined in subsystem.rs; in: GraphQLInput; out: GraphQLOutput;
//
// GraphQLInput is only needed if Input is a struct that contains fields with types other than i32,f64,String or bool
// GraphQLOutput is only needed if the Output should be formatted in humanly readable way
// (e.g. Payload telemetry returns a Vec<u8>, but resembles analog data like Voltage,Current,Temperature etc.)
// If GraphQLInput/Output are not needed then please set to Input and Output of function
service_macro! {
    subsystem::Subsystem{ 
        // Note that Ping, GetLastError, GetLastMutation are already inculded in the CubeOS-Service           
        query: Get => fn get_values(&self, get: ExampleEnum) -> Result<ExampleOutput>; in: ExampleEnum; out: GqlExampleOutput;
        query: GetI2c => fn get_i2c(&self) -> Result<Vec<u8>>; out: Vec<u8>;
        query: GetUart => fn get_uart(&self) -> Result<Vec<u8>>; out: Vec<u8>;
        // query: GetUdp => fn get_udp(&self, command: Vec<u8>, rx_len: usize) -> Result<Vec<u8>>; in: Vec<u8>, usize; out: Vec<u8>;
        mutation: Set => fn set_values(&self, sub: ExampleInput, choice: ExampleEnum) -> Result<()>; in: GqlExampleInput, ExampleEnum;
        mutation: SetI2c => fn set_i2c(&self, input: u8) -> Result<()>; in: i32;
        mutation: SetUart => fn set_uart(&self, input: u8) -> Result<()>; in: i32;
        // mutation: SetUdp => fn set_udp(&self, input: Vec<u8>) -> Result<()>; in: Vec<u8>;
    }
}
