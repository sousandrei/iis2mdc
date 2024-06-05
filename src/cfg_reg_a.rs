use core::fmt;

use crate::Register;

/// The CFG_REG_A register
///
/// The configuration register is used to configure the output data rate and the measurement configuration.
#[derive(Debug)]
pub struct CfgRegA(u8);

impl fmt::Display for CfgRegA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for CfgRegA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for CfgRegA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x60u8;

///Enables the magnetometer temperature compensation
///
/// Default value: 0
///
///(0: temperature compensation disabled; 1: temperature compensation enabled)
const COMP_TEMP_EN: u8 = 7;

/// Reboot magnetometer memory content
///
/// Default value: 0
///
/// (0: normal mode; 1: reboot memory content)
const REBOOT: u8 = 6;

/// When this bit is set, the configuration registers and user registers are reset.
///
/// Flash registers keep their values
///
/// Default value: 0
const SOFT_RST: u8 = 5;

/// Enables low-power mode
///
/// Default value: 0
///
/// 0: high-resolution mode 1: low-power mode enabled
const LP: u8 = 4;

const ODR_MASK: u8 = 0b11;
const ODR_OFFSET: u8 = 2;

/// Output data rate configuration
///
/// Default value: 00
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Odr {
    Hz10,  // 10  Hz
    Hz20,  // 20  Hz
    Hz50,  // 50  Hz
    Hz100, // 100 Hz
}

const MODE_MASK: u8 = 0b11;
const MODE_OFFSET: u8 = 0;

/// These bits select the mode of operation of the device
///
/// Default value: 11
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Mode {
    Continuous,
    Single,
    Idle,
}

impl Register for CfgRegA {}

impl CfgRegA {
    pub fn new(bits: u8) -> Self {
        CfgRegA(bits)
    }

    pub fn value<I2C>(&mut self, i2c: &mut I2C) -> Result<u8, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.read(i2c, ADDR)
    }

    pub fn data_rate(&self) -> f32 {
        match (self.0 >> ODR_OFFSET) & ODR_MASK {
            0 => 10.0,
            1 => 20.0,
            2 => 50.0,
            3 => 100.0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_data_rate<I2C>(&mut self, i2c: &mut I2C, value: Odr) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(ODR_MASK << ODR_OFFSET);
        self.0 |= (value as u8) << ODR_OFFSET;
        self.write(i2c, ADDR, self.0)
    }

    pub fn mode(&self) -> Mode {
        match (self.0 >> MODE_OFFSET) & MODE_MASK {
            0 => Mode::Continuous,
            1 => Mode::Single,
            2 => Mode::Idle,
            3 => Mode::Idle,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_mode<I2C>(&mut self, i2c: &mut I2C, value: Mode) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(MODE_MASK << MODE_OFFSET);
        self.0 |= (value as u8) << MODE_OFFSET;
        self.write(i2c, ADDR, self.0)
    }

    pub fn comp_temp_en(&mut self) -> bool {
        self.0 & (1 << COMP_TEMP_EN) != 0
    }

    pub fn set_comp_temp_en<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(1 << COMP_TEMP_EN);
        self.0 |= (value as u8) << COMP_TEMP_EN;
        self.write(i2c, ADDR, self.0)
    }

    pub fn reboot(&mut self) -> bool {
        self.0 & (1 << REBOOT) != 0
    }

    pub fn set_reboot<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(1 << REBOOT);
        self.0 |= (value as u8) << REBOOT;
        self.write(i2c, ADDR, self.0)
    }

    pub fn soft_rst(&mut self) -> bool {
        self.0 & (1 << SOFT_RST) != 0
    }

    pub fn set_soft_rst<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(1 << SOFT_RST);
        self.0 |= (value as u8) << SOFT_RST;
        self.write(i2c, ADDR, self.0)
    }

    pub fn lp(&mut self) -> bool {
        self.0 & (1 << LP) != 0
    }

    pub fn set_lp<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.0 &= !(1 << LP);
        self.0 |= (value as u8) << LP;
        self.write(i2c, ADDR, self.0)
    }
}
