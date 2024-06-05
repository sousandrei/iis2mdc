//! This is a simple driver for ST's `iis2mdc` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust
//! let sensor = Iis2mdc::new(&mut i2c).unwrap()
//! ```
//!
//! All registers have the bits addressed by their function, for example here se set the `BOOT` register in the `CTRL_3C` register to `1`
//!
//! ```rust
//! sensor.cfg_reg_a.set_reboot(i2c, true).unwrap();
//! ```
//!
//! For bits that operate together, they have their custom type abstracted. For example, to set the accelerometer data rate you have to operate 4 bits. But here you just have to specify your desired data rate and the driver takes care of it.
//!
//! ```rust
//! // Sets the following bits
//! // ODR_0 to 1
//! // ODR_1 to 0
//!
//! sensor
//!     .cfg_reg_a
//!     .set_data_rate(i2c, iis2mdc::cfg_reg_a::Odr::Hz50)
//!     .unwrap();
//! ```
//!
//! # Reference
//!
//!- [Sensor page](https://www.st.com/en/mems-and-sensors/iis2mdc.html)
//!- [Datasheet](https://www.st.com/resource/en/datasheet/iis2mdc.pdf)

#![no_std]

pub mod cfg_reg_a;
pub mod cfg_reg_b;
pub mod cfg_reg_c;

use cfg_reg_a::CfgRegA;
use cfg_reg_b::CfgRegB;
use cfg_reg_c::CfgRegC;

/// Datasheed write address for the device. (3Ch) ??
/// My sample only answers on 0x1e
pub const I2C_ADDRESS: u8 = 0x1eu8;

trait Register {
    fn read<I2C>(&self, i2c: &mut I2C, reg_addr: u8) -> Result<u8, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut data: [u8; 1] = [0];
        i2c.write_read(I2C_ADDRESS, &[reg_addr], &mut data)?;
        Ok(data[0])
    }

    fn write<I2C>(&self, i2c: &mut I2C, reg_addr: u8, bits: u8) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        i2c.write(I2C_ADDRESS, &[reg_addr, bits])
    }
}

pub struct Iis2mdc {
    pub cfg_reg_a: CfgRegA,
    pub cfg_reg_b: CfgRegB,
    pub cfg_reg_c: CfgRegC,
}

impl Iis2mdc {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Iis2mdc, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut registers = [0u8; 3];
        i2c.write_read(I2C_ADDRESS, &[0x60], &mut registers)?;

        let cfg_reg_a = CfgRegA::new(registers[0]);
        let cfg_reg_b = CfgRegB::new(registers[1]);
        let cfg_reg_c = CfgRegC::new(registers[2]);

        let iis2mdc = Iis2mdc {
            cfg_reg_a,
            cfg_reg_b,
            cfg_reg_c,
        };

        Ok(iis2mdc)
    }

    pub fn get_temperature<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut measurements = [0u8; 2];
        i2c.write_read(I2C_ADDRESS, &[0x6e], &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    pub fn get_measurements<I2C>(&mut self, i2c: &mut I2C) -> Result<[f64; 3], I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let mut measurements = [0u8; 6];
        i2c.write_read(I2C_ADDRESS, &[0x68], &mut measurements)?;

        let raw_mag_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_mag_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_mag_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);

        let mag_x = raw_mag_x as f64;
        let mag_y = raw_mag_y as f64;
        let mag_z = raw_mag_z as f64;

        Ok([mag_x, mag_y, mag_z])
    }
}
