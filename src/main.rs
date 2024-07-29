use std::thread;
use std::time::Duration;
use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::Hertz;
use esp_idf_svc::hal::uart;
use anyhow::{bail, Result};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;
    log::info!("Hello, world!");


    let mut modem_poweron = PinDriver::output(pins.gpio23)?;
    let mut modem_powerkey = PinDriver::output(pins.gpio4)?;

    modem_poweron.set_level(Level::High)?;

    modem_powerkey.set_level(Level::High)?;
    thread::sleep(Duration::from_millis(100));
    modem_powerkey.set_level(Level::Low)?;
    thread::sleep(Duration::from_millis(1000));
    modem_powerkey.set_level(Level::High)?;

    let mut uart: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio27,
        pins.gpio26,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &uart::config::Config::default().baudrate(Hertz(115_200)),
    ).unwrap();
    thread::sleep(Duration::from_millis(5000));

    Ok(())
}
