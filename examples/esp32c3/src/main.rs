//! Example using the HS3003 sensor with Rust ESP Board (ESP32-C3-DevKit-RUST-1)
//!
//! This example demonstrates how to use the `hs3003` driver on an ESP32-C3 board
//! using the `esp-hal` crate. A self-contained example crate is available at
//! `examples/esp32c3/` in this repository.
//!
//! # Hardware Setup
//!
//! Connect the HS3003 sensor to the ESP32-C3 board:
//! - VCC to 3.3V
//! - GND to Ground
//! - SCL to GPIO8
//! - SDA to GPIO10
//!
//! # Building
//!
//! Build the standalone example crate from the repository root:
//!
//! ```bash
//! cd examples/esp32c3
//! cargo build --target riscv32imc-unknown-none-elf --release
//! ```
//!
//! To run or flash the example, use the runner configured in
//! `examples/esp32c3/.cargo/config.toml` (for example `probe-rs`, `espflash`,
//! or another runner). Example run command:
//!
//! ```bash
//! cd examples/esp32c3
//! cargo run --target riscv32imc-unknown-none-elf
//! ```


#![no_std]
#![no_main]

use defmt::info;
use esp_hal::esp_riscv_rt::entry;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config as I2cConfig, I2c},
};
use hs3003::Hs3003;
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Initializing HS3003 sensor...");

    let sda = peripherals.GPIO10;
    let scl = peripherals.GPIO8;

    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .unwrap()
        .with_sda(sda)
        .with_scl(scl);

    let mut delay = Delay::new();

    let mut sensor = Hs3003::new(i2c);

    info!("HS3003 Sensor Example for Rust ESP Board (ESP32-C3-DevKit-RUST-1)");

    loop {
        match sensor.read(&mut delay) {
            Ok(measurement) => {
                info!("Temperature: {}°C, Humidity: {}%", measurement.temperature, measurement.humidity);
            }
            Err(_) => {
                info!("Failed to read sensor");
            }
        }

        delay.delay_millis(2000);
    }
}
