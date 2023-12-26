use cyw43::{Control, ControlError};
use cyw43_pio::PioSpi;
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_net_wiznet::Device;
use embassy_rp::{
    bind_interrupts,
    clocks::RoscRng,
    gpio::{Level, Output},
    peripherals::{DMA_CH1, PIN_23, PIN_24, PIN_25, PIN_29, PIO0},
    pio::{InterruptHandler, Pio},
};
use embassy_time::Timer;
use rand::RngCore;
use static_cell::make_static;

const MAX_WIFI_TRIES: u8 = 3;
const MAX_DHCP_TRIES: u8 = 20;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH1>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

// Initialise the wwifi chip of the RP2040 and return the control interface as well as the network
// stack.
pub async fn init_wifi(
    spawner: Spawner,
    pin_23: PIN_23,
    pin_24: PIN_24,
    pin_25: PIN_25,
    pin_29: PIN_29,
    pio0: PIO0,
    dma_ch1: DMA_CH1,
) -> (Control<'static>, &'static Stack<Device<'static>>) {
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(pin_23, Level::Low);
    let cs = Output::new(pin_25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        pin_24,
        pin_29,
        dma_ch1,
    );

    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    let mut rng = RoscRng;
    let seed = rng.next_u64();

    // Init network stack
    let stack = &*make_static!(Stack::new(
        net_device,
        config,
        make_static!(StackResources::<2>::new()),
        seed
    ));

    unwrap!(spawner.spawn(net_task(stack)));

    (control, stack)
}

/// Connect to a WiFi network using WPA2 and try to get an IP from DHCP (IPv4).
pub async fn connect<'a>(
    wifi: &mut Control<'a>,
    net_stack: &Stack<cyw43::NetDriver<'a>>,
    ssid: &str,
    pw: &str,
) {
    info!("trying to connect to {}...", ssid);

    let mut idx = 0;
    let mut status: Result<(), ControlError> = Err(ControlError { status: 0 });

    while idx < MAX_WIFI_TRIES && status.is_err() {
        status = wifi.join_wpa2(ssid, pw).await;
        Timer::after_millis(100).await;

        idx += 1;
    }

    if let Err(e) = status {
        error!("WiFi connection failed: {}", e.status);
    } else {
        info!("waiting for DHCP...");

        idx = 0;
        while idx < MAX_DHCP_TRIES && !net_stack.is_config_up() {
            Timer::after_millis(1000).await;
            idx += 1;
        }

        if net_stack.is_config_up() {
            info!("DHCP is up!");
        } else {
            error!("Failed to get an IP from DHCP");
        }
    }
}
