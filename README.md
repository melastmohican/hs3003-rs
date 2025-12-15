# hs3003

Platform-agnostic Rust driver for the Renesas HS3003 temperature and humidity sensor.

[![crates.io](https://img.shields.io/crates/v/hs3003.svg)](https://crates.io/crates/hs3003)
[![Docs](https://docs.rs/hs3003/badge.svg)](https://docs.rs/hs3003)
[![Verify publish readiness](https://github.com/melastmohican/hs3003-rs/actions/workflows/verify-publish.yml/badge.svg)](https://github.com/melastmohican/hs3003-rs/actions/workflows/verify-publish.yml)

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

**CI / Verify publish readiness**

The repository includes a GitHub Actions workflow `verify-publish.yml` that runs the checks used before publishing to crates.io: build, tests, formatting check, clippy (deny warnings), documentation build, and a `cargo publish --dry-run`. The workflow also attempts cross-target builds for common embedded targets (`thumbv6m-none-eabi`, `thumbv8m.main-none-eabihf`, and `riscv32imc-unknown-none-elf`).

- **Badge:** The badge near the top links to the workflow runs and shows the latest status.
- **Interpreting failures:**
	- If the `verify` job fails, the failure is likely in unit tests, formatting, clippy lints, or doc generation — inspect the job logs for the failing step.
	- If the `embedded-build` matrix job fails for a specific target, the logs will show whether the failure is due to missing toolchain (linker) or a build error in the example code. The CI installs `gcc-arm-none-eabi` for ARM targets; RISC-V toolchain installation is attempted but may require a prebuilt toolchain or container on CI runners.
	- For persistent cross-toolchain failures, consider adding a prebuilt toolchain container or customizing the workflow to install the exact toolchain used locally.

You can run the same checks locally before pushing:

```bash
cargo build --release
cargo test --all --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps
cargo publish --dry-run
```

### Example

```rust
# use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
# use embedded_hal_mock::eh1::delay::NoopDelay;
use hs3003::Hs3003;

# let expectations = [
#     I2cTransaction::write(0x44, vec![0x00]),
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

A complete Raspberry Pi Pico 2 (RP2350) example is available at `examples/rp235x/`.

- Build the example for the RP2350 (Cortex-M33) target:

```bash
cd examples/rp235x
cargo build --target thumbv8m.main-none-eabihf
```

- Run or flash the example using the runner configured in `examples/rp235x/.cargo/config.toml` (for example `probe-rs`):

```bash
cd examples/rp235x
cargo run --target thumbv8m.main-none-eabihf
```

See `examples/rp235x/.cargo/config.toml` and `examples/rp235x/build.rs` for per-example target configuration and linker scripts.

#### ESP32-C3

A complete ESP32-C3 example application is provided in the repository at `examples/esp32c3/`.

- Build the example for the ESP32-C3 (RISC‑V) target:

```bash
cd examples/esp32c3
cargo build --target riscv32imc-unknown-none-elf
```

- Run or flash the example to hardware using the runner configured in `examples/esp32c3/.cargo/config.toml` (for example, `probe-rs`, `espflash` or `cargo run` if your runner is set up):

```bash
# from repository root or from the example directory
cd examples/esp32c3
cargo run --target riscv32imc-unknown-none-elf
```

See `examples/esp32c3/.cargo/config.toml` and `examples/esp32c3/rust-toolchain.toml` for per-example target and runner configuration.

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
