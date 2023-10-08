# SPA with Leptos

## Status

Accepted.

## Context

Modern single-page-applications are written using component-based UI libraries.
This is because updating the DOM globally in am imperative manner quickly becomes unmanageable.
In the JavaScript ecosystem, [React] is the most popular, but there are other ones using different reactivity systems.
Rust also has several such UI libraries, despite the ecosystem being much younger.
The Rust-equivalent for React is [Yew], but another notable library is [Leptos].
It has comparable popularity to Yew and uses a rendering system most similar to JavaScript's [Solid].
For our requirement of implementing the flux/redux/store pattern, both are equally suitable.
Both provide an API for making a reactive singleton object accesible to the entire component tree.
There is existing experience with Leptos in the development team.

## Decision

The SPA will be developed using the Leptos UI library.

## Consequences

Developing the SPA will be a smooth experience.

[React]: https://react.dev/
[Yew]: https://yew.rs/
[Leptos]: https://leptos.dev/
[Solid]: https://www.solidjs.com/
