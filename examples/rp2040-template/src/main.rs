#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};
use embassy_rp::{
    self,
    i2c::Config,
};
use bmp280_embedded_hal::BMP280;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let p = embassy_rp::init(Default::default());

    let scl = p.PIN_15;
    let sda = p.PIN_14;

    let i2c = embassy_rp::i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());

    let mut bmp280 = BMP280::new(i2c).unwrap();

    loop {
        let temperature = bmp280.temp().unwrap();
        let pressure = bmp280.pressure().unwrap();
        let altitude = bmp280.altitude().unwrap();

        defmt::info!("Temp: {} C", temperature);
        defmt::info!("Pressure: {} Pa", pressure);
        defmt::info!("Altitude: {} m", altitude);

        cortex_m::asm::delay(1_000_000);
    }
}
