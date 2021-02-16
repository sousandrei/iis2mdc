use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The CFG_REG_B register
#[derive(Debug)]
pub struct CfgRegC(u8);

impl fmt::Display for CfgRegC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for CfgRegC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for CfgRegC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x62u8;

/// If '1', the INTERRUPT signal (INT bit in INT_SOURCE_REG (64h)) is driven
/// on the INT/DRDY pin. The INT/DRDY pin is configured in push-pull output mode.
///
/// Default value: 0
const INT_ON_PIN: u8 = 6;

/// I2C_DIS If ‘1’, the I2C interface is inhibited. Only the SPI interface can be used.
const I2C_DIS: u8 = 5;

/// If enabled, reading of incorrect data is avoided when the user reads asynchronously. In fact if the read request arrives during an update of the output data, a
/// latch is possible, reading incoherent high and low parts of the same register. Only
/// one part is updated and the other one remains old.
const BDU: u8 = 4;

/// If ‘1’, an inversion of the low and high parts of the data occurs.
const BLE: u8 = 3;

/// If ‘1’, the self-test is enabled.
const SELF_TEST: u8 = 1;

/// If '1', the data-ready signal (Zyxda bit in STATUS_REG (67h)) is driven on the
/// INT/DRDY pin. The INT/DRDY pin is configured in push-pull output mode.
///
/// Default value: 0
const DRDY_ON_PIN: u8 = 0;

impl Register for CfgRegC {}

impl CfgRegC {
    pub fn new(bits: u8) -> Self {
        CfgRegC(bits)
    }

    pub fn int_on_pin(&mut self) -> bool {
        self.0 & (1 << INT_ON_PIN) != 0
    }

    pub fn set_int_on_pin<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << INT_ON_PIN);
        self.0 |= (value as u8) << INT_ON_PIN;
        self.write(i2c, ADDR, self.0)
    }

    pub fn i2c_dis(&mut self) -> bool {
        self.0 & (1 << I2C_DIS) != 0
    }

    pub fn set_i2c_dis<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << I2C_DIS);
        self.0 |= (value as u8) << I2C_DIS;
        self.write(i2c, ADDR, self.0)
    }

    pub fn bdu(&mut self) -> bool {
        self.0 & (1 << BDU) != 0
    }

    pub fn set_bdu<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << BDU);
        self.0 |= (value as u8) << BDU;
        self.write(i2c, ADDR, self.0)
    }

    pub fn ble(&mut self) -> bool {
        self.0 & (1 << BLE) != 0
    }

    pub fn set_ble<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << BLE);
        self.0 |= (value as u8) << BLE;
        self.write(i2c, ADDR, self.0)
    }

    pub fn self_test(&mut self) -> bool {
        self.0 & (1 << SELF_TEST) != 0
    }

    pub fn set_self_test<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << SELF_TEST);
        self.0 |= (value as u8) << SELF_TEST;
        self.write(i2c, ADDR, self.0)
    }

    pub fn drdy_on_pin(&mut self) -> bool {
        self.0 & (1 << DRDY_ON_PIN) != 0
    }

    pub fn set_drdy_on_pin<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << DRDY_ON_PIN);
        self.0 |= (value as u8) << DRDY_ON_PIN;
        self.write(i2c, ADDR, self.0)
    }
}
