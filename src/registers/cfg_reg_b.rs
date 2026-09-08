use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Configuration register B.
    ///
    /// Bits 7:5 are reserved by the device and have no setter. Read-modify-
    /// write operations preserve their current values.
    pub struct CfgRegB(u8);
    impl Debug;
    /// Reserved bits.
    pub reserved, _: 7, 5;
    /// Enables offset cancellation in single measurement mode
    pub off_canc_one_shot, set_off_canc_one_shot: 4;
    /// Interrupt block recognition checks data after hard-iron correction
    pub int_on_dataoff, set_int_on_dataoff: 3;
    /// Selects the frequency of the set pulse
    pub set_freq, set_set_freq: 2;
    /// Enables offset cancellation
    pub off_canc, set_off_canc: 1;
    /// Enables low-pass filter
    pub lpf, set_lpf: 0;
}

impl CfgRegB {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0x00)
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

impl Default for CfgRegB {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CFG_REG_B register.
pub trait CfgRegBConfig {
    /// Enables offset cancellation in single measurement mode
    fn set_off_canc_one_shot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Interrupt block recognition checks data after hard-iron correction
    fn set_int_on_dataoff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Selects the frequency of the set pulse
    fn set_set_freq<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables offset cancellation
    fn set_off_canc<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables low-pass filter
    fn set_lpf<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl CfgRegBConfig for Iis2mdc {
    fn set_off_canc_one_shot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegB, |b| {
            let mut reg = CfgRegB(b);
            reg.set_off_canc_one_shot(val);
            reg.0
        })
    }
    fn set_int_on_dataoff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegB, |b| {
            let mut reg = CfgRegB(b);
            reg.set_int_on_dataoff(val);
            reg.0
        })
    }
    fn set_set_freq<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegB, |b| {
            let mut reg = CfgRegB(b);
            reg.set_set_freq(val);
            reg.0
        })
    }
    fn set_off_canc<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegB, |b| {
            let mut reg = CfgRegB(b);
            reg.set_off_canc(val);
            reg.0
        })
    }
    fn set_lpf<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegB, |b| {
            let mut reg = CfgRegB(b);
            reg.set_lpf(val);
            reg.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bits() {
        let mut reg = CfgRegB::from_bytes([0xe0]);
        reg.set_off_canc_one_shot(true);
        reg.set_int_on_dataoff(true);
        reg.set_set_freq(true);
        reg.set_off_canc(true);
        reg.set_lpf(true);

        assert_eq!(reg.into_bytes(), [0xff]);
        assert_eq!(CfgRegB::default().into_bytes(), [0x00]);
    }
}
