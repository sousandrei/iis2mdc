use embedded_hal::i2c::I2c;

use crate::Iis2mdc;
use crate::registers::{CfgRegA, CfgRegB, CfgRegC, Mode, Odr, Register};

/// Core IIS2MDC configuration methods.
pub trait Configuration {
    fn set_comp_temp_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_reboot<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_soft_rst<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_lp<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_odr<I2C>(&self, i2c: &mut I2C, odr: Odr) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_md<I2C>(&self, i2c: &mut I2C, mode: Mode) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_off_canc_one_shot<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int_on_dataoff<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_set_freq<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_off_canc<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_lpf<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int_on_pin<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_i2c_dis<I2C>(&self, i2c: &mut I2C, disable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_bdu<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ble<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_self_test<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_drdy_on_pin<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

macro_rules! set_bool {
    ($method:ident, $register:ident, $type:ident, $setter:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
        where
            I2C: I2c,
        {
            self.modify_reg(i2c, Register::$register, |value| {
                let mut register = $type::from_bytes([value]);
                register.$setter(enable);
                register.into_bytes()[0]
            })
        }
    };
}

impl Configuration for Iis2mdc {
    set_bool!(set_comp_temp_en, CfgRegA, CfgRegA, set_comp_temp_en);
    set_bool!(set_reboot, CfgRegA, CfgRegA, set_reboot);
    set_bool!(set_soft_rst, CfgRegA, CfgRegA, set_soft_rst);
    set_bool!(set_lp, CfgRegA, CfgRegA, set_lp);
    set_bool!(
        set_off_canc_one_shot,
        CfgRegB,
        CfgRegB,
        set_off_canc_one_shot
    );
    set_bool!(set_int_on_dataoff, CfgRegB, CfgRegB, set_int_on_dataoff);
    set_bool!(set_set_freq, CfgRegB, CfgRegB, set_set_freq);
    set_bool!(set_off_canc, CfgRegB, CfgRegB, set_off_canc);
    set_bool!(set_lpf, CfgRegB, CfgRegB, set_lpf);
    set_bool!(set_int_on_pin, CfgRegC, CfgRegC, set_int_on_pin);
    set_bool!(set_i2c_dis, CfgRegC, CfgRegC, set_i2c_dis);
    set_bool!(set_bdu, CfgRegC, CfgRegC, set_bdu);
    set_bool!(set_ble, CfgRegC, CfgRegC, set_ble);
    set_bool!(set_self_test, CfgRegC, CfgRegC, set_self_test);
    set_bool!(set_drdy_on_pin, CfgRegC, CfgRegC, set_drdy_on_pin);

    fn set_odr<I2C>(&self, i2c: &mut I2C, odr: Odr) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |value| {
            let mut register = CfgRegA::from_bytes([value]);
            register.set_odr(odr);
            register.into_bytes()[0]
        })
    }

    fn set_md<I2C>(&self, i2c: &mut I2C, mode: Mode) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CfgRegA, |value| {
            let mut register = CfgRegA::from_bytes([value]);
            register.set_md(mode);
            register.into_bytes()[0]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use std::vec;

    #[test]
    fn set_odr_preserves_other_configuration_bits() {
        let sensor = Iis2mdc {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::CfgRegA.addr()],
                vec![0xe3],
            ),
            Transaction::write(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::CfgRegA.addr(), 0xeb],
            ),
        ]);

        sensor.set_odr(&mut i2c, Odr::Hz50).unwrap();
        i2c.done();
    }
}
