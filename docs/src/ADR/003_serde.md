# JSON with serde

## Status

Accepted.

## Context

Web services communicate using well-specified formats, typically JSON.
The best way to do de-/serialization of JSON or another format in Rust is to use [serde].
Since our tech stack is pure Rust, there is no need for OpenAPI or similar contract enforcement mechanisms.
Shared definitions of the data types sent across process boudaries ensure compatibility.
JSON, being a text-based format, is not the most efficient, but easily inspectable.
For more efficient communication, binary formats are better suited.

## Decision

The web service communicates via JSON, powered by serde.

## Consequences

De-/serialization of inter-process messages will not have to be done by hand.
A potential future migration to a more performant format than JSON is trivial.

[serde]: https://serde.rs/
