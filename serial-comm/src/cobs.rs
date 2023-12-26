// sentinel value for backage boundaries
// could be anything but 0x00 is used as a default and makes sense
pub const SENTINEL: u8 = 0x00;
// max size of non SENTINEL values before a code split occurs
const SPLIT_BOUNDARY: u8 = 0xff;

/// Encode a byte array of size `INPUT` with an actual data length of`length`
/// to an output array of size `OUTPUT`, using the COBS algorithm.
///
/// Return a tuple of the structure (<encoded_length>, <output_array>)
pub fn encode<const INPUT: usize, const OUTPUT: usize>(
    input: [u8; INPUT],
    length: usize,
) -> (usize, [u8; OUTPUT]) {
    let mut output: [u8; OUTPUT] = [0; OUTPUT];

    let mut encode_idx = 1;
    let mut code_idx = 0;
    let mut code = 1;

    for byte in input.iter().take(length) {
        if *byte == SENTINEL {
            output[code_idx] = code;
            code = 1;
            code_idx = encode_idx;
            encode_idx += 1;
        } else {
            output[encode_idx] = *byte;
            encode_idx += 1;
            code += 1;

            if code == SPLIT_BOUNDARY {
                output[code_idx] = code;
                code = 1;
                code_idx = encode_idx;

                if encode_idx < length {
                    encode_idx += 1;
                }
            }
        }
    }

    if code_idx < OUTPUT {
        output[code_idx] = code;
    }

    (encode_idx, output)
}

/// Decode a COBS encoded byte array of length `INPUT` and an actual data length of `length` to an
/// array of length `OUTPUT`.
///
/// Return a tuple of the structure (<decoded_length>, <output_array>)
pub fn decode<const INPUT: usize, const OUTPUT: usize>(
    input: [u8; INPUT],
    length: usize,
) -> (usize, [u8; OUTPUT]) {
    let mut output: [u8; OUTPUT] = [0; OUTPUT];

    let mut out_idx = 0;
    let mut idx = 0;

    while idx < length {
        let code = input[idx];
        idx += 1;

        for _ in 1..code {
            if idx < length {
                output[out_idx] = input[idx];
                out_idx += 1;
                idx += 1;
            }
        }

        if code != SPLIT_BOUNDARY && idx < length {
            output[out_idx] = SENTINEL;
            out_idx += 1;
        }
    }

    (out_idx, output)
}

#[cfg(test)]
mod tests {
    use crate::cobs::{decode, encode};

    #[test]
    fn test_paper_example() {
        let input = [
            0x45, 0x00, 0x00, 0x2c, 0x4c, 0x79, 0x00, 0x00, 0x40, 0x06, 0x4f, 0x37,
        ];
        let output = [
            0x02, 0x45, 0x01, 0x04, 0x2c, 0x4c, 0x79, 0x01, 0x05, 0x40, 0x06, 0x4f, 0x37,
        ];
        assert_eq!(encode(input, 12), (13, output));
        assert_eq!(decode(output, 13), (12, input));
    }

    #[test]
    fn test_empty_input() {
        let input = [];
        let output = [0x01];
        assert_eq!(encode(input, 0), (1, output));
        assert_eq!(decode(output, 1), (0, input));
    }

    #[test]
    fn test_all_zeros() {
        let input = [0x00, 0x00, 0x00];
        let output = [0x01, 0x01, 0x01, 0x01];
        assert_eq!(encode(input, 3), (4, output));
        assert_eq!(decode(output, 4), (3, input));
    }

    #[test]
    fn test_no_zeros() {
        let input = [0x01, 0x02, 0x03];
        let output = [0x04, 0x01, 0x02, 0x03];
        assert_eq!(encode(input, 3), (4, output));
        assert_eq!(decode(output, 4), (3, input));
    }

    #[test]
    fn test_no_zeros_in_254_bytes() {
        let input: [u8; 254] = [0x01; 254];

        // expected output will have the first byte as 0xFF followed by the 255 non-zero bytes.
        let mut output: [u8; 255] = [0x01; 255];
        output[0] = 0xff;

        assert_eq!(encode(input, 254), (255, output));
        assert_eq!(decode(output, 255), (254, input));
    }

    #[test]
    fn test_no_zeros_in_254_bytes_and_additional_block() {
        let mut input: [u8; 259] = [0x01; 259];
        input[254..259].copy_from_slice(&[0x02, 0x00, 0x03, 0x0c, 0x01]);

        // expected output will have the first byte as 0xFF followed by the 254 non-zero bytes.
        let mut output: [u8; 261] = [0x01; 261];
        output[0] = 0xff;
        output[255..261].copy_from_slice(&[0x02, 0x02, 0x04, 0x03, 0x0c, 0x01]);

        assert_eq!(encode(input, 259), (261, output));
        assert_eq!(decode(output, 261), (259, input));
    }
}
