#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::Timer;

use iis2mdc::Iis2mdc;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    //==========================================
    // Initilizing board

    let p = embassy_stm32::init(Default::default());

    //==========================================
    // Declaring I2C1

    let mut i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB7, Hertz(100_000), Default::default());

    //==============================================
    // Declaring sensor

    let mut sensor = match Iis2mdc::new(&mut i2c) {
        Ok(sensor) => sensor,
        Err(error) => {
            defmt::error!("Error: {:?}", error);
            panic!("failed to create sensor")
        }
    };

    // Initializing sensor
    boot_sensor(&mut sensor, &mut i2c).await;

    // =======================================

    loop {
        defmt::info!("Temperature: {}", sensor.get_temperature(&mut i2c).unwrap());
        defmt::info!(
            "Measurements: {:?}",
            sensor.get_measurements(&mut i2c).unwrap()
        );

        Timer::after_millis(1000).await;
    }
}

// Configuring the sensor driver
async fn boot_sensor<I2C>(sensor: &mut Iis2mdc, i2c: &mut I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    sensor.cfg_reg_a.set_soft_rst(i2c, true).unwrap();

    Timer::after_millis(1).await;

    sensor.cfg_reg_a.set_reboot(i2c, true).unwrap();

    Timer::after_millis(20).await;

    sensor
        .cfg_reg_a
        .set_data_rate(i2c, iis2mdc::cfg_reg_a::Odr::Hz50)
        .unwrap();

    sensor
        .cfg_reg_a
        .set_mode(i2c, iis2mdc::cfg_reg_a::Mode::Continuous)
        .unwrap();
}
