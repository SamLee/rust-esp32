use esp_idf_hal::gpio::{Gpio17, Gpio18};
use esp_idf_hal::i2c::{I2cConfig, I2cDriver, I2C0};
use log::{info, error};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*, mode::BufferedGraphicsMode};
use esp_idf_hal::prelude::*;

pub fn get_display(i2c0: I2C0, sda: Gpio17, scl: Gpio18) -> Ssd1306<I2CInterface<I2cDriver<'static>>, ssd1306::prelude::DisplaySize128x64, BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>> { 
    info!("Starting I2C SSD1306");

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c0, sda, scl, &config).unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180).into_buffered_graphics_mode();

    loop {
        if let Err(e) = display.init() {
            error!("Failed to init display: {:?}", e);
        } else {
            info!("Display initialised");
            return display;
        }
    }
}
