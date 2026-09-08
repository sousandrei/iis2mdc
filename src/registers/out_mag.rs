use bitfield::bitfield;

bitfield! {
    /// Magnetometer output register block.
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
    /// Create output registers from six little-endian bytes.
    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        let mut full_bytes = [0u8; 8];
        full_bytes[..6].copy_from_slice(&bytes);
        Self(u64::from_le_bytes(full_bytes))
    }

    /// Return raw X, Y, and Z counts.
    pub fn counts(&self) -> [i16; 3] {
        [self.x() as i16, self.y() as i16, self.z() as i16]
    }
}
