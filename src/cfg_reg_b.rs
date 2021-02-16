use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The CFG_REG_B register
#[derive(Debug)]
pub struct CfgRegB(u8);

impl fmt::Display for CfgRegB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for CfgRegB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for CfgRegB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x61u8;

/// Enables offset cancellation in single measurement mode. The OFF_CANC bit
/// must be set to 1 when enabling offset cancellation in single measurement mode.
///
/// Default value: 0
///
/// (0: offset cancellation in single measurement mode disabled;
/// 1: offset cancellation in single measurement mode enabled)
const OFF_CANC_ONE_SHOT: u8 = 4;

/// If ‘1’, the interrupt block recognition checks data after the hard-iron correction to
/// discover the interrupt.
///
/// Default value: 0
const INT_ON_DATAOFF: u8 = 3;

/// Selects the frequency of the set pulse.
///
/// Default value: 0
///
/// (0: set pulse is released every 63 ODR;
/// 1: set pulse is released only at power-on after PD condition)
const SET_FREQ: u8 = 2;

/// Enables offset cancellation.
///
/// Default value: 0
///
/// (0: offset cancellation disabled; 1: offset cancellation enabled)
const OFF_CANC: u8 = 1;

///Enables low-pass filter.
///
/// Default value: 0
///
///(0: digital filter disabled; 1: digital filter enabled)
const LPF: u8 = 0;

impl Register for CfgRegB {}

impl CfgRegB {
    pub fn new(bits: u8) -> Self {
        CfgRegB(bits)
    }

    pub fn off_canc_one_shot<I2C>(&mut self) -> bool {
        self.0 & (1 << OFF_CANC_ONE_SHOT) != 0
    }

    pub fn set_off_canc_one_shot<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << OFF_CANC_ONE_SHOT);
        self.0 |= (value as u8) << OFF_CANC_ONE_SHOT;
        self.write(i2c, ADDR, self.0)
    }

    pub fn int_on_dataoff<I2C>(&mut self) -> bool {
        self.0 & (1 << INT_ON_DATAOFF) != 0
    }

    pub fn set_int_on_dataoff<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << INT_ON_DATAOFF);
        self.0 |= (value as u8) << INT_ON_DATAOFF;
        self.write(i2c, ADDR, self.0)
    }

    pub fn set_freq<I2C>(&mut self) -> bool {
        self.0 & (1 << SET_FREQ) != 0
    }

    pub fn set_set_freq<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << SET_FREQ);
        self.0 |= (value as u8) << SET_FREQ;
        self.write(i2c, ADDR, self.0)
    }

    pub fn off_canc<I2C>(&mut self) -> bool {
        self.0 & (1 << OFF_CANC) != 0
    }

    pub fn set_off_canc<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << OFF_CANC);
        self.0 |= (value as u8) << OFF_CANC;
        self.write(i2c, ADDR, self.0)
    }

    pub fn lpf<I2C>(&mut self) -> bool {
        self.0 & (1 << LPF) != 0
    }

    pub fn set_lpf<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << LPF);
        self.0 |= (value as u8) << LPF;
        self.write(i2c, ADDR, self.0)
    }
}
