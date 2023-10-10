# Notifications with WebSocket

## Status

Accepted.

## Context

The traditional combination of REST API with SPA has no good mechanism for server-to-client communication.
The standard way to solve this problem is to use websockets.
They are lightweight and flexible, but they do add some amount of complexity to an application.
Websockets should therefore only be used if they provide concrete value.
For most applications, data mutations are immediately triggered by the user themself.
In that case, the REST API responses are usually sufficient to keep user's data up-to-date with the server.
In our case however, data mutations are mainly triggered by an IoT device without user interaction.
Therefore, we require server-to-client communication to ensure the user's data is up-to-date.

## Decision

Websockets will be used for server-to-client communication.

## Consequences

The user experience will be more responsive.
Ensuring correct bidirectional communication between server and client across two protocols will be more challending.
