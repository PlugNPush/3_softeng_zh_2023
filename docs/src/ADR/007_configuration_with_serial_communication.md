# Configuration With Serial Communication

## Status

Accepted.

## Context

The microcontroller needs to connect to networks and MQTT brokers to send data.
In order not to hardcode passwords and other configuration, there needs to be a
mechanism to set those values and persist them across reboots. The used
microcontroller (Raspberry Pi Pico) has flash memory that can be used for such
an endeavour.

When the device is connected with USB, a serial connection can be established
and used for communication with the microcontroller.

## Decision

Create a simple protocol to write configuration values across a serial
connection. These are persisted in flash memory and read at startup.

## Consequences

The microcontroller can be configured without having settings baked in at
compile time.
