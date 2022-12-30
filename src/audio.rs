use std::{f64::consts::PI, i32::MAX};

pub const SAMPLE_RATE: u32 = 44100;

fn genvec(duration: f64) -> Vec<i32> {
    let arr: Vec<i32> = vec![0; (duration * SAMPLE_RATE as f64) as usize];
    arr
}
/// Sine wave generator. Returns samples for a given sample rate.
///
/// Dev tip: remove some float precision for a real robot sound!
pub fn sine(duration: f64, frequency: f64) -> Vec<i32> {
    dbg!(&frequency);
    let mut samples = genvec(duration);
    for i in 0..samples.capacity() {
        samples[i] =
            (MAX as f64 * (i as f64 * frequency * 2.0 * PI / SAMPLE_RATE as f64).sin()) as i32;
    }
    samples
}

pub fn silence(duration: f64) -> Vec<i32> {
    genvec(duration)
}
