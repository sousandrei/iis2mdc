use crate::registers::Register;
use crate::{Iis2mdc, RegisterBus as I2c};

/// High-level temperature reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct TempValue {
    count: i16,
}

impl TempValue {
    /// Create a reading from raw counts.
    pub fn new(count: i16) -> Self {
        Self { count }
    }

    /// Create a reading from two little-endian output bytes.
    pub fn from_msr(measurements: &[u8; 2]) -> Self {
        Self::new(i16::from_le_bytes(*measurements))
    }

    /// Return the raw signed count.
    pub fn count(&self) -> i16 {
        self.count
    }

    /// Return temperature in degrees Celsius.
    pub fn as_celsius(&self) -> f32 {
        self.count as f32 / 256.0 + 25.0
    }
}

/// Temperature sensor methods.
pub trait Temperature {
    fn get_temperature<I2C>(&self, i2c: &mut I2C) -> Result<TempValue, I2C::Error>
    where
        I2C: I2c;
}

impl Temperature for Iis2mdc {
    fn get_temperature<I2C>(&self, i2c: &mut I2C) -> Result<TempValue, I2C::Error>
    where
        I2C: I2c,
    {
        let mut measurements = [0u8; 2];
        self.read_regs(i2c, Register::TempOutLReg, &mut measurements)?;
        Ok(TempValue::from_msr(&measurements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn reads_signed_temperature_counts() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::TempOutLReg.addr() | 0x80],
            vec![0x00, 0x01],
        )]);

        let value = sensor.get_temperature(&mut i2c).unwrap();

        assert_eq!(value.count(), 256);
        assert_eq!(value.as_celsius(), 26.0);
        i2c.done();
    }
}
