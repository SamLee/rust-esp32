mod effects;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use core::time::Duration;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::*;
use log::info;
use rand::seq::SliceRandom;
use rand::thread_rng;

// My LEDS come on at 4
const GAMMA8: [u32; 256] = [
            0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8,
            8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15,
            16, 16, 17, 17, 17, 18, 18, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25,
            25, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 32, 33, 33, 34, 35, 35, 36, 37,
            38, 39, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50, 51, 52, 54, 55, 56, 57, 58,
            59, 60, 61, 62, 63, 64, 66, 67, 68, 69, 70, 72, 73, 74, 75, 77, 78, 79, 81, 82, 83, 85,
            86, 87, 89, 90, 92, 93, 95, 96, 98, 99, 101, 102, 104, 105, 107, 109, 110, 112, 114,
            115, 117, 119, 120, 122, 124, 126, 127, 129, 131, 133, 135, 137, 138, 140, 142, 144,
            146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 167, 169, 171, 173, 175, 177, 180,
            182, 184, 186, 189, 191, 193, 196, 198, 200, 203, 205, 208, 210, 213, 215, 218, 220,
            223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 249, 252, 255,
];

#[derive(Debug)]
pub struct Strand {
    pub number_leds: u32,
    pub bits_per_led: u32,
    pub on: (Pulse, Pulse),
    pub off: (Pulse, Pulse),
}

pub fn rgb(r: i32, g:i32, b:i32) -> u32 {
    let rg = GAMMA8[r as usize].checked_shl(8).unwrap() + GAMMA8[g as usize];
    let rgb = rg.checked_shl(8).unwrap() + GAMMA8[b as usize];
    return rgb.reverse_bits().checked_shr(8).unwrap();
}

fn static_colour_all(strand: &Strand, rgb: u32) -> Vec<u32> {
    info!("Colour: {:08X}", rgb);
    return vec![rgb; strand.number_leds as usize];
}

fn vec_to_signal(strand: &Strand, rgbs: &Vec<u32>) -> VariableLengthSignal {
    // If strand length doesnt match rgbs length then default to off
    // info!("Colours: {:08X?}", rgbs);
    let mut signal = VariableLengthSignal::new();
    for led in 0..strand.number_leds {
        let rgb: u32 = if led >= rgbs.len() as u32 { 0u32 } else { rgbs[led as usize] };
        for i in 0..strand.bits_per_led {
            let bit = 2_u32.pow(i) & rgb != 0;
            let (high_pulse, low_pulse) = if bit { strand.on } else { strand.off };
            signal.push(&[high_pulse, low_pulse]).unwrap();
        }
    }

    return signal;
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals.pins.gpio4;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = TxRmtDriver::new(channel, led, &config)?;

    let number_leds: u32 = 50;
    let signal_length: u32 = 24;

    let ticks_hz = tx.counter_clock()?;
    let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(500))?;
    let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(2000))?;
    let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(1200))?;
    let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(1300))?;
    let strand = Strand {
        number_leds,
        bits_per_led: signal_length,
        on: (t1h, t1l),
        off: (t0h, t0l),
    };

    info!("Beginning rgb cycle");
    loop {
        let rgbs = vec![
            rgb(125, 0, 0), // Red
            rgb(125, 100, 0), // Yellow
            rgb(0, 125, 0), // Green
            rgb(0, 0, 125), // Blue
            rgb(125, 75, 0), // Orange
        ];

        for _ in 0..25 {
            // let reds = (0..10).map(|_| rgb(127, 0, 0)).collect::<Vec<u32>>();
            // let yellows = (0..10).map(|_| rgb(127, 100, 0)).collect::<Vec<u32>>();
            // let oranges = (0..10).map(|_| rgb(127, 75, 0)).collect::<Vec<u32>>();
            // let greens = (0..10).map(|_| rgb(0, 127, 0)).collect::<Vec<u32>>();
            // let blues = (0..10).map(|_| rgb(0, 0, 127)).collect::<Vec<u32>>();
            // let mut manual_rgbs = vec![reds, yellows, oranges, greens, blues];
            // manual_rgbs.shuffle(&mut thread_rng());
            let signal = vec_to_signal(&strand, &effects::ribbon_shuffle::generate(&strand));
            tx.start_blocking(&signal)?;
            Ets::delay_ms(1000);
        }

        for signal in effects::order_shuffle::generate_default_signals(&strand) {
            tx.start_blocking(&signal)?;
            Ets::delay_ms(1000);
        }

        for i in 0..rgbs.len() * 5 {
            let colour = static_colour_all(&strand, rgbs[i % rgbs.len()]);
            let signal = vec_to_signal(&strand, &colour);
            tx.start_blocking(&signal)?;
            Ets::delay_ms(1000);
        }

        // This has random flashes on individual leds
        // for j in 0..rgbs.len() * 2 {
        //     // Ramp up
        //     for i in (25..125).step_by(2) {
        //         let rgbs = vec![
        //             rgb(i, 0, 0), // Red
        //             rgb(i, (i as f64 * 0.8) as i32, 0), // Yellow
        //             rgb(0, i, 0), // Green
        //             rgb(0, 0, i), // Blue
        //             rgb(i, (i as f64 * 0.8) as i32, 0), // Orange
        //         ];
        //         let strip_state = static_colour_all(&strand, rgbs[j % rgbs.len()]);
        //         let signal = manual(&strand, &strip_state);
        //         tx.start_blocking(&signal)?;
        //         Ets::delay_ms(8);
        //     }

        //     for i in (25..125).step_by(2).rev() {
        //         let rgbs = vec![
        //             rgb(i, 0, 0), // Red
        //             rgb(i, (i as f64 * 0.8) as i32, 0), // Yellow
        //             rgb(0, i, 0), // Green
        //             rgb(0, 0, i), // Blue
        //             rgb(i, (i as f64 * 0.8) as i32, 0), // Orange
        //         ];
        //         let strip_state = static_colour_all(&strand, rgbs[j % rgbs.len()]);
        //         let signal = manual(&strand, &strip_state);
        //         tx.start_blocking(&signal)?;
        //         Ets::delay_ms(8);
        //     }
        // }
    }
}
