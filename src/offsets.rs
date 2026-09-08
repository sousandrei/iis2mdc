use crate::registers::Register;
use crate::{Iis2mdc, RegisterBus as I2c};

/// Hard-iron offset access for all magnetometer axes.
pub trait HardIronOffsets {
    fn get_offset_x<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    fn set_offset_x<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn get_offset_y<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    fn set_offset_y<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn get_offset_z<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
    where
        I2C: I2c;
    fn set_offset_z<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

macro_rules! offset_accessors {
    ($get:ident, $set:ident, $register:ident) => {
        fn $get<I2C>(&self, i2c: &mut I2C) -> Result<i16, I2C::Error>
        where
            I2C: I2c,
        {
            let mut bytes = [0u8; 2];
            self.read_regs(i2c, Register::$register, &mut bytes)?;
            Ok(i16::from_le_bytes(bytes))
        }

        fn $set<I2C>(&self, i2c: &mut I2C, offset: i16) -> Result<(), I2C::Error>
        where
            I2C: I2c,
        {
            self.write_regs(i2c, Register::$register, &offset.to_le_bytes())
        }
    };
}

impl HardIronOffsets for Iis2mdc {
    offset_accessors!(get_offset_x, set_offset_x, OffsetXRegL);
    offset_accessors!(get_offset_y, set_offset_y, OffsetYRegL);
    offset_accessors!(get_offset_z, set_offset_z, OffsetZRegL);
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn writes_signed_offset_as_little_endian_block() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::OffsetYRegL.addr(), 0xa6, 0xff],
        )]);

        sensor.set_offset_y(&mut i2c, -90).unwrap();
        i2c.done();
    }
}
