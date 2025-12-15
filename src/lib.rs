#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(warnings)]

//! Platform-agnostic Rust driver for the Renesas HS3003 temperature and humidity sensor.
//!
//! This driver uses the `embedded-hal` traits to provide a hardware-independent interface
//! to the HS3003 sensor. It supports reading both temperature and humidity measurements
//! over I2C.

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

/// Default I2C address of the HS3003 sensor
pub const HS3003_I2C_ADDRESS: u8 = 0x44;

/// Measurement settling time in microseconds
const MEASUREMENT_TIME_US: u32 = 100_000; // 100ms

/// HS3003 temperature and humidity sensor driver
#[derive(Debug)]
pub struct Hs3003<I2C> {
    i2c: I2C,
    address: u8,
}

/// Measurement result containing temperature and humidity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measurement {
    /// Temperature in degrees Celsius
    pub temperature: f32,
    /// Relative humidity in percent
    pub humidity: f32,
}

/// Errors that can occur when interacting with the sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error<E> {
    /// I2C bus error
    I2c(E),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}

impl<I2C, E> Hs3003<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Creates a new HS3003 driver instance with the default I2C address (0x44)
    ///
    /// # Arguments
    ///
    /// * `i2c` - An I2C interface implementing the `embedded_hal::i2c::I2c` trait
    ///
    /// # Example
    ///
    /// ```
    /// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    /// # use hs3003::Hs3003;
    /// # let i2c = I2cMock::new(&[]);
    /// let sensor = Hs3003::new(i2c);
    /// # let mut i2c = sensor.destroy();
    /// # i2c.done();
    /// ```
    pub fn new(i2c: I2C) -> Self {
        Self::new_with_address(i2c, HS3003_I2C_ADDRESS)
    }

    /// Creates a new HS3003 driver instance with a custom I2C address
    ///
    /// # Arguments
    ///
    /// * `i2c` - An I2C interface implementing the `embedded_hal::i2c::I2c` trait
    /// * `address` - Custom I2C address for the sensor
    ///
    /// # Example
    ///
    /// ```
    /// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    /// # use hs3003::Hs3003;
    /// # let i2c = I2cMock::new(&[]);
    /// let sensor = Hs3003::new_with_address(i2c, 0x44);
    /// # let mut i2c = sensor.destroy();
    /// # i2c.done();
    /// ```
    pub fn new_with_address(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Triggers a measurement and reads temperature and humidity
    ///
    /// This function:
    /// 1. Sends a measurement request to the sensor
    /// 2. Waits for the measurement to complete (100ms)
    /// 3. Reads the raw data from the sensor
    /// 4. Converts the raw data to temperature and humidity values
    ///
    /// # Arguments
    ///
    /// * `delay` - A delay provider implementing `embedded_hal::delay::DelayNs`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Measurement` with temperature and humidity values,
    /// or an `Error` if the operation fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    /// # use embedded_hal_mock::eh1::delay::NoopDelay;
    /// # use hs3003::Hs3003;
    /// # let expectations = [
    /// #     I2cTransaction::write(0x44, vec![0x00]),
    /// #     I2cTransaction::read(0x44, vec![0x1F, 0xFF, 0x66, 0x64]),
    /// # ];
    /// # let i2c = I2cMock::new(&expectations);
    /// # let mut delay = NoopDelay::new();
    /// let mut sensor = Hs3003::new(i2c);
    /// let measurement = sensor.read(&mut delay)?;
    /// // Use measurement.temperature and measurement.humidity
    /// # let mut i2c = sensor.destroy();
    /// # i2c.done();
    /// # Ok::<(), hs3003::Error<embedded_hal::i2c::ErrorKind>>(())
    /// ```
    pub fn read<D>(&mut self, delay: &mut D) -> Result<Measurement, Error<E>>
    where
        D: DelayNs,
    {
        // Trigger measurement by writing to the sensor
        self.i2c.write(self.address, &[0x00])?;

        // Wait for measurement to complete
        delay.delay_us(MEASUREMENT_TIME_US);

        // Read 4 bytes of data
        let mut buffer = [0u8; 4];
        self.i2c.read(self.address, &mut buffer)?;

        // Parse the measurement
        Ok(Self::parse_measurement(&buffer))
    }

    /// Destroys the driver and returns the I2C interface
    ///
    /// This allows the I2C bus to be reused for other devices.
    ///
    /// # Example
    ///
    /// ```
    /// # use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    /// # use hs3003::Hs3003;
    /// # let i2c = I2cMock::new(&[]);
    /// let sensor = Hs3003::new(i2c);
    /// let mut i2c = sensor.destroy();
    /// # i2c.done();
    /// ```
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

// Separate impl block without trait bounds for parsing (allows testing)
impl<I2C> Hs3003<I2C> {
    /// Parses raw sensor data into a Measurement
    ///
    /// The HS3003 returns 4 bytes:
    /// - Bytes 0-1: Humidity data (14 bits, upper 2 bits are status)
    /// - Bytes 2-3: Temperature data (14 bits, lower 2 bits are unused)
    ///
    /// Humidity calculation: (raw_value / 16383) * 100
    /// Temperature calculation: ((raw_value / 16383) * 165) - 40
    fn parse_measurement(data: &[u8; 4]) -> Measurement {
        // Extract humidity from first two bytes (top 14 bits)
        let humidity_raw = u16::from_be_bytes([data[0] & 0x3F, data[1]]);
        let humidity = (f32::from(humidity_raw) / 16383.0) * 100.0;

        // Extract temperature from last two bytes (shift right 2 bits for 14-bit value)
        let temp_raw = u16::from_be_bytes([data[2], data[3]]) >> 2;
        let temperature = ((f32::from(temp_raw) / 16383.0) * 165.0) - 40.0;

        Measurement {
            temperature,
            humidity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_measurement_typical() {
        // Test with typical values
        // Humidity: 50% RH -> raw value = 8191 (0x1FFF)
        // Temperature: 25°C -> raw value = 6553 (0x1999)
        let data = [
            0x1F, 0xFF, // Humidity: 50% (with status bits clear)
            0x66, 0x64, // Temperature: 25°C (shifted left 2 bits)
        ];

        let measurement = Hs3003::<()>::parse_measurement(&data);

        // Temperature: (6553 / 16383.0) * 165.0 - 40.0 = 26.0°C
        // Allow small floating point error
        assert!((measurement.humidity - 50.0).abs() < 0.1);
        assert!(
            (measurement.temperature - 26.0).abs() < 0.5,
            "Temperature was {}",
            measurement.temperature
        );
    }

    #[test]
    fn test_parse_measurement_min_max() {
        // Test minimum values (0% RH, -40°C)
        let data_min = [0x00, 0x00, 0x00, 0x00];
        let measurement_min = Hs3003::<()>::parse_measurement(&data_min);
        assert!((measurement_min.humidity - 0.0).abs() < 0.1);
        assert!((measurement_min.temperature - (-40.0)).abs() < 0.5);

        // Test maximum values (100% RH, 125°C)
        let data_max = [0xFF, 0xFF, 0xFF, 0xFC];
        let measurement_max = Hs3003::<()>::parse_measurement(&data_max);
        assert!((measurement_max.humidity - 100.0).abs() < 0.1);
        assert!((measurement_max.temperature - 125.0).abs() < 0.5);
    }

    #[test]
    fn test_default_address() {
        assert_eq!(HS3003_I2C_ADDRESS, 0x44);
    }
}
