//! Register modules and address definitions.

pub mod cfg_reg_a;
pub mod cfg_reg_b;
pub mod cfg_reg_c;
pub mod int_ctrl_reg;
pub mod int_source_reg;
pub mod int_ths;
pub mod offset;
pub mod out_mag;
pub mod out_temp;
pub mod status_reg;

pub use cfg_reg_a::*;
pub use cfg_reg_b::*;
pub use cfg_reg_c::*;
pub use int_ctrl_reg::*;
pub use int_source_reg::*;
pub use int_ths::*;
pub use offset::*;
pub use out_mag::*;
pub use out_temp::*;
pub use status_reg::*;

/// Register addresses.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// X-axis hard-iron offset (low)
    OffsetXRegL = 0x45,
    /// X-axis hard-iron offset (high)
    OffsetXRegH = 0x46,
    /// Y-axis hard-iron offset (low)
    OffsetYRegL = 0x47,
    /// Y-axis hard-iron offset (high)
    OffsetYRegH = 0x48,
    /// Z-axis hard-iron offset (low)
    OffsetZRegL = 0x49,
    /// Z-axis hard-iron offset (high)
    OffsetZRegH = 0x4A,
    /// Who am I register
    WhoAmI = 0x4F,
    /// Configuration register A
    CfgRegA = 0x60,
    /// Configuration register B
    CfgRegB = 0x61,
    /// Configuration register C
    CfgRegC = 0x62,
    /// Interrupt control register
    IntCtrlReg = 0x63,
    /// Interrupt source register
    IntSourceReg = 0x64,
    /// Interrupt threshold (low)
    IntThsLReg = 0x65,
    /// Interrupt threshold (high)
    IntThsHReg = 0x66,
    /// Status register
    StatusReg = 0x67,
    /// X-axis output register (low)
    OutXRegL = 0x68,
    /// X-axis output register (high)
    OutXRegH = 0x69,
    /// Y-axis output register (low)
    OutYRegL = 0x6A,
    /// Y-axis output register (high)
    OutYRegH = 0x6B,
    /// Z-axis output register (low)
    OutZRegL = 0x6C,
    /// Z-axis output register (high)
    OutZRegH = 0x6D,
    /// Temperature output register (low)
    TempOutLReg = 0x6E,
    /// Temperature output register (high)
    TempOutHReg = 0x6F,
}

impl Register {
    /// Return the register address.
    pub const fn addr(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn register_addresses_match_datasheet() {
        let addresses = [
            (Register::OffsetXRegL, 0x45),
            (Register::OffsetXRegH, 0x46),
            (Register::OffsetYRegL, 0x47),
            (Register::OffsetYRegH, 0x48),
            (Register::OffsetZRegL, 0x49),
            (Register::OffsetZRegH, 0x4a),
            (Register::WhoAmI, 0x4f),
            (Register::CfgRegA, 0x60),
            (Register::CfgRegB, 0x61),
            (Register::CfgRegC, 0x62),
            (Register::IntCtrlReg, 0x63),
            (Register::IntSourceReg, 0x64),
            (Register::IntThsLReg, 0x65),
            (Register::IntThsHReg, 0x66),
            (Register::StatusReg, 0x67),
            (Register::OutXRegL, 0x68),
            (Register::OutXRegH, 0x69),
            (Register::OutYRegL, 0x6a),
            (Register::OutYRegH, 0x6b),
            (Register::OutZRegL, 0x6c),
            (Register::OutZRegH, 0x6d),
            (Register::TempOutLReg, 0x6e),
            (Register::TempOutHReg, 0x6f),
        ];

        for (register, address) in addresses {
            assert_eq!(register.addr(), address);
        }
    }
}
