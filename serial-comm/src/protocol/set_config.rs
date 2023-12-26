use core::str::Utf8Error;

use crate::cobs;

use super::cmd::{Cmd, Cobs};

#[derive(Debug)]
/// The SetConfig command sets the value of a key.
pub struct SetConfig<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> SetConfig<'a> {
    pub const PREFIX: &'a str = "SC";
    const PARAMS: u8 = 3;
    const HEADER_LENGTH: usize = Self::PARAMS as usize + 1;

    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

impl<'a> Cobs<'a> for SetConfig<'a> {
    /// Encode a SetConfig command with the COBS algorithm.
    ///
    /// Return a tuple of the structure (<encoded_length>, <output_array>)
    fn as_cobs<const OUTPUT: usize>(&self) -> (usize, [u8; OUTPUT]) {
        let prefix_bytes = Self::PREFIX.as_bytes();
        let key_bytes = self.key.as_bytes();
        let val_bytes = self.value.as_bytes();

        let prefix_end = prefix_bytes.len() + Self::HEADER_LENGTH;
        let key_end = prefix_end + key_bytes.len();
        let val_end = key_end + val_bytes.len();

        let mut output: [u8; OUTPUT] = [0; OUTPUT];

        output[0] = Self::PARAMS;
        output[1] = prefix_bytes.len() as u8;
        output[2] = key_bytes.len() as u8;
        output[3] = val_bytes.len() as u8;
        output[Self::HEADER_LENGTH..prefix_end].copy_from_slice(prefix_bytes);
        output[prefix_end..key_end].copy_from_slice(key_bytes);
        output[key_end..val_end].copy_from_slice(val_bytes);

        cobs::encode(output, val_end)
    }
}

impl<'a> Cmd<'a> for SetConfig<'a> {
    fn from_bytes<const INPUT: usize>(
        input: [u8; INPUT],
        buffer: &'a mut [u8; INPUT],
    ) -> Result<Self, Utf8Error> {
        let params = <SetConfig<'a> as Cmd>::params_from_bytes::<
            INPUT,
            { SetConfig::PARAMS as usize },
        >(input, buffer)?;

        Ok(Self {
            key: params[1],
            value: params[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::cobs;
    use crate::protocol::cmd::{Cmd, Cobs};
    use crate::protocol::set_config::SetConfig;

    #[test]
    fn test_config_cmd() {
        let cmd = SetConfig::new("ssid", "pw");
        let (length, cmd_cobs) = cmd.as_cobs::<64>();

        assert_eq!(length, 13);
        assert_eq!(
            &cmd_cobs[0..13],
            &[0x0d, 0x03, 0x02, 0x04, 0x02, 0x53, 0x43, 0x73, 0x73, 0x69, 0x64, 0x70, 0x77]
        );

        let (_, decoded): (usize, [u8; 64]) = cobs::decode(cmd_cobs, 13);

        let mut cmd_buffer: [u8; 64] = [0; 64];
        let cmd = SetConfig::from_bytes::<64>(decoded, &mut cmd_buffer).unwrap();
        assert_eq!(cmd.key, "ssid");
        assert_eq!(cmd.value, "pw");
    }
}
