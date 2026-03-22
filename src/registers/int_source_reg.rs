use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Interrupt source register
    pub struct IntSourceReg(u8);
    impl Debug;
    /// X-axis value exceeds the threshold positive side
    pub p_th_s_x, _: 7;
    /// Y-axis value exceeds the threshold positive side
    pub p_th_s_y, _: 6;
    /// Z-axis value exceeds the threshold positive side
    pub p_th_s_z, _: 5;
    /// X-axis value exceeds the threshold negative side
    pub n_th_s_x, _: 4;
    /// Y-axis value exceeds the threshold negative side
    pub n_th_s_y, _: 3;
    /// Z-axis value exceeds the threshold negative side
    pub n_th_s_z, _: 2;
    /// MROI flag generation is always enabled
    pub mroi, _: 1;
    /// This bit signals when the interrupt event occurs
    pub int_active, _: 0;
}

impl IntSourceReg {
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
}

/// Interrupt source methods.
pub trait IntSource {
    /// Get the current interrupt source status.
    fn get_int_source<I2C>(&self, i2c: &mut I2C) -> Result<IntSourceReg, I2C::Error>
    where
        I2C: I2c;
}

impl IntSource for Iis2mdc {
    fn get_int_source<I2C>(&self, i2c: &mut I2C) -> Result<IntSourceReg, I2C::Error>
    where
        I2C: I2c,
    {
        let val = self.read_reg(i2c, Register::IntSourceReg)?;
        Ok(IntSourceReg(val))
    }
}
