//! This is a simple driver for ST's `iis2mdc` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust,ignore
//! let mut sensor = Iis2mdc::new(&mut i2c).unwrap();
//! ```
//! For SPI, wrap an already-configured four-wire [`embedded_hal::spi::SpiDevice`]:
//! ```rust,ignore
//! let mut spi = SpiDeviceBus::new(spi_device);
//! let mut sensor = Iis2mdc::new_spi(&mut spi).unwrap();
//! ```
//! SPI mode, clock frequency, and electrical setup are selected by the caller
//! according to the datasheet and their HAL.
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

#![cfg_attr(not(test), no_std)]

pub mod configuration;
pub mod interrupts;
pub mod magnetometer;
pub mod offsets;
pub mod registers;
pub mod spi;
pub mod status;
pub mod temperature;

pub use registers::{
    Register,
    cfg_reg_a::{CfgRegA, Mode, Odr},
    cfg_reg_b::CfgRegB,
    cfg_reg_c::CfgRegC,
    int_ctrl_reg::IntCtrlReg,
    int_source_reg::IntSourceReg,
    out_mag::OutMag,
    status_reg::StatusReg,
};

pub use configuration::Configuration;
pub use interrupts::{InterruptControl, InterruptSource, InterruptThreshold};
pub use magnetometer::{MagValue, Magnetometer};
pub use offsets::HardIronOffsets;
pub use spi::SpiDeviceBus;
pub use status::Status;
pub use temperature::{TempValue, Temperature};

use embedded_hal::i2c::I2c;
use embedded_hal::spi::SpiDevice;

/// Datasheet write address for the device. (1Eh)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x1Eu8;

/// Errors for the IIS2MDC driver.
#[derive(Debug, Copy, Clone, defmt::Format)]
pub enum Error<E> {
    /// I2C bus error.
    I2c(E),
    /// SPI bus error.
    Spi(E),
    /// Invalid device found (WHO_AM_I mismatch).
    InvalidDevice(u8),
}

/// Internal register transport used by the feature implementations.
/// Internal register transport contract implemented by I2C and SPI buses.
pub trait RegisterBus {
    type Error;

    fn read_register(
        &mut self,
        address: u8,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error>;

    fn write_register(&mut self, address: u8, register: u8, data: &[u8])
    -> Result<(), Self::Error>;
}

impl<I2C> RegisterBus for I2C
where
    I2C: I2c,
{
    type Error = I2C::Error;

    fn read_register(
        &mut self,
        address: u8,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_read(address, &[register], data)
    }

    fn write_register(
        &mut self,
        address: u8,
        register: u8,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        // IIS2MDC block writes currently require at most six data bytes.
        let mut buffer = [0u8; 7];
        buffer[0] = register;
        buffer[1..data.len() + 1].copy_from_slice(data);
        self.write(address, &buffer[..data.len() + 1])
    }
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
        let sensor = Self { address };
        let mut buffer = [0u8];
        sensor.read_regs(i2c, Register::WhoAmI, &mut buffer)?;

        if buffer[0] != 0x40 {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        // Set sane defaults: BDU
        sensor.set_bdu(i2c, true)?;

        Ok(sensor)
    }

    /// Create a driver using an already-configured SPI device.
    pub fn new_spi<SPI>(spi: &mut SpiDeviceBus<SPI>) -> Result<Self, Error<SPI::Error>>
    where
        SPI: SpiDevice<u8>,
    {
        let sensor = Self {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut buffer = [0u8];
        sensor
            .read_regs(spi, Register::WhoAmI, &mut buffer)
            .map_err(Error::Spi)?;

        if buffer[0] != 0x40 {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        sensor.set_bdu(spi, true).map_err(Error::Spi)?;
        Ok(sensor)
    }

    pub(crate) fn read_reg<I2C>(&self, i2c: &mut I2C, reg: Register) -> Result<u8, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut buffer = [0u8];
        self.read_regs(i2c, reg, &mut buffer)?;
        Ok(buffer[0])
    }

    pub(crate) fn read_regs<I2C>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        data: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        i2c.read_register(self.address, reg.addr(), data)
    }

    pub(crate) fn write_reg<I2C>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.write_regs(i2c, reg, &[value])
    }

    pub(crate) fn write_regs<I2C>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        data: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        i2c.write_register(self.address, reg.addr(), data)
    }

    pub(crate) fn modify_reg<I2C, F>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        f: F,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
        F: FnOnce(u8) -> u8,
    {
        let val = self.read_reg(i2c, reg)?;
        self.write_reg(i2c, reg, f(val))
    }

    /// Set the I2C address used by subsequent operations.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn read_regs_uses_register_as_the_starting_address() {
        let sensor = Iis2mdc {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::OutXRegL.addr()],
            vec![1, 2, 3, 4, 5, 6],
        )]);
        let mut data = [0u8; 6];

        sensor
            .read_regs(&mut i2c, Register::OutXRegL, &mut data)
            .unwrap();

        assert_eq!(data, [1, 2, 3, 4, 5, 6]);
        i2c.done();
    }

    #[test]
    fn write_regs_sends_register_and_contiguous_data() {
        let sensor = Iis2mdc {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write(
            DEFAULT_I2C_ADDRESS,
            vec![Register::OffsetXRegL.addr(), 0x34, 0x12],
        )]);

        sensor
            .write_regs(&mut i2c, Register::OffsetXRegL, &[0x34, 0x12])
            .unwrap();

        i2c.done();
    }

    #[test]
    fn set_address_updates_subsequent_transport_address() {
        let mut sensor = Iis2mdc {
            address: DEFAULT_I2C_ADDRESS,
        };

        sensor.set_address(0x1f);

        assert_eq!(sensor.address, 0x1f);
    }
}
