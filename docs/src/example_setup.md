# Example Setup

A description of a sample setup, bringing all pieces together.

## Prerequisites

Refer to the main readme and the readme of the controller project to install
necessary tools.

## MQTT Broker

Create a file `mosquitto.conf` with the following contents:

```
listener 1883
allow_anonymous true
connection_messages true
log_type all
```

This allows for anonymous access and shows log messages.

Run the mosquitto container in the same directory:

```
podman run -it -p 1883:1883 -v "$PWD"/mosquitto.conf:/mosquitto/config/mosquitto.conf docker.io/eclipse-mosquitto
```

This example uses podman, but docker should work with the same parameters.

## Microcontroller

Flash the microcontroller as described in the `controller` subproject.

Configure all necessary values on the microcontroller. For the following example
an ssid of `test` with a password of `test`, an mqtt url of `127.0.0.1` and a
client-id of `potato` is used.

Set the all these values:

```
echo -en "\x0f\x03\x02\x04\x04\x53\x43\x73\x73\x69\x64\x74\x65\x73\x74\x00" > /dev/ttyACM1
echo -en "\x12\x03\x02\x07\x04\x53\x43\x73\x73\x69\x64\x5f\x70\x77\x74\x65\x73\x74\x00" > /dev/ttyACM1
echo -en "\x14\x03\x02\x04\x09\x53\x43\x6d\x71\x74\x74\x31\x32\x37\x2e\x30\x2e\x30\x2e\x31\x00" > /dev/ttyACM1
echo -en "\x16\x03\x02\x09\x06\x53\x43\x63\x6c\x69\x65\x6e\x74\x5f\x69\x64\x70\x6f\x74\x61\x74\x6f\x00" > /dev/ttyACM1
```

These bytes can be generated with the help of the `serial-comm` crate, using the
following code snippet (replace `ssid` and `test` with your respective key and
value):

```
let cmd = SetConfig::new("ssid", "test");
let (length, cmd_cobs) = cmd.as_cobs::<64>();
for i in 0..length {
  print!("\\x{:02x}", cmd_cobs[i]);
}
println!();
```

Reboot the microcontroller and it will connect to the specified network (allow
up to 20 seconds for the connection) and start sending temperature measurements
every ten seconds.

To check that the publishing is working it is possible to leverage mosquitto as
follows:

```
mosquitto_sub -v -h 127.0.0.1 -p 1883 -t 'temps/#' -F %X
```

Replace the host with where your mqtt broker is accessible.

## Backend

Run the following from the project root to start the server:

```
cd server && cargo run
```

The default mqtt url is `localhost`, the default mqtt port is `1883`. If these
are different for your setup, supply them as arguments as follows:

```
server && cargo run -q -- -m <mqtt-host> -q <mqtt-port>
```

## Frontend

Run the following from the project root to start the frontend:

```
cd app && trunk serve --open
```

If all the above configurations are correct, a new browser tab will open at
[http://localhost:3000](http://localhost:3000) where new measurements will come
in every ten seconds.
