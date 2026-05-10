//! Example using the HS3003 sensor with Raspberry Pi Pico 2 (RP2350) and Embassy
//!
//! This example demonstrates how to use the hs3003 driver asynchronously 
//! using the embassy-rp crate.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::i2c::{Config, I2c, InterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_rp::bind_interrupts;
use embassy_time::{Duration, Timer};
use hs3003::Hs3003;
use panic_probe as _;
use rp235x_hal as hal;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("HS3003 Async Sensor Example for RP2350 (Embassy)");

    let p = embassy_rp::init(Default::default());

    // Configure I2C0 (GP4 = SDA, GP5 = SCL)
    let sda = p.PIN_4;
    let scl = p.PIN_5;
    let i2c = I2c::new_async(p.I2C0, scl, sda, Irqs, Config::default());

    // Create sensor instance
    let mut sensor = Hs3003::new(i2c);
    let mut delay = embassy_time::Delay;

    info!("Starting measurements...");

    loop {
        match sensor.read_async(&mut delay).await {
            Ok(measurement) => {
                info!(
                    "Temperature: {}°C, Humidity: {}%",
                    measurement.temperature, measurement.humidity
                );
            }
            Err(_) => {
                error!("Failed to read sensor");
            }
        }
        Timer::after(Duration::from_secs(2)).await;
    }
}
