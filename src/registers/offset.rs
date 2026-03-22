use crate::Iis2mdc;
use crate::registers::Register;
use embedded_hal::i2c::I2c;

/// Hard-iron offset configuration.
pub trait HardIronOffsetConfig {
    /// Get hard-iron offset for X axis.
    fn get_offset_x<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    /// Set hard-iron offset for X axis.
    fn set_offset_x<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Get hard-iron offset for Y axis.
    fn get_offset_y<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    /// Set hard-iron offset for Y axis.
    fn set_offset_y<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Get hard-iron offset for Z axis.
    fn get_offset_z<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    /// Set hard-iron offset for Z axis.
    fn set_offset_z<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl HardIronOffsetConfig for Iis2mdc {
    fn get_offset_x<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 2];
        i2c.write_read(self.address, &[Register::OffsetXRegL.addr()], &mut bytes)?;
        Ok(i16::from_le_bytes(bytes))
    }
    fn set_offset_x<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        let bytes = offset.to_le_bytes();
        self.write_reg(i2c, Register::OffsetXRegL, bytes[0])?;
        self.write_reg(i2c, Register::OffsetXRegH, bytes[1])
    }

    fn get_offset_y<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 2];
        i2c.write_read(self.address, &[Register::OffsetYRegL.addr()], &mut bytes)?;
        Ok(i16::from_le_bytes(bytes))
    }
    fn set_offset_y<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        let bytes = offset.to_le_bytes();
        self.write_reg(i2c, Register::OffsetYRegL, bytes[0])?;
        self.write_reg(i2c, Register::OffsetYRegH, bytes[1])
    }

    fn get_offset_z<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 2];
        i2c.write_read(self.address, &[Register::OffsetZRegL.addr()], &mut bytes)?;
        Ok(i16::from_le_bytes(bytes))
    }
    fn set_offset_z<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        let bytes = offset.to_le_bytes();
        self.write_reg(i2c, Register::OffsetZRegL, bytes[0])?;
        self.write_reg(i2c, Register::OffsetZRegH, bytes[1])
    }
}
