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
let sensor = Iis2mdc::new(&mut i2c).unwrap()
```

All registers have the bits addressed by their function, for example here se set the `BOOT` register in the `CTRL_3C` register to `1`

```rust
sensor.cfg_reg_a.set_reboot(i2c, true).unwrap();
```

For bits that operate together, they have their custom type abstracted. For example, to set the accelerometer data rate you have to operate 4 bits. But here you just have to specify your desired data rate and the driver takes care of it.

```rust
// Sets the following bits
// ODR_0 to 1
// ODR_1 to 0

sensor
    .cfg_reg_a
    .set_data_rate(i2c, iis2mdc::cfg_reg_a::Odr::Hz50)
    .unwrap();
```

## <a name="help-wanted"></a> Help wanted 🤝

All contributions are welcome!

If you are using or plan to use this create don't hesitate to open an issue or a PR.

Multiple registers are yet to be referenced!

## <a name="license"></a> License

See [LICENSE](https://github.com/sousandrei/firesquid/blob/master/LICENSE) for more details.
