use embedded_hal::i2c::I2c;

use crate::Iis2mdc;
use crate::registers::{Register, StatusReg};

/// Status register access methods.
pub trait Status {
    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: I2c;
}

impl Status for Iis2mdc {
    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(StatusReg::from_bytes([
            self.read_reg(i2c, Register::StatusReg)?
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn reads_status_register() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::StatusReg.addr()],
            vec![0x89],
        )]);

        let status = sensor.get_status(&mut i2c).unwrap();

        assert!(status.zyxor());
        assert!(status.zyxda());
        assert!(status.xda());
        i2c.done();
    }
}
