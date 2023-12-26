# MQTT Implementation in Rust

## Status

Accepted.

## Context

Using the
[MQTT protocol](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html)
for communication is an established way to communicate with resource constrained
devices like microcontrollers.

While there are pre-existing MQTT libraries in rust, none of them work on the
target architecture. As MQTT is well specified and we only need the bare minimum
of functionality for now, it is possible to implement it on our own.

## Decision

Enough of MQTT will be implemented to send out messages (version 5.0, QoS 0, no
properties, no keep alive, merely fire and forget).

## Consequences

The microcontroller can send out data in an efficient and standard way.
