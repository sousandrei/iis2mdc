use bitfield::bitfield;

bitfield! {
    /// Interrupt control register.
    ///
    /// Bits 4:3 are reserved and have no setter. Read-modify-write operations
    /// preserve their current values.
    pub struct IntCtrlReg(u8);
    impl Debug;
    /// Enables interrupt detection for the X axis.
    pub xien, set_xien: 7;
    /// Enables interrupt detection for the Y axis.
    pub yien, set_yien: 6;
    /// Enables interrupt detection for the Z axis.
    pub zien, set_zien: 5;
    /// Reserved bits.
    pub reserved, _: 4, 3;
    /// Controls interrupt polarity.
    pub iea, set_iea: 2;
    /// Selects latched or pulsed interrupts.
    pub iel, set_iel: 1;
    /// Enables interrupt generation.
    pub ien, set_ien: 0;
}

impl IntCtrlReg {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0xE0)
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

impl Default for IntCtrlReg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bits() {
        let mut register = IntCtrlReg::from_bytes([0x18]);
        register.set_xien(true);
        register.set_yien(true);
        register.set_zien(true);
        register.set_iea(true);
        register.set_iel(true);
        register.set_ien(true);

        assert_eq!(register.into_bytes(), [0xff]);
        assert_eq!(IntCtrlReg::default().into_bytes(), [0xe0]);
    }
}
