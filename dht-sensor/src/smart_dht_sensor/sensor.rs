use embedded_dht_rs::{dht22::Dht22, SensorError, SensorReading};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use esp_idf_svc::{
    http::{
        server::{Configuration, EspHttpServer},
        Method,
    },
    io::{EspIOError, Write},
};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

pub struct SmartSensor<P: InputPin + OutputPin, D: DelayNs> {
    sensor: Dht22<P, D>,
}

impl<P: InputPin + OutputPin, D: DelayNs> SmartSensor<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self {
            sensor: Dht22::new(pin, delay),
        }
    }

    pub fn run(&mut self, port: u16) -> Result<(), EspIOError> {
        let conf = Configuration {
            http_port: port,
            ..Default::default()
        };

        let reading = self.read().unwrap();
        let shared_reading = Arc::new(Mutex::new(reading));
        let shared_reading_cloned = shared_reading.clone();

        let mut server = EspHttpServer::new(&conf)?;
        server.fn_handler(
            "/alive",
            Method::Get,
            |request| -> core::result::Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                let res_text = "alive";
                response.write_all(res_text.as_bytes())?;
                Ok(())
            },
        )?;
        server.fn_handler(
            "/measurement",
            Method::Get,
            move |request| -> core::result::Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                let reading_ref = &shared_reading_cloned;
                let reading = reading_ref.lock().unwrap();
                let res_text = format!("{}°C, {}% RH", reading.temperature, reading.humidity);
                response.write_all(res_text.as_bytes())?;
                Ok(())
            },
        )?;

        println!("running server");

        loop {
            let Ok(res) = self.read() else {
                sleep(Duration::from_millis(250));
                continue;
            };
            *shared_reading.lock().unwrap() = res;
            // let mut reading = *shared_reading.get_mut().unwrap();
            sleep(Duration::from_secs(1));
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
        self.sensor.read()
    }
}
