use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Configuration register B
    pub struct CfgRegB(u8);
    impl Debug;
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
    pub fn new() -> Self {
        Self(0x00)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
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
