mod display;
mod wifi;

use std::time::Duration;

use esp_idf_hal::{peripherals::Peripherals, gpio::PinDriver};
use log::info;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X18_BOLD},
        MonoTextStyleBuilder,
    },
    prelude::*,
    text::{Alignment, Text},
    pixelcolor::BinaryColor,
};

use crate::{display::get_display, wifi::get_wifi};

#[toml_cfg::toml_config]
pub struct CONFIG {
    #[default("Oh Pippin...")]
    wifi_ssid: &'static str,
    #[default("sausages")]
    wifi_pawssword: &'static str,
}

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("{}", CONFIG.wifi_ssid);

    let peripherals = Peripherals::take().unwrap();
    let reset = peripherals.pins.gpio21;
    let sda = peripherals.pins.gpio17;
    let scl = peripherals.pins.gpio18;
    let i2c0 = peripherals.i2c0;
    let modem = peripherals.modem;

    // BusWriteError if this isnt done
    let mut reset_pin = PinDriver::output(reset).unwrap();
    reset_pin.set_high().unwrap();

    let mut display = get_display(i2c0, sda, scl);
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let text_style_big = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();


    let wifi = get_wifi(modem, CONFIG.wifi_ssid.to_string(), CONFIG.wifi_pawssword.to_string());
    info!("IP info: {:?}", wifi.sta_netif().get_ip_info().unwrap());

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

        Text::with_alignment(
            wifi.sta_netif().get_ip_info().unwrap().ip.to_string().as_str(),
            display.bounding_box().center(),
            text_style_big,
            Alignment::Center,
        ).draw(&mut display).unwrap();

        display.flush().unwrap();
        display.clear();

        std::thread::sleep(Duration::from_millis(5000));

    }
}
