# hs3003

Platform-agnostic Rust driver for the Renesas HS3003 temperature and humidity sensor.

[![crates.io](https://img.shields.io/crates/v/hs3003.svg)](https://crates.io/crates/hs3003)
[![Docs](https://docs.rs/hs3003/badge.svg)](https://docs.rs/hs3003)
[![Build Status](https://github.com/yourusername/hs3003/workflows/CI/badge.svg)](https://github.com/yourusername/hs3003/actions)

## Features

- `no_std` compatible
- Platform-agnostic using `embedded-hal` traits
- Simple, ergonomic API
- Supports temperature range: -40°C to +125°C
- Supports humidity range: 0% to 100% RH
- I2C interface

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hs3003 = "0.1"
```

### Example

```rust
# use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
# use embedded_hal_mock::eh1::delay::NoopDelay;
use hs3003::Hs3003;

# let expectations = [
#     I2cTransaction::write(0x44, vec![]),
#     I2cTransaction::read(0x44, vec![0x1F, 0xFF, 0x66, 0x64]),
# ];
# let i2c = I2cMock::new(&expectations);
# let mut delay = NoopDelay::new();
// Create sensor instance with I2C interface
let mut sensor = Hs3003::new(i2c);

// Read temperature and humidity
let measurement = sensor.read(&mut delay)?;

# #[cfg(feature = "std")]
println!("Temperature: {:.2}°C", measurement.temperature);
# #[cfg(feature = "std")]
println!("Humidity: {:.2}%", measurement.humidity);
# let mut i2c = sensor.destroy();
# i2c.done();
# Ok::<(), hs3003::Error<embedded_hal::i2c::ErrorKind>>(())
```

### Platform-Specific Examples

#### Raspberry Pi Pico (RP2350/RP2040)

```rust,ignore
use embedded_hal::delay::DelayNs;
use rp235x_hal as hal;
use hs3003::Hs3003;

// Set up I2C
let mut i2c = hal::I2C::i2c0(
    peripherals.I2C0,
    sda_pin,
    scl_pin,
    100.kHz(),
    &mut resets,
    system_clock.freq(),
);

let mut sensor = Hs3003::new(i2c);
let mut delay = cortex_m::delay::Delay::new(core.SYST, system_clock.freq().to_Hz());

loop {
    match sensor.read(&mut delay) {
        Ok(measurement) => {
            // Use measurement data
        }
        Err(e) => {
            // Handle error
        }
    }
    delay.delay_ms(2000);
}
```

#### ESP32-C3

```rust,ignore
use esp_hal::{i2c::I2C, delay::Delay};
use hs3003::Hs3003;

let i2c = I2C::new(
    peripherals.I2C0,
    sda_pin,
    scl_pin,
    100.kHz(),
    &clocks,
);

let mut sensor = Hs3003::new(i2c);
let mut delay = Delay::new(&clocks);

loop {
    match sensor.read(&mut delay) {
        Ok(measurement) => {
            // Use measurement data
        }
        Err(e) => {
            // Handle error
        }
    }
    delay.delay_ms(2000);
}
```

## Sensor Information

The HS3003 is a digital temperature and humidity sensor with:
- High accuracy: ±2% RH, ±0.2°C
- Low power consumption
- I2C interface (fixed address 0x44)
- 14-bit resolution for both temperature and humidity
- Fast response time

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
