// This is an example for the implementation of GraphQL structs
// to use in the Cube-OS framework
// 
// GraphQL only allows four different types:
// i32,f64,String and bool
// 
// Therefore it is often necessary and desired (to reduce a data overhead over UDP)
// to translate from those types to types in the corresponding API or input struct 

use example_api::*;
use cubeos_error::{Result as CubeOSResult,Error as CubeOSError};
use std::convert::TryFrom;
use juniper::*;
use serde::*;

#[derive(GraphQLInputObject)]
pub struct GqlExampleInput {
    gql_ex_no: i32,
    gql_ex_str: String,
    gql_ex_bool: bool,
}
// Translation from GraphQLInput Type to normal types
impl TryFrom<GqlExampleInput> for ExampleInput {
    type Error = CubeOSError;

    fn try_from(e: GqlExampleInput) -> CubeOSResult<ExampleInput> {
        Ok(ExampleInput{
            in_no: e.gql_ex_no as u16,
            in_str: e.gql_ex_str,
            in_bool: e.gql_ex_bool,
        })
    }
}

#[derive(Serialize,Deserialize)]
pub struct GqlExampleOutput {
    gql_out_no: Option<f64>,
    gql_out_str: Option<String>,
    gql_out_bool: Option<bool>,
}
// Translation from Output to GraphQLOutput
impl From<ExampleOutput> for GqlExampleOutput {
    fn from(o: ExampleOutput) -> GqlExampleOutput {
        GqlExampleOutput{
            gql_out_no: Some(o.out_no.unwrap() as f64 * 1.4),
            gql_out_str: o.out_str,
            gql_out_bool: o.out_bool,
        }
    }
}