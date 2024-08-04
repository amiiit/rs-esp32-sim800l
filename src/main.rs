use std::io::stdin;
use std::{io, thread};
use std::time::Duration;
use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::Hertz;
use esp_idf_svc::hal::uart;
use anyhow::{bail, Result};
use esp_idf_svc::io::Write;
use esp_idf_svc::sys::esp;

use embassy_executor::Spawner;
use embassy_time::{Timer};


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

    let mut sim800l: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio27,
        pins.gpio26,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &uart::config::Config::default().baudrate(Hertz(115_200)),
    ).unwrap();


    unsafe {
        esp!(esp_idf_svc::sys::uart_driver_install(0, 512, 512, 10, std::ptr::null_mut(), 0)).unwrap();
        esp_idf_svc::sys::esp_vfs_dev_uart_use_driver(0);
    }

    loop {
        let mut buffer = String::new();
        let read_result = io::stdin().read_line(&mut buffer);
        match read_result {
            Ok(count) => {
                log::info!("Echo {}", buffer);
            }
            Err(err) => {
                log::error!("{}", err)
            }
        }
    }
}
