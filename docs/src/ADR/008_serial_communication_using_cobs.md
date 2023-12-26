# Serial Communication Using COBS

## Status

Accepted.

## Context

Serial communication needs a way to distinguish messages from each other (the
data does not necessarily arrive neatly bounded, it might be split up). An easy
way to achieve this is to designate a boundary byte, for example `0x00`. A
consequence of this is having to escape all boundary bytes within a message,
which can lead to almost doubling of message sizes in the worst case.

A simple way around this is to use a byte stuffing protocol like
[COBS](https://ieeexplore.ieee.org/document/769765). It is simple to implement
and brings the worst case overhead down to a negligible size.

## Decision

Use COBS for message encoding in serial communication.

## Consequences

Messages for serial communication do not get blown up by escaping of boundary
bytes.
