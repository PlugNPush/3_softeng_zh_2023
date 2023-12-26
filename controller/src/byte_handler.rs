use defmt::error;
use embassy_rp::usb::{Driver, Instance};
use embassy_usb::{class::cdc_acm::CdcAcmClass, driver::EndpointError};
use serial_comm::{
    cobs::SENTINEL,
    protocol::{cmd::Cobs, error::ErrorCommand, ok::OkCommand},
};

use crate::{
    cmd_handlers::{handle_cmd, PACKET_SIZE},
    flash::Config,
};

pub struct Disconnected;

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

/// Handle incoming bytes over the usb-serial connection.
///
/// Always buffer data until you encounter the `SENTINEL` value. After that try to decode and
/// execute the received command.
pub async fn handle_bytes<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    config: &mut Config<'d>,
) -> Result<(), Disconnected> {
    let mut cmd_buffer: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
    let mut cursor: usize = 0;

    loop {
        let mut read_buf: [u8; 64] = [0; 64];
        let n = class.read_packet(&mut read_buf).await?;
        let data = &read_buf[..n];

        if n > 0 {
            cmd_buffer[cursor..cursor + n].copy_from_slice(data);
            cursor += n;
            if data[n - 1] == SENTINEL {
                match handle_cmd(cmd_buffer, cursor, config) {
                    Err(e) => {
                        error!(
                            "Failed to handle command: {:?} - {:?}",
                            e,
                            cmd_buffer[0..cursor]
                        );
                        let err_cmd = ErrorCommand::new("Failed to handle command");
                        let (length, err_cmd): (usize, [u8; 128]) = err_cmd.as_cobs();
                        class.write_packet(&err_cmd[0..length]).await?;
                    }
                    _ => {
                        let ok_cmd = OkCommand::new();
                        let (length, ok_cmd): (usize, [u8; 128]) = ok_cmd.as_cobs();
                        class.write_packet(&ok_cmd[0..length]).await?;
                    }
                };

                cursor = 0;
            }
        }
    }
}
