use bitfield::bitfield;

bitfield! {
    /// Status register.
    pub struct StatusReg(u8);
    impl Debug;
    /// X, Y, and Z-axis data overrun.
    pub zyxor, _: 7;
    /// Z-axis data overrun.
    pub zor, _: 6;
    /// Y-axis data overrun.
    pub yor, _: 5;
    /// X-axis data overrun.
    pub xor, _: 4;
    /// X, Y, and Z-axis new data available.
    pub zyxda, _: 3;
    /// Z-axis new data available.
    pub zda, _: 2;
    /// Y-axis new data available.
    pub yda, _: 1;
    /// X-axis new data available.
    pub xda, _: 0;
}

impl StatusReg {
    /// Create a register from its serialized byte.
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }

    /// Serialize the register as one byte.
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}
