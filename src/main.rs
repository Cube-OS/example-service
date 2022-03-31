// This is the main.rs file for Services in the Cube-OS framework

// #[deny(missing_docs)]
// #[deny(warnings)]

pub mod service;
pub mod subsystem;
pub mod graphql;

// include API
use example_api::*;

use cubeos_service::{Config,Service};
// include output of macro in service.rs file
use crate::service::*;
use crate::subsystem::Subsystem;
use std::sync::{Arc};
use log::{error,info};
use failure::*;


fn main() -> ExampleResult<()> {

    let service_config = Config::new("example-service")
    .map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
    })
    .unwrap();

    let bus = service_config
    .get("bus")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();

    // Only needed for the debug feature
    #[cfg(feature = "debug")]
    let socket = service_config
    .get("udp_socket")
    .ok_or_else(|| {
        error!("Failed to load 'udp-socket' config value");
        format_err!("Failed to load 'udp-socket' config value");
    })
    .unwrap();

    #[cfg(feature = "debug")]
    let target = service_config
    .get("target")
    .ok_or_else(|| {
        error!("Failed to load 'target' config value");
        format_err!("Failed to load 'target' config value");
    })
    .unwrap();

    let bus = bus.as_str().unwrap();

    let subsystem: Box<Subsystem> = Box::new(
        match Subsystem::new(&bus)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            }) {
                Ok(b) => b,
                Err(e) => {
                    info!("Failed to create subsystem");
                    panic!("Subsystem creation failed: {:?}", e);
                }
            },
    );

    #[cfg(feature = "debug")]
    // Start debug service
    Service::new(
        service_config,
        QueryRoot,
        MutationRoot,
        socket.as_str().unwrap().to_string(),
        target.as_str().unwrap().to_string(),
    ).start();

    #[cfg(feature = "graphql")]
    // Start up graphql server
    Service::new(
        service_config,
        subsystem,
        QueryRoot,
        MutationRoot,
    )
    .start();

    #[cfg(not(any(feature = "debug",feature = "graphql")))]
    //Start up UDP server
    Service::new(
        service_config,
        subsystem,
        Some(Arc::new(udp_handler)),
    )
    .start();

    Ok(())
}