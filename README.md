[![Build Status](https://github.com/sousandrei/iis2mdc/workflows/Main/badge.svg)](https://github.com/sousandrei/iis2mdc/actions)
[![Docs.rs](https://docs.rs/iis2mdc/badge.svg)](https://docs.rs/iis2mdc)
[![Crates.io](https://img.shields.io/crates/v/iis2mdc)](https://crates.io/crates/iis2mdc)

## Table of Contents

- [About the project](#about)
- [Usage](#usage)
- [Help Wanted](#help-wanted)
- [License](#license)

## <a name="about"></a> About the Project 📃

This is a simple driver for ST's `iis2mdc` sensor.

Documentation for that sensor can be found at ST's website

- [Sensor page](https://www.st.com/en/mems-and-sensors/iis2mdc.html)
- [Datasheet](https://www.st.com/resource/en/datasheet/iis2mdc.pdf)

## <a name="usage"></a> Usage

The driver supports I2C and standard four-wire SPI through `embedded-hal`.
The bus and, for SPI, chip-select configuration are supplied by the caller.

### I2C

```rust
use iis2mdc::Iis2mdc;

let sensor = Iis2mdc::new(&mut i2c).unwrap();
```

Feature-level APIs are exposed through traits:

```rust
use iis2mdc::{Configuration, Iis2mdc, Odr};

let sensor = Iis2mdc::new(&mut i2c).unwrap();
sensor.set_odr(&mut i2c, Odr::Hz50).unwrap();
sensor.set_comp_temp_en(&mut i2c, true).unwrap();
```

Read measurements through the output feature traits:

```rust
use iis2mdc::Magnetometer;

let mag = sensor.get_magnetometer(&mut i2c).unwrap();
println!("Mag: {:?}", mag.as_ut());
```

### SPI

Wrap an already-configured `embedded_hal::spi::SpiDevice`. The caller selects
SPI mode, clock frequency, electrical setup, and chip-select behavior according
to the datasheet and HAL.

```rust
use iis2mdc::{Iis2mdc, SpiDeviceBus};

let mut spi = SpiDeviceBus::new(spi_device);
let sensor = Iis2mdc::new_spi(&mut spi).unwrap();
```

Four-wire SPI is supported. Three-wire SPI is not currently supported. Reset
and reboot operations remain explicit and must be followed by the datasheet's
required delays when used.

See the `examples` directory for a board-level I2C example.

## <a name="help-wanted"></a> Help wanted 🤝

All contributions are welcome!

If you are using or plan to use this create don't hesitate to open an issue or a PR.

## <a name="license"></a> License

See [LICENSE](https://github.com/sousandrei/firesquid/blob/master/LICENSE) for more details.
