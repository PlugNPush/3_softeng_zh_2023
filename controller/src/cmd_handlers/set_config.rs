use defmt::info;
use serial_comm::protocol::{cmd::Cmd, set_config::SetConfig};

use crate::{cmd_handlers::CmdError, flash::Config};

use super::PACKET_SIZE;

/// Handle a SetConfig command by writing the received key-value pair to flash storage.
pub fn handle(data: [u8; PACKET_SIZE], config: &mut Config<'_>) -> Result<(), CmdError> {
    let mut cmd_buffer: [u8; PACKET_SIZE] = [0; PACKET_SIZE];

    let set_config = SetConfig::from_bytes::<PACKET_SIZE>(data, &mut cmd_buffer)?;
    config.write_config(set_config.key, set_config.value)?;
    info!("set {} to {}", set_config.key, set_config.value);

    Ok(())
}
