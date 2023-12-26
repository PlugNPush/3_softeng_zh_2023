use core::str::Utf8Error;

use crate::cobs;

use super::cmd::{Cmd, Cobs};

// The Ok command has no parameters, it should merely signify something went ok.
pub struct OkCommand<'a> {
    pub prefix: &'a str,
}

impl<'a> OkCommand<'a> {
    const PARAMS: u8 = 1;
    const HEADER_LENGTH: usize = Self::PARAMS as usize + 1;

    pub fn new() -> Self {
        Self { prefix: "OK" }
    }
}

impl<'a> Default for OkCommand<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Cobs<'a> for OkCommand<'a> {
    fn as_cobs<const OUTPUT: usize>(&self) -> (usize, [u8; OUTPUT]) {
        let prefix_bytes = self.prefix.as_bytes();
        let prefix_end = prefix_bytes.len() + Self::HEADER_LENGTH;

        let mut output: [u8; OUTPUT] = [0; OUTPUT];

        output[0] = OkCommand::PARAMS;
        output[1] = prefix_bytes.len() as u8;
        output[OkCommand::HEADER_LENGTH..prefix_end].copy_from_slice(prefix_bytes);

        cobs::encode(output, prefix_end)
    }
}

impl<'a> Cmd<'a> for OkCommand<'a> {
    fn from_bytes<const INPUT: usize>(
        input: [u8; INPUT],
        buffer: &'a mut [u8; INPUT],
    ) -> Result<Self, Utf8Error> {
        let params = <OkCommand<'a> as Cmd>::params_from_bytes::<
            INPUT,
            { OkCommand::PARAMS as usize },
        >(input, buffer)?;

        Ok(Self { prefix: params[0] })
    }
}

#[cfg(test)]
mod tests {
    use super::OkCommand;
    use crate::cobs;
    use crate::protocol::cmd::{Cmd, Cobs};

    #[test]
    fn test_ok_cmd() {
        let cmd = OkCommand::new();
        let (length, cmd_cobs) = cmd.as_cobs::<64>();

        assert_eq!(length, 5);
        assert_eq!(&cmd_cobs[0..5], &[0x05, 0x01, 0x02, 0x4f, 0x4b]);

        let (_, decoded): (usize, [u8; 64]) = cobs::decode(cmd_cobs, 5);

        let mut cmd_buffer: [u8; 64] = [0; 64];
        let cmd = OkCommand::from_bytes::<64>(decoded, &mut cmd_buffer).unwrap();
        assert_eq!(cmd.prefix, "OK");
    }
}
