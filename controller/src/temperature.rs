use embassy_rp::{
    adc::{Adc, Async, Channel, Config, Error, InterruptHandler},
    bind_interrupts,
    peripherals::{ADC, ADC_TEMP_SENSOR},
};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

/// Convert the raw temperature sensor reading into degrees celsius.
///
/// Taken from the [embassy examples](https://github.com/embassy-rs/embassy/blob/b6fc682117a41e8e63a9632e06da5a17f46d9ab0/examples/rp/src/bin/adc.rs#L43).
fn convert_to_celsius(raw_temp: u16) -> f32 {
    // According to chapter 4.9.5. Temperature Sensor in RP2040 datasheet
    let temp = 27.0 - (raw_temp as f32 * 3.3 / 4096.0 - 0.706) / 0.001721;
    let sign = if temp < 0.0 { -1.0 } else { 1.0 };
    let rounded_temp_x10: i16 = ((temp * 10.0) + 0.5 * sign) as i16;
    (rounded_temp_x10 as f32) / 10.0
}

pub struct Temperature<'a> {
    adc: Adc<'a, Async>,
    ts: Channel<'a>,
}

impl<'a> Temperature<'a> {
    /// Initialize the temperature sensor.
    pub fn new(adc_pin: ADC, adc_temp_pin: ADC_TEMP_SENSOR) -> Self {
        let adc = Adc::new(adc_pin, Irqs, Config::default());
        let ts = Channel::new_temp_sensor(adc_temp_pin);

        Self { adc, ts }
    }

    /// Read from the temperature sensor and return the value in degree celsius.
    pub async fn read(&mut self) -> Result<f32, Error> {
        let temp = self.adc.read(&mut self.ts).await?;
        let temp = convert_to_celsius(temp);

        Ok(temp)
    }
}
