# HTTP routing with axum

## Status

Accepted.

## Context

The Rust standard library does not include an HTTP router.
A common task like that is best solved by using a library.
There are several suitable ones available for Rust.
[Actix] and [axum] are the most popular at the time of writing.
For our simple requirements, neither is better or worse.
There is existing experience with axum in the development team.

## Decision

HTTP routing in the web service will be handled using axum.

## Consequences

HTTP routing will not have to be done by hand.

[Actix]: https://docs.rs/actix/latest/actix/
[axum]: https://docs.rs/axum/latest/axum/
