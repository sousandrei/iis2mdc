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

## <a name="usage"></a> Usage 👀

Check out the `examples` folder for simple implementation

To declare a sensor is pretty simple:

```rust
let mut sensor = Iis2mdc::new(&mut i2c).unwrap();
```

To configure the sensor, use the high-level methods:

```rust
use iis2mdc::{CfgRegAConfig, Odr};

sensor.set_odr(&mut i2c, Odr::Hz50).unwrap();
sensor.set_comp_temp_en(&mut i2c, true).unwrap();
```

To read measurements:

```rust
use iis2mdc::Magnetometer;

let mag = sensor.get_magnetometer(&mut i2c).unwrap();
println!("Mag: {:?}", mag.as_ut());
```

## <a name="help-wanted"></a> Help wanted 🤝

All contributions are welcome!

If you are using or plan to use this create don't hesitate to open an issue or a PR.

## <a name="license"></a> License

See [LICENSE](https://github.com/sousandrei/firesquid/blob/master/LICENSE) for more details.
