use std::time::Duration;

use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::prelude::*;
use log::{info, error};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X18_BOLD},
        MonoTextStyleBuilder,
    },
    prelude::*,
    text::{Alignment, Text},
    pixelcolor::BinaryColor
};


fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let mut reset = PinDriver::output(peripherals.pins.gpio21).unwrap();
    let sda = peripherals.pins.gpio17;
    let scl = peripherals.pins.gpio18;

    info!("Starting I2C SSD1306");

    // BusWriteError if this sint done
    reset.set_high().unwrap();

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180).into_buffered_graphics_mode();

    loop {
        if let Err(e) = display.init() {
            error!("Failed to init display: {:?}", e);
        } else {
            info!("Display initialised");
            break; 
        }
    }

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let text_style_big = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    loop {
        Text::with_alignment(
            "esp-hal",
            display.bounding_box().center() + Point::new(0, 0),
            text_style_big,
            Alignment::Center
        ).draw(&mut display).unwrap();

        Text::with_alignment(
            "Chip: ESP32S3",
            display.bounding_box().center() + Point::new(0, 14),
            text_style,
            Alignment::Center,
        ).draw(&mut display).unwrap();

        display.flush().unwrap();
        display.clear();

        std::thread::sleep(Duration::from_millis(5000));

        Text::with_alignment(
            "Hello World!",
            display.bounding_box().center(),
            text_style_big,
            Alignment::Center,
        ).draw(&mut display).unwrap();

        display.flush().unwrap();
        display.clear();

        std::thread::sleep(Duration::from_millis(5000));
    }
}
