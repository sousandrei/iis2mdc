use embedded_hal::i2c::I2c;

use crate::Iis2mdc;
use crate::registers::{IntCtrlReg, IntSourceReg, Register};

/// Interrupt control configuration methods.
pub trait InterruptControl {
    fn set_xien<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_yien<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_zien<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_iea<I2C>(&self, i2c: &mut I2C, active_high: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_iel<I2C>(&self, i2c: &mut I2C, latched: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ien<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

/// Interrupt source access methods.
pub trait InterruptSource {
    fn get_int_source<I2C>(&self, i2c: &mut I2C) -> Result<IntSourceReg, I2C::Error>
    where
        I2C: I2c;
}

/// Interrupt threshold access methods.
pub trait InterruptThreshold {
    fn get_int_threshold<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c;
    fn set_int_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

macro_rules! set_control {
    ($method:ident, $setter:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
        where
            I2C: I2c,
        {
            self.modify_reg(i2c, Register::IntCtrlReg, |byte| {
                let mut register = IntCtrlReg::from_bytes([byte]);
                register.$setter(value);
                register.into_bytes()[0]
            })
        }
    };
}

impl InterruptControl for Iis2mdc {
    set_control!(set_xien, set_xien);
    set_control!(set_yien, set_yien);
    set_control!(set_zien, set_zien);
    set_control!(set_iea, set_iea);
    set_control!(set_iel, set_iel);
    set_control!(set_ien, set_ien);
}

impl InterruptSource for Iis2mdc {
    fn get_int_source<I2C>(&self, i2c: &mut I2C) -> Result<IntSourceReg, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(IntSourceReg::from_bytes([
            self.read_reg(i2c, Register::IntSourceReg)?
        ]))
    }
}

impl InterruptThreshold for Iis2mdc {
    fn get_int_threshold<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 2];
        self.read_regs(i2c, Register::IntThsLReg, &mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn set_int_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.write_regs(i2c, Register::IntThsLReg, &threshold.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn reads_interrupt_source() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::IntSourceReg.addr()],
            vec![0x81],
        )]);

        let source = sensor.get_int_source(&mut i2c).unwrap();

        assert!(source.p_th_s_x());
        assert!(source.int_active());
        i2c.done();
    }

    #[test]
    fn writes_interrupt_threshold_as_little_endian_block() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::IntThsLReg.addr(), 0x34, 0x12],
        )]);

        sensor.set_int_threshold(&mut i2c, 0x1234).unwrap();
        i2c.done();
    }
}
