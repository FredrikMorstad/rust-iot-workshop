# rust-iot-workshop 🦀
In this workshop we will create a "smart" humidity and temperature sensor. The goal is to become familiar with some embedded programming in rust on ESP controllers. 

## Project
todo!

## Part 0: Setup
To participate in this workshop you need three components: A esp microcontroller, a DHT sensor and a breadboard pluss wires. 

#### 1. Controllers
We have two types of esp controllers to use, either the ESP32 D1 or ESP3286
<details>
<summary>ESP32 D1</summary>
<img src="./assets/a1.png" alt="esp32 D1" width="200"/> <br>

[pinout](https://lastminuteengineers.com/wemos-d1-mini-pinout-reference/)
</details>

<details>
<summary>ESP3286</summary>
<img src="./assets/esp3286.jpg" alt="esp3286" width="200"/>
</details>

#### 2. DHT-22 sensor
<details>
<summary>DHT-22 sensor</summary>
<img src="./assets/dht22.jpg" alt="dht22 sensor" width="200"/> <br>
</details>

#### 3. Breadboard and wires
<details>
<summary>breadboard and wires</summary>
<img src="./assets/breadboard-jumper-wire.jpg" alt="dht22 sensor" width="200"/> <br>
</details>

#### Setup development environment
To build and upload our application onto a esp board we need to generate our project for our chip and some tooling. First we need to setup esp on our pc:
<details>
<summary> Setup esp on your computer 💻 </summary>
To be able to work with our esp controller we need to setup our development environment. Esp has a [official book](https://docs.esp-rs.org/book/introduction.html) which explains how to work with esp controllers with rust 

### Prerequisites
To run application with standard library(std) we need ldproxy.
```
cargo install ldproxy
```

### Setup tooling for RISC-V and Xtensa Targets
This setup is also described in the book [here](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html). So feel free to check it out for a more detail description of the tooling. Setting up the tooling is a three step process:
1. Install espup
```
cargo install espup
```
2. Install dependencies
```
espup install
```
3. Setup environment variables
ESP uses some specific environment variables when building the project, these values need to be exported via the export script downloaded by espup. To avoid having to run this command 
```
. $HOME/export-esp.sh
```
each time we need change project I recommend adding a alias to your rc file. By adding this line to our rc file
```
alias get_idf='. $HOME/esp/esp-idf/export.sh'
```
we can run `get_idf` befor building a different esp project. Remember to source the shell after updating your rc file.
</details>
<br>
Once esp is setup on our computer we can try to build and run the project on our controller. I have created a template project called `dht-sensor` move into the project and run it using: <br >

```
cargo run
```
If everything worked, you should get the following output:
<details>
<summary> Output:</summary>
<img src="./assets/output-setup.png" alt="output" width="400"/> <br>
</details>

## Part 1: Getting data from the sensor (using lib)
The first challenge is to create a program that reads sensor values at an interval. The first ting we need to so is 
The first thing we need to do wire our sensor and controller on the breadboard correctly. Once that is done we can driver  

<details>
<summary>Solution 💡</summary>

```rust
use std::{thread::sleep, time::Duration};

use embedded_dht_rs::dht22::Dht22;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let delay = Delay::new(1000);

    let pin = PinDriver::input_output_od(peripherals.pins.gpio4).unwrap();

    let mut sensor = Dht22::new(pin, delay);

    loop {
        match sensor.read() {
            Ok(reading) => {
                println!("{}°C, {}% RH", reading.temperature, reading.humidity)
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }

        sleep(Duration::from_secs(1));
    }
}
```
</details>

## Part 2: Creating our own "smart" sensor
Now that we have a working DHT sensor, we can start creating our smart sensor. The general idea is to be able to access our sensor data via internet. The first step towards this is to create our own struct that receives a GPIO pin and creates a DHT instance and exposes a read method. 

<!-- todo: Create a new package in a --> 

<details> 
<summary> Solution💡</summary>

```rust
//smart_dht_sensor/sensor.rs
use embedded_dht_rs::{dht22::Dht22, SensorError, SensorReading};
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
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

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
        self.sensor.read()
    }
}
```
Then main has a simple update like so:

```rust
let mut sensor = SmartSensor::new(pin, delay);
```
</details>

## Part 3: Running our web server
Now that we have created our smart sensor package we can start making it "smart". The next step is to make our smart sensor run a http server that we can reach from our computer. Lets create a `run` method that starts a server with a simple alive endpoint on `/alive`

<details> 
<summary>Solution💡 </summary>
</details>

## Part 4: Get measurements from server

## Part 5: (Optional) Measurements SSE?
