// This is the main.rs file for Services in the Cube-OS framework

// #[deny(missing_docs)]
// #[deny(warnings)]

pub mod service;
pub mod subsystem;
pub mod graphql;
pub mod objects;

// include API
use example_api::*;

use cubeos_service::{Config,Service};
// include output of macro in service.rs file
use crate::service::*;
use crate::subsystem::Subsystem;
// use crate::service::udp_handler;
use std::sync::{Arc};
use log::{error,info};
use failure::*;
use std::time::Duration;
use std::str::FromStr;
use serial::*;


fn main() -> ExampleResult<()> {

    let service_config = Config::new("example-service")
    .map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
    })
    .unwrap();

    let i2c_bus = service_config
    .get("i2c_bus")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();

    let i2c_bus = i2c_bus.as_str().unwrap().to_string();

    // Alternatively the I2C address can be hardcoded here
    let i2c_addr = service_config
    .get("i2c_addr")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();
    let i2c_addr = i2c_addr.as_str().unwrap();
    let i2c_addr: u16 = if i2c_addr.starts_with("0x") {
        u16::from_str_radix(&i2c_addr[2..], 16).unwrap()
    } else {
        u16::from_str_radix(i2c_addr, 16).unwrap()
    };

    let uart_bus = service_config
    .get("uart_bus")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap()
    .as_str()
    .unwrap()
    .to_string();

    // UART Settings can be loaded from the service config file
    // alternatively they can be hardcoded, any change at run time then requires functions in the service
    // let uart_setting = service_config
    // .get("uart_setting")
    // .ok_or_else(|| {
    //     error!("Failed to load 'bus' config value");
    //     format_err!("Failed to load 'bus' config value");
    // })
    // .unwrap();
    let uart_setting = serial::PortSettings{
        baud_rate: Baud9600,
        char_size: Bits8,
        parity: ParityNone,
        stop_bits: Stop1,
        flow_control: FlowNone,
    };

    let uart_timeout = service_config
    .get("uart_timeout")
    .ok_or_else(|| {
        error!("Failed to load 'bus' config value");
        format_err!("Failed to load 'bus' config value");
    })
    .unwrap();

    let uart_timeout: Duration = Duration::from_secs(u64::from_str(uart_timeout.as_str().unwrap()).unwrap());

    // Only needed for the ground feature
    #[cfg(feature = "ground")]
    let socket = service_config
    .get("udp_socket")
    .ok_or_else(|| {
        error!("Failed to load 'udp-socket' config value");
        format_err!("Failed to load 'udp-socket' config value");
    })
    .unwrap();

    #[cfg(feature = "ground")]
    let target = service_config
    .get("target")
    .ok_or_else(|| {
        error!("Failed to load 'target' config value");
        format_err!("Failed to load 'target' config value");
    })
    .unwrap();

    // let i2c_bus = i2c_bus.as_str().unwrap();
    // let i2c_addr = i2c_addr.as_u16().unwrap();
    // let i2c_bus = bus.as_str().unwrap();
    // let i2c_bus = bus.as_str().unwrap();
    // let i2c_bus = bus.as_str().unwrap();

    let subsystem: Box<Subsystem> = Box::new(
        match Subsystem::new(i2c_bus,i2c_addr,uart_bus,uart_setting,uart_timeout)
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

    #[cfg(feature = "ground")]
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

    #[cfg(not(any(feature = "ground",feature = "graphql")))]
    //Start up UDP server
    Service::new(
        service_config,
        subsystem,
        Some(Arc::new(udp_handler)),
    )
    .start();

    #[cfg(debug)]
    println!("{:?}", service_config);

    #[cfg(debug)]
    debug();

    Ok(())
}