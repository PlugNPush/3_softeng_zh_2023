//! Implement just enough of the MQTT protocol to be able to publish data with
//! QOS 0.
//! All formats are taken from [MQTT Version 5.0](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html).

#![cfg_attr(not(test), no_std)]

pub mod connect;
pub mod fixed_header;
pub mod publish;
pub mod variable_length;
