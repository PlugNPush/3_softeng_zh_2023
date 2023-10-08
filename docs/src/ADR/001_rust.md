# Rust everywhere

## Status

Accepted.

## Context

The requirements for this project include:
- collecting data on a microcontroller
- storing and serving the data with a web service
- displaying the data in an SPA

These are vastly different domains, for which different programming languages are well suited.
For an SPA, JavaScript is completely dominant.
For the embedded part, C has the necessary performance and ecosystem.
In the web service space, competition is lively.
Java, Python and Go and more are common choices.

The only language that's a rock-solid option for every one of these domains is Rust.
It has the performance and control of C with memory-safety on top and a fast-paced embedded ecosystem.
Its expressive type system makes it a breeze to build reliable and correct web services.
Finally, its best-in-class support for compiling to webassembly and GUI-libraries utilizing state-of-the-art rendering patterns make it well-suited for developing an SPA.

## Decision

The entire software system will be written in Rust.

## Consequences

Developing both the SPA and the microcontroller will be more challenging, as Rust doesn't have as mature of an ecosystem as JavaScript and C respectively.

It will be significantly easier to share code between the three software components.
Developing cross-cutting features will require less context-switching.
Testing and CI/CD will be significantly easier to setup, as there is only one toolchain to worry about.
