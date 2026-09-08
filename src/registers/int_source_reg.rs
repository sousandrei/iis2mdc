use bitfield::bitfield;

bitfield! {
    /// Interrupt source register.
    pub struct IntSourceReg(u8);
    impl Debug;
    /// X-axis value exceeds the positive threshold.
    pub p_th_s_x, _: 7;
    /// Y-axis value exceeds the positive threshold.
    pub p_th_s_y, _: 6;
    /// Z-axis value exceeds the positive threshold.
    pub p_th_s_z, _: 5;
    /// X-axis value exceeds the negative threshold.
    pub n_th_s_x, _: 4;
    /// Y-axis value exceeds the negative threshold.
    pub n_th_s_y, _: 3;
    /// Z-axis value exceeds the negative threshold.
    pub n_th_s_z, _: 2;
    /// MROI flag.
    pub mroi, _: 1;
    /// Interrupt event is active.
    pub int_active, _: 0;
}

impl IntSourceReg {
    /// Create a register from its serialized byte.
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }

    /// Serialize the register as one byte.
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}
