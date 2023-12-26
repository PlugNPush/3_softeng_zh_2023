//! Read the temperature at regular intervals and publish that data via MQTT:
//! Configure the microcontroller with commands sent across an usb-serial connection.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::str::FromStr;

use cyw43::Control;
use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_net::Stack;
use embassy_net_wiznet::Device;
use embassy_time::Timer;
use flash::{Config, FlashError};
use mqtt_protocol::publish::MAX_TOPIC_LENGTH;
use static_cell::make_static;
use temperature::Temperature;
use usb_serial::init_usb;
use wifi::connect;
use {defmt_rtt as _, panic_probe as _};

use crate::{byte_handler::handle_bytes, mqtt::BUFFER_SIZE, wifi::init_wifi};

mod byte_handler;
mod cmd_handlers;
mod flash;
mod mqtt;
mod temperature;
mod usb_serial;
mod wifi;

async fn connect_wifi<'a>(
    config: &mut Config<'a>,
    wifi: &mut Control<'a>,
    net_stack: &'a Stack<Device<'a>>,
) -> Result<(), FlashError> {
    let mut ssid_buffer = [0; flash::ENTRY_SIZE as usize];
    let ssid = config.read_config(flash::SSID_KEY, &mut ssid_buffer)?;
    let mut ssid_pw_buffer = [0; flash::ENTRY_SIZE as usize];
    let ssid_pw = config.read_config(flash::SSID_PW_KEY, &mut ssid_pw_buffer)?;

    if !ssid.is_empty() {
        connect(wifi, net_stack, ssid, ssid_pw).await;
    }
    Ok(())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    Timer::after_millis(100).await;
    let mut config = Config::init(p.FLASH, p.DMA_CH0);

    let (mut wifi, net_stack) = init_wifi(
        spawner, p.PIN_23, p.PIN_24, p.PIN_25, p.PIN_29, p.PIO0, p.DMA_CH1,
    )
    .await;
    match connect_wifi(&mut config, &mut wifi, net_stack).await {
        Ok(_) => info!("Connected to WiFi."),
        Err(_) => error!("Failed to connect to WiFi"),
    }

    let rx_buffer = make_static!([0u8; BUFFER_SIZE]);
    let tx_buffer = make_static!([0u8; BUFFER_SIZE]);

    let client_id_buffer = make_static!([0; flash::ENTRY_SIZE as usize]);
    let client_id = config
        .read_config(flash::CLIENT_ID_KEY, client_id_buffer)
        .unwrap_or("tomato");

    let mqtt_ip_buffer = make_static!([0; flash::ENTRY_SIZE as usize]);
    let mqtt_ip = config
        .read_config(flash::MQTT_KEY, mqtt_ip_buffer)
        .unwrap_or("127.0.0.1");

    let mut mqtt = mqtt::Client::new(client_id, net_stack, rx_buffer, tx_buffer);
    let host_addr = embassy_net::IpAddress::from_str(mqtt_ip)
        .unwrap_or(embassy_net::IpAddress::v4(127, 0, 1, 1));
    let mut temps = Temperature::new(p.ADC, p.ADC_TEMP_SENSOR);

    let (mut usb, class) = init_usb(p.USB);
    let usb_fut = usb.run();

    let cmd_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            let _ = handle_bytes(class, &mut config).await;
            info!("Disconnected");
        }
    };

    let temp_fut = async {
        const TOPIC_PREFIX: &str = "temps/";
        let mut topic_buffer = [0u8; MAX_TOPIC_LENGTH - TOPIC_PREFIX.len()];
        topic_buffer[..TOPIC_PREFIX.len()].copy_from_slice(TOPIC_PREFIX.as_bytes());

        let client_id_bytes = client_id.as_bytes();
        topic_buffer[TOPIC_PREFIX.len()..TOPIC_PREFIX.len() + client_id_bytes.len()]
            .copy_from_slice(client_id_bytes);

        let topic_full =
            core::str::from_utf8(&topic_buffer[..TOPIC_PREFIX.len() + client_id_bytes.len()])
                .unwrap_or("temps/tomato");

        loop {
            match temps.read().await {
                Ok(temp) => {
                    info!("publishing temperature data...");
                    if let Err(e) = mqtt
                        .publish(host_addr, topic_full, &temp.to_le_bytes())
                        .await
                    {
                        error!("failed to publish temperature data: {}", e);
                    }
                }
                Err(e) => error!("failed to read temperature: {}", e),
            }

            Timer::after_secs(10).await;
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join3(usb_fut, cmd_fut, temp_fut).await;
}
