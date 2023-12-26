use super::variable_length::encode_length;

pub struct FixedHeader {
    pub type_flags: u8,
    // length of variable header + payload, encoded as a variable byte integer
    // ([MQTT 5.0: 1.5.5](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc473619950))
    pub remaining_length: [u8; 4],
    // how many bytes in the remaining length field are relevant (left to right)
    pub length: usize,
}

impl FixedHeader {
    /// Create a fixed header ([MQTT 5.0: 2.1.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc511988498)) for an MQTT packet.
    ///
    /// - `packet_type`: [MQTT 5.0: 2.1.2](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc353481061)
    /// - `flags`: [MQTT 5.0: 2.1.3](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc353481062)
    /// - `remaining_length`: Length of the variable header + payload.
    pub fn new(packet_type: u8, flags: u8, remaining_length: u32) -> Self {
        let mut remaining_length_data = [0u8; 4];
        let length = encode_length(remaining_length, &mut remaining_length_data);

        Self {
            type_flags: packet_type << 4 | (flags & 0x0F),
            remaining_length: remaining_length_data,
            length: length.into(),
        }
    }
}
