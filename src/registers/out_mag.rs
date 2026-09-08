use crate::Iis2mdc;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Magnetometer output register.
    pub struct OutMag(u64);
    impl Debug;
    /// X-axis output.
    pub x, _: 15, 0;
    /// Y-axis output.
    pub y, _: 31, 16;
    /// Z-axis output.
    pub z, _: 47, 32;
}

impl OutMag {
    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        let mut full_bytes = [0u8; 8];
        full_bytes[..6].copy_from_slice(&bytes);
        Self(u64::from_le_bytes(full_bytes))
    }

    /// Returns raw counts as [x, y, z]
    pub fn counts(&self) -> [i16; 3] {
        [self.x() as i16, self.y() as i16, self.z() as i16]
    }
}

/// High-level magnetometer reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct MagValue {
    count: [i16; 3],
}

impl MagValue {
    /// Create a new `MagValue` from raw counts.
    pub fn new(count: [i16; 3]) -> MagValue {
        MagValue { count }
    }

    /// Create a new `MagValue` from raw byte measurements (little-endian).
    pub fn from_msr(measurements: &[u8; 6]) -> MagValue {
        let out = OutMag::from_bytes(*measurements);
        MagValue {
            count: out.counts(),
        }
    }

    /// Return the raw signed integer counts for X, Y, Z axes.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return magnetic field in Gauss [G].
    pub fn as_gauss(&self) -> [f64; 3] {
        self.as_mgauss().map(|v| v / 1000.0)
    }

    /// Return magnetic field in milli-Gauss [mG].
    /// Sensitivity is fixed at 1.5 mG/LSB.
    pub fn as_mgauss(&self) -> [f64; 3] {
        self.count.map(|r| r as f64 * 1.5)
    }

    /// Return magnetic field in micro-Tesla [µT].
    /// 1 Gauss = 100 micro-Tesla.
    pub fn as_ut(&self) -> [f64; 3] {
        self.as_gauss().map(|v| v * 100.0)
    }
}

/// Magnetometer sensor methods.
pub trait Magnetometer {
    /// Get current magnetometer reading.
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
        i2c.write_read(
            self.address,
            &[Register::OutXRegL.addr()],
            &mut measurements,
        )?;

        Ok(MagValue::from_msr(&measurements))
    }
}
