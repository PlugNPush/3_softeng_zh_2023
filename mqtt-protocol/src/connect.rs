use crate::fixed_header::FixedHeader;

// [MQTT 5.0: 3.1.3.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349242)
const ALLOWED_CLIENT_ID_CHARS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// [MQTT 5.0: 3.1.3.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349242)
const MAX_CLIENT_ID_SIZE: usize = 23;

// [MQTT 5.0: 3.1.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_CONNECT_Fixed_Header)
const FIXED_CONNECT_TYPE: u8 = 0x01;
// [MQTT 5.0: 3.1.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_CONNECT_Fixed_Header)
const FIXED_CONNECT_FLAGS: u8 = 0x00;

// "MQTT" [MQTT 5.0: 3.1.2.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349225)
const PROTOCOL_NAME: [u8; 6] = [0x00, 0x04, 0x4d, 0x51, 0x54, 0x54];
// "version 5" [MQTT 5.0: 3.1.2.2](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349227)
const PROTOCOL_VERSION: [u8; 1] = [0x05];
// no user name, no will qos, no will, no clean start [MQTT 5.0: 3.1.2.3 - 3.1.2.9](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349229)
const CONNECT_FLAGS: [u8; 1] = [0x00];
// turn off keep alive mechanism [MQTT 5.0: 3.1.2.10](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Keep_Alive_1)
const KEEP_ALIVE: [u8; 2] = [0x00, 0x00];
// no properties [MQTT 5.0: 3.1.2.11](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc511988523)
const PROPERTIES: [u8; 1] = [0x00];

// calculate position in the data array (those that are constant)
const PROTOCOL_NAME_IDX: usize = 0;
const PROTOCOL_VERSION_IDX: usize = PROTOCOL_NAME_IDX + PROTOCOL_NAME.len();
const CONNECT_FLAGS_IDX: usize = PROTOCOL_VERSION_IDX + PROTOCOL_VERSION.len();
const KEEP_ALIVE_IDX: usize = CONNECT_FLAGS_IDX + CONNECT_FLAGS.len();
const PROPERTIES_IDX: usize = KEEP_ALIVE_IDX + KEEP_ALIVE.len();
const PAYLOAD_IDX: usize = PROPERTIES_IDX + PROPERTIES.len();

// max size the variable header + payload of a connect packet
const MAX_PACKET_LENGTH: usize = PROTOCOL_NAME.len()
    + PROTOCOL_VERSION.len()
    + CONNECT_FLAGS.len()
    + KEEP_ALIVE.len()
    + PROPERTIES.len()
    + MAX_CLIENT_ID_SIZE
    + 2; // two bytes for the client id length

// max size a whole connect packet can be
const MAX_CONNECT_LENGTH: usize = MAX_PACKET_LENGTH + 5; // 5 is the max length of the fixed header

pub struct Connect {
    // byte representation of the connect packet
    pub data: [u8; MAX_CONNECT_LENGTH],
    // actual length of the whole connect packet
    pub length: usize,
}

impl Connect {
    /// Create an MQTT connect packet.
    ///
    /// See [MQTT 5.0: 3.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_CONNECT_%E2%80%93_Connection)
    /// for the byte structure.
    ///
    /// `client_id` is how the client identifies itself. According to
    /// [MQTT 5.0: 3.1.3.1]((https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc385349242)
    /// this is not strictly necessary but a server does not have to accept this.
    ///
    /// The same holds for the limitation of 23 bytes for the `client_id`.
    pub fn new(client_id: &str) -> Self {
        assert!(client_id
            .chars()
            .all(|x| ALLOWED_CLIENT_ID_CHARS.contains(x)));

        let client_id_data = client_id.as_bytes();
        assert!(!client_id_data.is_empty() && client_id_data.len() <= MAX_CLIENT_ID_SIZE);

        let client_id_length = client_id_data.len() as u16;
        let high_byte = ((client_id_length & 0xFF00) >> 8) as u8;
        let low_byte = (client_id_length & 0x00FF) as u8;

        let mut payload_data = [0u8; MAX_CLIENT_ID_SIZE + 2];
        payload_data[0] = high_byte;
        payload_data[1] = low_byte;
        payload_data[2..2 + client_id_data.len()].copy_from_slice(client_id_data);

        let packet_length = MAX_PACKET_LENGTH - MAX_CLIENT_ID_SIZE + client_id_data.len();
        let fixed_header = FixedHeader::new(
            FIXED_CONNECT_TYPE,
            FIXED_CONNECT_FLAGS,
            packet_length as u32,
        );

        let mut data = [0u8; MAX_CONNECT_LENGTH];
        data[0] = fixed_header.type_flags;
        data[1..1 + fixed_header.length]
            .copy_from_slice(&fixed_header.remaining_length[0..fixed_header.length]);
        let fixed_offset = fixed_header.length + 1;

        data[fixed_offset + PROTOCOL_NAME_IDX..fixed_offset + PROTOCOL_VERSION_IDX]
            .copy_from_slice(&PROTOCOL_NAME);
        data[fixed_offset + PROTOCOL_VERSION_IDX..fixed_offset + CONNECT_FLAGS_IDX]
            .copy_from_slice(&PROTOCOL_VERSION);
        data[fixed_offset + CONNECT_FLAGS_IDX..fixed_offset + KEEP_ALIVE_IDX]
            .copy_from_slice(&CONNECT_FLAGS);
        data[fixed_offset + KEEP_ALIVE_IDX..fixed_offset + PROPERTIES_IDX]
            .copy_from_slice(&KEEP_ALIVE);
        data[fixed_offset + PROPERTIES_IDX..fixed_offset + PAYLOAD_IDX]
            .copy_from_slice(&PROPERTIES);
        data[fixed_offset + PAYLOAD_IDX..fixed_offset + PAYLOAD_IDX + client_id_data.len() + 2]
            .copy_from_slice(&payload_data[0..client_id_data.len() + 2]);

        Self {
            data,
            length: packet_length + fixed_header.length + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Connect;

    #[test]
    fn test_encode_decode_length_1_byte() {
        let connect = Connect::new("Tomato");
        assert_eq!(
            connect.data[0..connect.length],
            [
                0x10, 0x13, 0x00, 0x04, 0x4d, 0x51, 0x54, 0x54, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x06, 0x54, 0x6f, 0x6d, 0x61, 0x74, 0x6f
            ]
        )
    }
}
