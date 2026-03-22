#![no_std]

//! This is a simple driver for ST's `iis2mdc` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust,ignore
//! let mut sensor = Iis2mdc::new(&mut i2c).unwrap();
//! ```
//!
//! To configure the sensor, use the high-level methods:
//!
//! ```rust,ignore
//! sensor.set_odr(&mut i2c, Odr::Hz50).unwrap();
//! sensor.set_comp_temp_en(&mut i2c, true).unwrap();
//! ```
//!
//! # Reference
//!
//! - [Sensor page](https://www.st.com/en/mems-and-sensors/iis2mdc.html)
//! - [Datasheet](https://www.st.com/resource/en/datasheet/iis2mdc.pdf)

pub mod registers;

pub use registers::{
    Register,
    cfg_reg_a::{CfgRegA, CfgRegAConfig, Mode, Odr},
    cfg_reg_b::{CfgRegB, CfgRegBConfig},
    cfg_reg_c::{CfgRegC, CfgRegCConfig},
    int_ctrl_reg::{IntCtrlReg, IntCtrlRegConfig},
    int_source_reg::{IntSource, IntSourceReg},
    int_ths::IntThsConfig,
    offset::HardIronOffsetConfig,
    out_mag::{MagValue, Magnetometer},
    out_temp::{TempValue, Temperature},
    status_reg::{Status, StatusReg},
};

use embedded_hal::i2c::I2c;

/// Datasheet write address for the device. (1Eh)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x1Eu8;

/// Errors for the IIS2MDC driver.
#[derive(Debug, Copy, Clone, defmt::Format)]
pub enum Error<E> {
    /// I2C bus error.
    I2c(E),
    /// Invalid device found (WHO_AM_I mismatch).
    InvalidDevice(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Self::I2c(error)
    }
}

/// Driver for the IIS2MDC sensor.
pub struct Iis2mdc {
    /// I2C address.
    pub address: u8,
}

impl Iis2mdc {
    /// Create a new driver instance with the default I2C address (0x1E).
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        Self::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    /// Create a new driver instance with a specific I2C address.
    pub fn new_with_address<I2C>(i2c: &mut I2C, address: u8) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8];
        i2c.write_read(address, &[Register::WhoAmI.addr()], &mut buffer)?;

        if buffer[0] != 0x40 {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        let sensor = Self { address };

        // Set sane defaults: BDU
        sensor.set_bdu(i2c, true)?;

        Ok(sensor)
    }

    pub(crate) fn read_reg<I2C>(&self, i2c: &mut I2C, reg: Register) -> Result<u8, I2C::Error>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8];
        i2c.write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    pub(crate) fn write_reg<I2C>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        i2c.write(self.address, &[reg.addr(), value])
    }

    pub(crate) fn modify_reg<I2C, F>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        f: F,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        F: FnOnce(u8) -> u8,
    {
        let val = self.read_reg(i2c, reg)?;
        self.write_reg(i2c, reg, f(val))
    }
}
