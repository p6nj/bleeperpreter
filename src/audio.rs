use std::f64::consts::PI;

pub const SAMPLE_RATE: u32 = 44100;

fn genvec(duration: f64) -> Vec<f32> {
    let arr: Vec<f32> = Vec::with_capacity((duration * SAMPLE_RATE as f64) as usize);
    arr.fill_with(Default::default);
    arr
}
/// Sine wave generator. Returns samples for a given sample rate.
///
/// Dev tip: remove some float precision for a real robot sound!
pub fn sine(duration: f64, frequency: f64) -> Vec<f32> {
    let mut samples = genvec(duration);
    for i in 0..samples.capacity() {
        samples[i] = (0.5 * (i as f64 * frequency * 2.0 * PI / SAMPLE_RATE as f64).sin()) as f32;
    }
    samples
}

pub fn silence(duration: f64) -> Vec<f32> {
    genvec(duration)
}
