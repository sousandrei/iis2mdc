use bitfield::bitfield;

bitfield! {
    /// Configuration register C.
    ///
    /// Bit 2 is reserved and must remain zero for correct operation. It has no
    /// setter, and read-modify-write operations preserve its current value.
    pub struct CfgRegC(u8);
    impl Debug;
    /// Routes the interrupt signal to the INT/DRDY pin.
    pub int_on_pin, set_int_on_pin: 6;
    /// Inhibits the I2C interface.
    pub i2c_dis, set_i2c_dis: 5;
    /// Enables block data update.
    pub bdu, set_bdu: 4;
    /// Inverts the low and high parts of the data.
    pub ble, set_ble: 3;
    /// Reserved bit.
    pub reserved, _: 2;
    /// Enables self-test.
    pub self_test, set_self_test: 1;
    /// Routes data-ready to the INT/DRDY pin.
    pub drdy_on_pin, set_drdy_on_pin: 0;
}

impl CfgRegC {
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

impl Default for CfgRegC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits_and_preserve_reserved_bit() {
        let mut register = CfgRegC::from_bytes([0x04]);
        register.set_int_on_pin(true);
        register.set_i2c_dis(true);
        register.set_bdu(true);
        register.set_ble(true);
        register.set_self_test(true);
        register.set_drdy_on_pin(true);

        assert_eq!(register.into_bytes(), [0x7f]);
        assert_eq!(CfgRegC::default().into_bytes(), [0x00]);
    }
}
