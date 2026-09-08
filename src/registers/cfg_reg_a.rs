use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Output data rate configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Odr {
    /// 10 Hz
    Hz10 = 0b00,
    /// 20 Hz
    Hz20 = 0b01,
    /// 50 Hz
    Hz50 = 0b10,
    /// 100 Hz
    Hz100 = 0b11,
}
impl From<u8> for Odr {
    fn from(val: u8) -> Self {
        match val {
            0b00 => Odr::Hz10,
            0b01 => Odr::Hz20,
            0b10 => Odr::Hz50,
            0b11 => Odr::Hz100,
            _ => Odr::Hz10,
        }
    }
}
impl From<Odr> for u8 {
    fn from(val: Odr) -> u8 {
        val as u8
    }
}

/// Mode of operation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Mode {
    /// Continuous mode
    Continuous = 0b00,
    /// Single mode
    Single = 0b01,
    /// Idle mode
    Idle = 0b10,
}
impl From<u8> for Mode {
    fn from(val: u8) -> Self {
        match val {
            0b00 => Mode::Continuous,
            0b01 => Mode::Single,
            0b10 => Mode::Idle,
            0b11 => Mode::Idle,
            _ => Mode::Idle,
        }
    }
}
impl From<Mode> for u8 {
    fn from(val: Mode) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Configuration register A.
    ///
    /// The reset value is `0x03`, which selects idle mode.
    pub struct CfgRegA(u8);
    impl Debug;
    /// Enables the magnetometer temperature compensation
    pub comp_temp_en, set_comp_temp_en: 7;
    /// Reboot magnetometer memory content
    pub reboot, set_reboot: 6;
    /// Configuration registers and user registers are reset
    pub soft_rst, set_soft_rst: 5;
    /// Enables low-power mode
    pub lp, set_lp: 4;
    /// Output data rate configuration
    pub from into Odr, odr, set_odr: 3, 2;
    /// Mode of operation
    pub from into Mode, md, set_md: 1, 0;
}

impl CfgRegA {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0x03)
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

impl Default for CfgRegA {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CFG_REG_A register.
pub trait CfgRegAConfig {
    /// Enables the magnetometer temperature compensation
    fn set_comp_temp_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Reboot magnetometer memory content
    fn set_reboot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Configuration registers and user registers are reset
    fn set_soft_rst<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables low-power mode
    fn set_lp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Output data rate configuration
    fn set_odr<I2C>(&self, i2c: &mut I2C, val: Odr) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Mode of operation
    fn set_md<I2C>(&self, i2c: &mut I2C, val: Mode) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl CfgRegAConfig for Iis2mdc {
    fn set_comp_temp_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_comp_temp_en(val);
            reg.0
        })
    }
    fn set_reboot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_reboot(val);
            reg.0
        })
    }
    fn set_soft_rst<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_soft_rst(val);
            reg.0
        })
    }
    fn set_lp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_lp(val);
            reg.0
        })
    }
    fn set_odr<I2C>(&self, i2c: &mut I2C, val: Odr) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_odr(val);
            reg.0
        })
    }
    fn set_md<I2C>(&self, i2c: &mut I2C, val: Mode) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |b| {
            let mut reg = CfgRegA(b);
            reg.set_md(val);
            reg.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits() {
        let mut reg = CfgRegA::default();
        reg.set_comp_temp_en(true);
        reg.set_reboot(true);
        reg.set_soft_rst(true);
        reg.set_lp(true);
        reg.set_odr(Odr::Hz50);
        reg.set_md(Mode::Continuous);

        assert_eq!(reg.into_bytes(), [0xf8]);
        assert_eq!(CfgRegA::default().into_bytes(), [0x03]);
    }
}
