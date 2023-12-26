use defmt::{error, Format};
use embassy_rp::{
    flash::{self, Async, Flash, ERASE_SIZE},
    peripherals::{DMA_CH0, FLASH},
};

use crate::cmd_handlers::Utf8Error;

const ADDR_OFFSET: u32 = 0x100000;
const FLASH_SIZE: usize = 2 * 1024 * 1024;

pub const ENTRY_SIZE: u32 = ERASE_SIZE as u32;
const SSID_CONFIG: u32 = ADDR_OFFSET;
const SSID_PW_CONFIG: u32 = ADDR_OFFSET + ENTRY_SIZE;
const MQTT_CONFIG: u32 = ADDR_OFFSET + 2 * ENTRY_SIZE;
const CLIENT_ID_CONFIG: u32 = ADDR_OFFSET + 3 * ENTRY_SIZE;

pub const SSID_KEY: &str = "ssid";
pub const SSID_PW_KEY: &str = "ssid_pw";
pub const MQTT_KEY: &str = "mqtt";
pub const CLIENT_ID_KEY: &str = "client_id";

#[derive(Debug, Format)]
pub enum FlashError {
    Internal(flash::Error),
    Utf8Error(Utf8Error),
    NoSuchKey,
    ValueTooLong,
    NoValue,
}

impl From<flash::Error> for FlashError {
    fn from(err: flash::Error) -> Self {
        FlashError::Internal(err)
    }
}

impl From<core::str::Utf8Error> for FlashError {
    fn from(err: core::str::Utf8Error) -> Self {
        FlashError::Utf8Error(Utf8Error(err))
    }
}

/// Write key-value data to flash storage for persistence and read it from there again.
pub struct Config<'a> {
    flash: Flash<'a, FLASH, Async, FLASH_SIZE>,
}

impl<'a> Config<'a> {
    /// Initialize the flash module on the RP2040.
    pub fn init(flash: FLASH, dma_ch0: DMA_CH0) -> Self {
        let flash = embassy_rp::flash::Flash::<_, Async, FLASH_SIZE>::new(flash, dma_ch0);

        Self { flash }
    }

    /// Get the position on flash for a configuration key.
    fn get_config_cursor(&self, key: &str) -> Option<u32> {
        match key {
            SSID_KEY => Some(SSID_CONFIG),
            SSID_PW_KEY => Some(SSID_PW_CONFIG),
            MQTT_KEY => Some(MQTT_CONFIG),
            CLIENT_ID_KEY => Some(CLIENT_ID_CONFIG),
            _ => None,
        }
    }

    /// Write a key-value pair to flash storage.
    ///
    /// The key must be one of the predefined ones and the value length can not be longer than
    /// `ENTRY_SIZE`.
    ///
    /// Before writing, the whole sector must be erased, otherwise garbage writes can occur. For
    /// that reason `ENTRY_SIZE` is set to the ERASE_SIZE of the RP2040 (4096 bits), otherwise we
    /// would get misaligned erasures.
    ///
    /// Each key-vaue pair is encoded as [<value_length>,<value>], value_length being one byte
    /// long.
    pub fn write_config(&mut self, key: &str, val: &str) -> Result<(), FlashError> {
        let entry_cursor = self.get_config_cursor(key).ok_or(FlashError::NoSuchKey)?;
        let val_data = val.as_bytes();

        if val_data.len() > ENTRY_SIZE as usize {
            error!(
                "data of length {} for {} is longer than the max entry size of {}",
                val_data.len(),
                key,
                ENTRY_SIZE
            );
            return Err(FlashError::ValueTooLong);
        }

        self.flash
            .blocking_erase(entry_cursor, entry_cursor + ERASE_SIZE as u32)?;
        self.flash
            .blocking_write(entry_cursor, &[val_data.len() as u8])?; // value length
        self.flash.blocking_write(entry_cursor + 1, val_data)?;

        Ok(())
    }

    /// Read the value of a key from flash storage.
    pub fn read_config<'b>(
        &mut self,
        key: &str,
        buffer: &'b mut [u8; ENTRY_SIZE as usize],
    ) -> Result<&'b str, FlashError> {
        let entry_cursor = self.get_config_cursor(key).ok_or(FlashError::NoSuchKey)?;

        let mut length_buffer: [u8; 1] = [0; 1];
        self.flash.blocking_read(entry_cursor, &mut length_buffer)?;
        let length = length_buffer[0] as usize;

        if length < 1 || length > ENTRY_SIZE as usize {
            error!("there is no valid value for {}", key);
            return Err(FlashError::NoValue);
        }

        self.flash.blocking_read(entry_cursor + 1, buffer)?;
        let val = core::str::from_utf8(&buffer[0..length])?;

        Ok(val)
    }
}
