use crate::fixed_header::FixedHeader;

// [MQTT 5.0: 3.3.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc359155681)
const FIXED_PUBLISH_TYPE: u8 = 0x03;
// no flags set [MQTT 5.0: 3.3.1](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc359155681)
const FIXED_PUBLISH_FLAGS: u8 = 0x00;

// no properties [MQTT 5.0: 3.3.2.3](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc511988586)
const PROPERTIES: [u8; 1] = [0x00];

// technically this is 65535 (see https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_UTF-8_Encoded_String)
// but we do not need that here
pub const MAX_TOPIC_LENGTH: usize = 254;

// this is again arbitrarilly constricted, technically it could be as long as the variable length
// encoding allows (minus fixed and variable header)
const MAX_PAYLOAD_LENGTH: usize = 128;
// 7 is 5 + 2 which is max length of the fixed header and the two bytes used to encode the topic
// length
const MAX_PUBLISH_LENGTH: usize = MAX_TOPIC_LENGTH + 7 + MAX_PAYLOAD_LENGTH;

/// Encode a string to an MQTT length encoded string
/// [MQTT 5.0: 1.5.4](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc462729066)
fn encode_mqtt_str(topic: &str, encode_buffer: &mut [u8; MAX_TOPIC_LENGTH]) -> usize {
    let topic_data = topic.as_bytes();
    assert!(topic_data.len() <= MAX_TOPIC_LENGTH);

    encode_buffer[0] = 0; // no need for the high bits as our max topic length is too small
    encode_buffer[1] = topic_data.len() as u8;
    encode_buffer[2..2 + topic_data.len()].copy_from_slice(topic_data);

    topic_data.len() + 2
}

pub struct Publish {
    // byte representation of the publish packet
    pub data: [u8; MAX_PUBLISH_LENGTH],
    // actual length of the whole publish packet
    pub length: usize,
}

impl Publish {
    /// Create an MQTT publish packet.
    ///
    /// See [MQTT 5.0: 3.3](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc384800410)
    /// for the byte structure.
    ///
    /// `topic`: defines the topic to publish to
    /// `payload` value to publish as is
    pub fn new(topic: &str, payload: &[u8]) -> Self {
        let mut topic_data = [0u8; MAX_TOPIC_LENGTH];
        let topic_length = encode_mqtt_str(topic, &mut topic_data);

        let packet_length = PROPERTIES.len() + topic_length + 2 + payload.len();
        let fixed_header = FixedHeader::new(
            FIXED_PUBLISH_TYPE,
            FIXED_PUBLISH_FLAGS,
            packet_length as u32,
        );

        let mut data = [0u8; MAX_PUBLISH_LENGTH];
        data[0] = fixed_header.type_flags;
        data[1..1 + fixed_header.length]
            .copy_from_slice(&fixed_header.remaining_length[0..fixed_header.length]);
        let fixed_offset = fixed_header.length + 1;

        let topic_idx = fixed_offset;
        let properties_idx = topic_idx + topic_length;
        let payload_idx = properties_idx + PROPERTIES.len();

        data[topic_idx..properties_idx].copy_from_slice(&topic_data[0..topic_length]);
        data[properties_idx..payload_idx].copy_from_slice(&PROPERTIES);
        data[payload_idx..payload_idx + payload.len()].copy_from_slice(payload);

        Self {
            data,
            length: packet_length + fixed_header.length + 1,
        }
    }
}
