use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Interrupt control register.
    ///
    /// Bits 4:3 are reserved and have no setter. Read-modify-write operations
    /// preserve their current values.
    pub struct IntCtrlReg(u8);
    impl Debug;
    /// Enables the interrupt detection for the X-axis
    pub xien, set_xien: 7;
    /// Enables the interrupt detection for the Y-axis
    pub yien, set_yien: 6;
    /// Enables the interrupt detection for the Z-axis
    pub zien, set_zien: 5;
    /// Reserved bits.
    pub reserved, _: 4, 3;
    /// Controls the polarity of the INT bit
    pub iea, set_iea: 2;
    /// Controls whether the INT bit is latched or pulsed
    pub iel, set_iel: 1;
    /// Interrupt enable
    pub ien, set_ien: 0;
}

impl IntCtrlReg {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0xE0)
    }

    /// Create a register from its serialized byte.
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }

    /// Serialize the register as one byte.
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for IntCtrlReg {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for INT_CTRL_REG register.
pub trait IntCtrlRegConfig {
    /// Enables the interrupt detection for the X-axis
    fn set_xien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables the interrupt detection for the Y-axis
    fn set_yien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables the interrupt detection for the Z-axis
    fn set_zien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Controls the polarity of the INT bit
    fn set_iea<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Controls whether the INT bit is latched or pulsed
    fn set_iel<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Interrupt enable
    fn set_ien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl IntCtrlRegConfig for Iis2mdc {
    fn set_xien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_xien(val);
            reg.0
        })
    }
    fn set_yien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_yien(val);
            reg.0
        })
    }
    fn set_zien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_zien(val);
            reg.0
        })
    }
    fn set_iea<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_iea(val);
            reg.0
        })
    }
    fn set_iel<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_iel(val);
            reg.0
        })
    }
    fn set_ien<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntCtrlReg, |b| {
            let mut reg = IntCtrlReg(b);
            reg.set_ien(val);
            reg.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bits() {
        let mut reg = IntCtrlReg::from_bytes([0x18]);
        reg.set_xien(true);
        reg.set_yien(true);
        reg.set_zien(true);
        reg.set_iea(true);
        reg.set_iel(true);
        reg.set_ien(true);

        assert_eq!(reg.into_bytes(), [0xff]);
        assert_eq!(IntCtrlReg::default().into_bytes(), [0xe0]);
    }
}
