#![no_std]
#![no_main]

use cortex_m_rt as rt;
use panic_semihosting as _;
use stm32l4xx_hal as hal;

use core::fmt::{Debug, Write};
use cortex_m_semihosting::hio::{self};
use hal::{delay::Delay, i2c::I2c, prelude::*, stm32};
use rt::entry;

use iis2mdc::Iis2mdc;

#[entry]
fn main() -> ! {
    //==========================================
    // Initilizing board

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(8.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut delay = Delay::new(cp.SYST, clocks);

    //==========================================
    // Declaring I2C1

    let scl = gpiob
        .pb8
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let sda = gpiob
        .pb7
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1);

    //==============================================

    let mut stdout = hio::hstdout().unwrap();

    let mut sensor = match Iis2mdc::new(&mut i2c) {
        Ok(sensor) => sensor,
        Err(error) => {
            writeln!(stdout, "{:?}", error).unwrap();
            panic!("failed to create sensor")
        }
    };

    boot_sensor(&mut sensor, &mut i2c);

    loop {
        writeln!(stdout, "{}", sensor.get_temperature(&mut i2c).unwrap()).unwrap();
        writeln!(stdout, "{:?}", sensor.get_measurements(&mut i2c).unwrap()).unwrap();

        delay.delay_ms(500u32);
    }
}

// Configuring the sensor driver
fn boot_sensor<I2C, E>(sensor: &mut Iis2mdc, i2c: &mut I2C)
where
    I2C: embedded_hal::blocking::i2c::WriteRead<Error = E>
        + embedded_hal::blocking::i2c::Write<Error = E>,
    E: Debug,
{
    // =======================================
    // CFG_REG_A

    sensor.cfg_reg_a.set_reboot(i2c, true).unwrap();

    sensor
        .cfg_reg_a
        .set_data_rate(i2c, iis2mdc::cfg_reg_a::Odr::Hz50)
        .unwrap();

    sensor
        .cfg_reg_a
        .set_mode(i2c, iis2mdc::cfg_reg_a::Mode::Continuous)
        .unwrap();
}
