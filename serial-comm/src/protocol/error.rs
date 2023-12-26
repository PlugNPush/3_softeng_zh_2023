use core::str::Utf8Error;

use crate::cobs;

use super::cmd::{Cmd, Cobs};

/// The error command carries a message.
pub struct ErrorCommand<'a> {
    pub prefix: &'a str,
    pub msg: &'a str,
}

impl<'a> ErrorCommand<'a> {
    const PARAMS: u8 = 2;
    const HEADER_LENGTH: usize = Self::PARAMS as usize + 1;

    pub fn new(msg: &'a str) -> Self {
        Self { prefix: "ER", msg }
    }
}

impl<'a> Cobs<'a> for ErrorCommand<'a> {
    fn as_cobs<const OUTPUT: usize>(&self) -> (usize, [u8; OUTPUT]) {
        let prefix_bytes = self.prefix.as_bytes();
        let msg_bytes = self.msg.as_bytes();

        let prefix_end = prefix_bytes.len() + Self::HEADER_LENGTH;
        let msg_end = prefix_bytes.len() + msg_bytes.len() + Self::HEADER_LENGTH;

        let mut output: [u8; OUTPUT] = [0; OUTPUT];

        output[0] = ErrorCommand::PARAMS;
        output[1] = prefix_bytes.len() as u8;
        output[2] = msg_bytes.len() as u8;
        output[ErrorCommand::HEADER_LENGTH..prefix_end].copy_from_slice(prefix_bytes);
        output[prefix_end..msg_end].copy_from_slice(msg_bytes);

        cobs::encode(output, msg_end)
    }
}

impl<'a> Cmd<'a> for ErrorCommand<'a> {
    fn from_bytes<const INPUT: usize>(
        input: [u8; INPUT],
        buffer: &'a mut [u8; INPUT],
    ) -> Result<Self, Utf8Error> {
        let params = <ErrorCommand<'a> as Cmd>::params_from_bytes::<
            INPUT,
            { ErrorCommand::PARAMS as usize },
        >(input, buffer)?;

        Ok(Self {
            prefix: params[0],
            msg: params[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCommand;
    use crate::cobs;
    use crate::protocol::cmd::{Cmd, Cobs};

    #[test]
    fn test_ok_cmd() {
        let cmd = ErrorCommand::new("error");
        let (length, cmd_cobs) = cmd.as_cobs::<64>();

        assert_eq!(length, 11);
        assert_eq!(
            &cmd_cobs[0..11],
            &[0x0b, 0x02, 0x02, 0x05, 0x45, 0x52, 0x65, 0x72, 0x72, 0x6f, 0x72]
        );

        let (_, decoded): (usize, [u8; 64]) = cobs::decode(cmd_cobs, 11);

        let mut cmd_buffer: [u8; 64] = [0; 64];
        let cmd = ErrorCommand::from_bytes::<64>(decoded, &mut cmd_buffer).unwrap();
        assert_eq!(cmd.prefix, "ER");
        assert_eq!(cmd.msg, "error");
    }
}
