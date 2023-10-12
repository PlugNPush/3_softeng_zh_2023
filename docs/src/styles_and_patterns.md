# Architectural Styles and Patterns

Here we will discuss some of the architectural styles and patterns
our applications adheres to.
Special attention will be given to the ones discussed during the lecture.

## Client-Server Architecture

The architecture of our application clearly adheres to the client-server style.
There is one central server exposing a REST API,
serving any number of clients in any physical location.

## Service-Oriented Architecture

This is similar to the client-server style, with the additional emphasis
on the service not being coupled to specific clients, making it reusable.
This style requires the service to expose a well-defined interface.
This role is fulfilled by the REST API in our case.
The service currently communicates with two distinct clients:
the microcontroller and the web app.
Adding additional clients, such as a mobile app,
would require no changes to the architecture.

## N-Tier Architecture

Our application does not fit the N-Tier style at first glance.
Its most common implementation, the 3-Tier architecture, is not used.
We have no separation of the persistence layer from the business logic,
at least not *physically*.
If only for maintainability, these two are obviously separate code modules.

But our architecture cannot be described as a 2-Tier architecture either.
The microcontroller is a physically separate component,
but it cannot be lumped together with the web app in one logical tier.

## Layered Architecture

Any resemblance of our code to a layered architecture is unintentional.
Our application is too small to benefit from enforcing a strict hierarchy of layers.
Nevertheless, parts of the code may end up being organized like this anyway.
This should be attributed more to the design principle of *separation of concerns*.
It is universally applicable and benefitial even in small projects such as ours.

## Publish-Subscribe

The publish-subscribe architectural style is used in one place.
The web service has one endpoint for subscribing to nofications.
These notifications are mostly for new temperature measurements.
Once a new temperature measurement is available, the web service publishes it
on the broadcasting channel to all subscribers.

It may be too much to say our application's *architecture*
implements the publish-subscribe style.
Only a small part of the software operates in this way.
The purpose is only to enable live updates of the GUI,
so this messaging system is an implementation detail of a single use case.
The consideration is relevant nonetheless.
