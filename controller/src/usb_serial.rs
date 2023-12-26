use embassy_rp::{
    bind_interrupts,
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    Builder, Config, UsbDevice,
};
use static_cell::make_static;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

type SerialDevice = UsbDevice<'static, Driver<'static, USB>>;
type DeviceClass = CdcAcmClass<'static, Driver<'static, USB>>;

/// Initialize the USB serial interface and return the device and CdcAcm class (Abstract Control Model).
pub fn init_usb(usb: USB) -> (SerialDevice, &'static mut DeviceClass) {
    let driver = Driver::new(usb, Irqs);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-serial");
    config.serial_number = Some("123456789");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility.
    // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let device_descriptor = make_static!([0; 256]);
    let config_descriptor = make_static!([0; 256]);
    let bos_descriptor = make_static!([0; 256]);
    let control_buf = make_static!([0; 64]);

    let state = make_static!(State::new());

    let mut builder = Builder::new(
        driver,
        config,
        device_descriptor,
        config_descriptor,
        bos_descriptor,
        control_buf,
    );

    let class = make_static!(CdcAcmClass::new(&mut builder, state, 64));
    let usb = builder.build();

    (usb, &mut *class)
}
