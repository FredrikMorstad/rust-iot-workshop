use std::{thread::sleep, time::Duration};

use anyhow::{bail, Result};
use dht_sensor::{config::APP_CONFIG, smart_dht_sensor::sensor::SmartSensor};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals},
};

use dht_sensor::wifi::wifi;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let delay = Delay::new(1000);

    let pin = PinDriver::input_output_od(peripherals.pins.gpio4).unwrap();

    let sysloop = EspSystemEventLoop::take()?;

    println!("{:?}: {:?}", APP_CONFIG.wifi_ssid, APP_CONFIG.wifi_ssid);

    let _wifi = match wifi(
        APP_CONFIG.wifi_ssid,
        APP_CONFIG.wifi_pwd,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let mut sensor = SmartSensor::new(pin, delay);
    sensor.run(4000).expect("error");

    loop {
        match sensor.read() {
            Ok(reading) => {
                println!("{}°C, {}% RH", reading.temperature, reading.humidity)
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }

        sleep(Duration::from_secs(2));
    }
}
