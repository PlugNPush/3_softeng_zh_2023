/// This module contains the http handlers of the server.
pub mod handlers;

/// This module contains the router. It is responsible for routing
/// incoming requests from users to the appropriate handlers,
/// based on the [path] and [method] of the request.
///
/// [path]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Identifying_resources_on_the_Web#path
/// [method]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub mod router;

/// Contains the runtime configruation of the server,
/// e.g. which port to run on.
pub mod config;

/// Contains the shared state of the server.
pub mod state;

/// This module contains the routes to the graphical user interface.
pub mod frontend;

/// This module contains the routes to the documentation.
pub mod docs;

/// Module subscribes to device measurements from an MQTT broker.
pub mod mqtt;
