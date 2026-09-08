use embedded_hal::i2c::I2c;

use crate::Iis2mdc;
use crate::registers::{OutMag, Register};

/// High-level magnetometer reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct MagValue {
    count: [i16; 3],
}

impl MagValue {
    /// Create a reading from raw counts.
    pub fn new(count: [i16; 3]) -> Self {
        Self { count }
    }

    /// Create a reading from six little-endian output bytes.
    pub fn from_msr(measurements: &[u8; 6]) -> Self {
        Self::new(OutMag::from_bytes(*measurements).counts())
    }

    /// Return raw signed counts for X, Y, and Z.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return magnetic field in Gauss.
    pub fn as_gauss(&self) -> [f64; 3] {
        self.as_mgauss().map(|value| value / 1000.0)
    }

    /// Return magnetic field in milli-Gauss.
    pub fn as_mgauss(&self) -> [f64; 3] {
        self.count.map(|value| value as f64 * 1.5)
    }

    /// Return magnetic field in micro-Tesla.
    pub fn as_ut(&self) -> [f64; 3] {
        self.as_gauss().map(|value| value * 100.0)
    }
}

/// Magnetometer sensor methods.
pub trait Magnetometer {
    fn get_magnetometer<I2C>(&self, i2c: &mut I2C) -> Result<MagValue, I2C::Error>
    where
        I2C: I2c;
}

impl Magnetometer for Iis2mdc {
    fn get_magnetometer<I2C>(&self, i2c: &mut I2C) -> Result<MagValue, I2C::Error>
    where
        I2C: I2c,
    {
        let mut measurements = [0u8; 6];
        self.read_regs(i2c, Register::OutXRegL, &mut measurements)?;
        Ok(MagValue::from_msr(&measurements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn reads_three_axes_as_signed_counts() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::OutXRegL.addr()],
            vec![0x34, 0x12, 0xff, 0xff, 0x00, 0x80],
        )]);

        let value = sensor.get_magnetometer(&mut i2c).unwrap();

        assert_eq!(value.count(), [0x1234, -1, -32768]);
        i2c.done();
    }
}
