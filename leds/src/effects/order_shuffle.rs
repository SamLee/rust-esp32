use esp_idf_hal::rmt::VariableLengthSignal;
use log::info;
use rand::{seq::SliceRandom, thread_rng};

use crate::{Strand, rgb};

pub fn generate_signal(strand: &Strand, rgbs: &Vec<u32>) -> VariableLengthSignal {
    let mut shuffled_rgbs = rgbs.to_vec();
    shuffled_rgbs.shuffle(&mut thread_rng());
    info!("Colours: {:08X?}", shuffled_rgbs);
    let mut signal = VariableLengthSignal::new();
    for led in 0..strand.number_leds {
        let rgb = shuffled_rgbs[led as usize % shuffled_rgbs.len()];
        for i in 0..strand.bits_per_led {
            let bit = 2_u32.pow(i) & rgb != 0;
            let (high_pulse, low_pulse) = if bit { strand.on } else { strand.off };
            signal.push(&[high_pulse, low_pulse]).unwrap();
        }
    }

    return signal;
}

pub fn generate_default_signals(strand: &Strand) -> Vec<VariableLengthSignal> {
    let rgbs = vec![
        rgb(125, 0, 0), // Red
        rgb(125, 100, 0), // Yellow
        rgb(0, 125, 0), // Green
        rgb(0, 0, 125), // Blue
        rgb(125, 75, 0), // Orange
    ];

    let mut signals = Vec::new();

    for _ in 0..5 {
        signals.push(generate_signal(strand, &rgbs));
    }

    return signals;
}
