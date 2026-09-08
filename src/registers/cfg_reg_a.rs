use bitfield::bitfield;

/// Output data rate configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Odr {
    /// 10 Hz.
    Hz10 = 0b00,
    /// 20 Hz.
    Hz20 = 0b01,
    /// 50 Hz.
    Hz50 = 0b10,
    /// 100 Hz.
    Hz100 = 0b11,
}

impl From<u8> for Odr {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::Hz10,
            0b01 => Self::Hz20,
            0b10 => Self::Hz50,
            0b11 => Self::Hz100,
            _ => Self::Hz10,
        }
    }
}

impl From<Odr> for u8 {
    fn from(value: Odr) -> Self {
        value as u8
    }
}

/// Mode of operation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Mode {
    /// Continuous mode.
    Continuous = 0b00,
    /// Single measurement mode.
    Single = 0b01,
    /// Idle mode.
    Idle = 0b10,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::Continuous,
            0b01 => Self::Single,
            0b10 | 0b11 => Self::Idle,
            _ => Self::Idle,
        }
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        value as u8
    }
}

bitfield! {
    /// Configuration register A.
    ///
    /// The reset value is `0x03`, which selects idle mode.
    pub struct CfgRegA(u8);
    impl Debug;
    /// Enables magnetometer temperature compensation.
    pub comp_temp_en, set_comp_temp_en: 7;
    /// Reboots magnetometer memory content.
    pub reboot, set_reboot: 6;
    /// Resets configuration and user registers.
    pub soft_rst, set_soft_rst: 5;
    /// Enables low-power mode.
    pub lp, set_lp: 4;
    /// Output data rate configuration.
    pub from into Odr, odr, set_odr: 3, 2;
    /// Mode of operation.
    pub from into Mode, md, set_md: 1, 0;
}

impl CfgRegA {
    /// Create a register with its datasheet reset value.
    pub fn new() -> Self {
        Self(0x03)
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

impl Default for CfgRegA {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits() {
        let mut register = CfgRegA::default();
        register.set_comp_temp_en(true);
        register.set_reboot(true);
        register.set_soft_rst(true);
        register.set_lp(true);
        register.set_odr(Odr::Hz50);
        register.set_md(Mode::Continuous);

        assert_eq!(register.into_bytes(), [0xf8]);
        assert_eq!(CfgRegA::default().into_bytes(), [0x03]);
    }
}
