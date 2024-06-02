use rand::{seq::SliceRandom, thread_rng};

use crate::{Strand, rgb};

/**
 * Generate a vector of u32s representing the rgb values for each led
 */
pub fn generate(strand: &Strand) -> Vec<u32> {
    let per_ribbon = strand.number_leds / 5;
    let remainder = strand.number_leds % 5;
    let reds = vec![rgb(127,0,0); per_ribbon as usize];
    let yellows = vec![rgb(127,100,0); per_ribbon as usize];
    let oranges = vec![rgb(127,75,0); per_ribbon as usize];
    let greens = vec![rgb(0,127,0); per_ribbon as usize];
    let blues = vec![rgb(0,0,127); per_ribbon as usize];

    let mut manual_rgbs = vec![reds, yellows, oranges, greens, blues];
    manual_rgbs.shuffle(&mut thread_rng());
    let mut leds = manual_rgbs.concat();
    let last = leds.last().unwrap().clone();
    
    for _ in 0..remainder {
        leds.push(last);
    }
    
    return leds;
}
