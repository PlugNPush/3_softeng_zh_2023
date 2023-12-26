/// Encode an integer as a variable byte integer
/// [MQTT 5.0: 1.5.5](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc473619950)
pub fn encode_length(length: u32, encoded_length: &mut [u8; 4]) -> u8 {
    let mut length = length;

    let mut idx = 0;
    while idx < encoded_length.len() && length > 0 {
        let mut encoded_byte = (length % 128) as u8;
        length /= 128;

        if length > 0 {
            encoded_byte |= 0x80; // Set the highest bit
        }
        encoded_length[idx] = encoded_byte;

        idx += 1;
    }

    idx as u8
}

/// Decode a variable byte integer as an integer.
/// [MQTT 5.0: 1.5.5](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc473619950)
pub fn decode_length(bytes: [u8; 4]) -> u32 {
    let mut length = 0u32;
    let mut multiplier = 1u32;

    let mut idx = 0;
    let mut byte = bytes[idx];

    while idx < bytes.len() && byte & 0x80 != 0 {
        length += ((byte & 0x7F) as u32) * multiplier;
        multiplier *= 128;

        idx += 1;
        byte = bytes[idx]
    }
    length += ((byte & 0x7F) as u32) * multiplier;

    length
}

#[cfg(test)]
mod tests {
    use crate::variable_length::{decode_length, encode_length};

    #[test]
    fn test_encode_decode_length_1_byte() {
        let mut length = [0u8; 4];
        let byte_length = encode_length(16, &mut length);

        assert_eq!(byte_length, 1);
        assert_eq!(length, [0x10, 0x00, 0x00, 0x00]);

        let length = decode_length(length);
        assert_eq!(length, 16);
    }

    #[test]
    fn test_encode_decode_length_2_bytes() {
        let mut length = [0u8; 4];
        let byte_length = encode_length(568, &mut length);

        assert_eq!(byte_length, 2);
        assert_eq!(length, [0xb8, 0x04, 0x00, 0x00]);

        let length = decode_length(length);
        assert_eq!(length, 568);
    }

    #[test]
    fn test_encode_decode_length_3_bytes() {
        let mut length = [0u8; 4];
        let byte_length = encode_length(85734, &mut length);

        assert_eq!(byte_length, 3);
        assert_eq!(length, [0xe6, 0x9d, 0x05, 0x00]);

        let length = decode_length(length);
        assert_eq!(length, 85734);
    }

    #[test]
    fn test_encode_length_4_bytes() {
        let mut length = [0u8; 4];
        let byte_length = encode_length(8573471, &mut length);

        assert_eq!(byte_length, 4);
        assert_eq!(length, [0x9f, 0xa4, 0x8b, 0x04]);

        let length = decode_length(length);
        assert_eq!(length, 8573471);
    }
}
