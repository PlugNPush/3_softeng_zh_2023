//! Handle specific commands, comming in from usb-serial.

pub mod set_config;

use defmt::Format;
use serial_comm::{cobs, protocol::set_config::SetConfig};

use crate::flash::{Config, FlashError};

#[derive(Debug)]
pub struct Utf8Error(pub core::str::Utf8Error);

impl Format for Utf8Error {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Utf8Error");
    }
}

#[derive(Debug, Format)]
pub enum CmdError {
    Utf8Error(Utf8Error),
    FlashError(FlashError),
    NoSuchCmd,
}

impl From<core::str::Utf8Error> for CmdError {
    fn from(err: core::str::Utf8Error) -> Self {
        CmdError::Utf8Error(Utf8Error(err))
    }
}

impl From<FlashError> for CmdError {
    fn from(err: FlashError) -> Self {
        CmdError::FlashError(err)
    }
}

// arbitrarily constrained, we know we don't need more for now
pub const PACKET_SIZE: usize = 256;

type CmdHandler = fn([u8; PACKET_SIZE], &mut Config) -> Result<(), CmdError>;
static CMD_HANDLERS: &[(&str, CmdHandler)] = &[(SetConfig::PREFIX, set_config::handle)];

/// Try decoding a command buffer and run it.
///
/// On failure return an ErrorCommand across the serial connection, otherwise an OkCommand.
pub fn handle_cmd(
    cmd_buffer: [u8; PACKET_SIZE],
    length: usize,
    config: &mut Config<'_>,
) -> Result<(), CmdError> {
    let (length, decoded_cmd): (usize, [u8; PACKET_SIZE]) = cobs::decode(cmd_buffer, length);

    if length > 1 {
        let header_length = (decoded_cmd[0] + 1) as usize;
        let cmd_prefix = core::str::from_utf8(&decoded_cmd[header_length..header_length + 2])?;

        let mut idx = 0;
        while idx < CMD_HANDLERS.len() && CMD_HANDLERS[idx].0 != cmd_prefix {
            idx += 1;
        }

        if idx == CMD_HANDLERS.len() {
            return Err(CmdError::NoSuchCmd);
        }

        let handler = CMD_HANDLERS[idx].1;
        handler(decoded_cmd, config)?;
    }

    Ok(())
}
