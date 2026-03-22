use crate::Iis2mdc;
use crate::registers::Register;
use embedded_hal::i2c::I2c;

/// High-level temperature reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct TempValue {
    count: i16,
}

impl TempValue {
    /// Create a new `TempValue` from raw counts.
    pub fn new(count: i16) -> TempValue {
        TempValue { count }
    }

    /// Create a new `TempValue` from raw byte measurements (little-endian).
    pub fn from_msr(measurements: &[u8; 2]) -> TempValue {
        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        TempValue { count: raw_temp }
    }

    /// Return the raw signed integer counts.
    pub fn count(&self) -> i16 {
        self.count
    }

    /// Return temperature in degrees Celsius [°C].
    pub fn as_celsius(&self) -> f32 {
        (self.count as f32 / 256.0) + 25.0
    }
}

/// Temperature sensor methods.
pub trait Temperature {
    /// Get current temperature reading.
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
        i2c.write_read(self.address, &[Register::TempOutLReg.addr()], &mut measurements)?;

        Ok(TempValue::from_msr(&measurements))
    }
}
