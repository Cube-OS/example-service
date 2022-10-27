// This is an example for the implementation of GraphQL structs
// to use in the Cube-OS framework
//
// GraphQL only allows four different types:
// i32,f64,String and bool
//
// Therefore it is often necessary and desired (to reduce a data overhead over UDP)
// to translate from those types to types in the corresponding API or input struct

use cubeos_error::{Error as CubeOSError, Result as CubeOSResult};
use example_api::*;
use cubeos_service::juniper as juniper;
use juniper::*;
use serde::*;
use std::convert::TryFrom;

// Example if Input was a struct
#[derive(GraphQLInputObject, Clone)]
pub struct GqlExampleInput {
    gql_ex_no: i32,
    gql_ex_no1: i32,
    gql_ex_no2: i32,
    gql_ex_str: String,
    gql_ex_bool: bool,
    // gql_choice: ExampleEnum,
}
// Translation from GraphQLInput Type to normal types
impl TryFrom<GqlExampleInput> for ExampleInput {
    type Error = CubeOSError;

    fn try_from(e: GqlExampleInput) -> CubeOSResult<ExampleInput> {
        Ok(ExampleInput {
            in_no: e.gql_ex_no as u16,
            in_no1: e.gql_ex_no1 as u32,
            in_no2: e.gql_ex_no2 as u16,
            in_str: e.gql_ex_str,
            in_bool: e.gql_ex_bool,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct GqlExampleOutput {
    gql_out_no: Vec<f64>,
    gql_out_str: String,
    gql_out_bool: Vec<bool>,
}
// Translation from Output to GraphQLOutput
impl From<ExampleOutput> for GqlExampleOutput {
    fn from(o: ExampleOutput) -> GqlExampleOutput {
        GqlExampleOutput {
            gql_out_no: {
                let mut vec: Vec<f64> = Vec::new();
                for i in 0..o.out_no.len() {
                    vec.push(o.out_no[i] as f64 * 1.4);
                }
                vec
            },
            gql_out_str: o.out_str,
            gql_out_bool: o.out_bool,
        }
    }
}
