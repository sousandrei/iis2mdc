use crate::Iis2mdc;
use crate::registers::Register;
use embedded_hal::i2c::I2c;

/// Interrupt threshold configuration.
pub trait IntThsConfig {
    /// Get interrupt threshold.
    fn get_int_threshold<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c;
    /// Set interrupt threshold.
    fn set_int_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl IntThsConfig for Iis2mdc {
    fn get_int_threshold<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 2];
        i2c.write_read(self.address, &[Register::IntThsLReg.addr()], &mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }
    fn set_int_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        let bytes = threshold.to_le_bytes();
        self.write_reg(i2c, Register::IntThsLReg, bytes[0])?;
        self.write_reg(i2c, Register::IntThsHReg, bytes[1])
    }
}
