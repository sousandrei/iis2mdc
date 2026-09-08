use bitfield::bitfield;

bitfield! {
    /// Configuration register B.
    ///
    /// Bits 7:5 are reserved by the device and have no setter. Read-modify-
    /// write operations preserve their current values.
    pub struct CfgRegB(u8);
    impl Debug;
    /// Reserved bits.
    pub reserved, _: 7, 5;
    /// Enables offset cancellation in single measurement mode.
    pub off_canc_one_shot, set_off_canc_one_shot: 4;
    /// Checks interrupt data after hard-iron correction.
    pub int_on_dataoff, set_int_on_dataoff: 3;
    /// Selects the frequency of the set pulse.
    pub set_freq, set_set_freq: 2;
    /// Enables offset cancellation.
    pub off_canc, set_off_canc: 1;
    /// Enables the low-pass filter.
    pub lpf, set_lpf: 0;
}

impl CfgRegB {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0x00)
    }

    /// Create a register from its serialized byte.
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }

    /// Serialize the register as one byte.
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for CfgRegB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bits() {
        let mut register = CfgRegB::from_bytes([0xe0]);
        register.set_off_canc_one_shot(true);
        register.set_int_on_dataoff(true);
        register.set_set_freq(true);
        register.set_off_canc(true);
        register.set_lpf(true);

        assert_eq!(register.into_bytes(), [0xff]);
        assert_eq!(CfgRegB::default().into_bytes(), [0x00]);
    }
}
