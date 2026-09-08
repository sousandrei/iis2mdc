use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Configuration register C.
    ///
    /// Bit 2 is reserved and must remain zero for correct operation. It has no
    /// setter, and read-modify-write operations preserve its current value.
    pub struct CfgRegC(u8);
    impl Debug;
    /// INTERRUPT signal is driven on the INT/DRDY pin
    pub int_on_pin, set_int_on_pin: 6;
    /// I2C interface is inhibited
    pub i2c_dis, set_i2c_dis: 5;
    /// Block data update
    pub bdu, set_bdu: 4;
    /// Inversion of the low and high parts of the data
    pub ble, set_ble: 3;
    /// Reserved bit.
    pub reserved, _: 2;
    /// Self-test enable
    pub self_test, set_self_test: 1;
    /// Data-ready signal is driven on the INT/DRDY pin
    pub drdy_on_pin, set_drdy_on_pin: 0;
}

impl CfgRegC {
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

impl Default for CfgRegC {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CFG_REG_C register.
pub trait CfgRegCConfig {
    /// INTERRUPT signal is driven on the INT/DRDY pin
    fn set_int_on_pin<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// I2C interface is inhibited
    fn set_i2c_dis<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Block data update
    fn set_bdu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Inversion of the low and high parts of the data
    fn set_ble<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Self-test enable
    fn set_self_test<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Data-ready signal is driven on the INT/DRDY pin
    fn set_drdy_on_pin<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl CfgRegCConfig for Iis2mdc {
    fn set_int_on_pin<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_int_on_pin(val);
            reg.0
        })
    }
    fn set_i2c_dis<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_i2c_dis(val);
            reg.0
        })
    }
    fn set_bdu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_bdu(val);
            reg.0
        })
    }
    fn set_ble<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_ble(val);
            reg.0
        })
    }
    fn set_self_test<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_self_test(val);
            reg.0
        })
    }
    fn set_drdy_on_pin<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegC, |b| {
            let mut reg = CfgRegC(b);
            reg.set_drdy_on_pin(val);
            reg.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bit() {
        let mut reg = CfgRegC::from_bytes([0x04]);
        reg.set_int_on_pin(true);
        reg.set_i2c_dis(true);
        reg.set_bdu(true);
        reg.set_ble(true);
        reg.set_self_test(true);
        reg.set_drdy_on_pin(true);

        assert_eq!(reg.into_bytes(), [0x7f]);
        assert_eq!(CfgRegC::default().into_bytes(), [0x00]);
    }
}
