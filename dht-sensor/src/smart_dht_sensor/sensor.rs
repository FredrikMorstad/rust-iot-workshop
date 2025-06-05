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
        let mut server = EspHttpServer::new(&conf)?;
        server.fn_handler(
            "/",
            Method::Get,
            |request| -> core::result::Result<(), EspIOError> {
                let mut response = request.into_ok_response()?;
                let res_text = "alive";
                response.write_all(res_text.as_bytes())?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
        self.sensor.read()
    }
}
