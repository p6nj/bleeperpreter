use std::i32::MAX;
// Dev tip: remove some float precision for a real robot sound!

pub const SAMPLE_RATE: u32 = 44100;

#[allow(dead_code)]
pub enum Signal {
    Sine,
    Square,
}

impl Signal {
    fn generate(&self, duration: f64, frequency: f32) -> Vec<i32> {
        let mut samples = genvec(duration);
        let freq = real(frequency);
        for i in 0..samples.capacity() {
            let curr = i as f64 * freq;
            samples[i] = (MAX as f64
                * match self {
                    Self::Sine => sine(curr),
                    Self::Square => square(curr),
                }) as i32;
        }
        samples
    }
}

fn genvec(duration: f64) -> Vec<i32> {
    let arr: Vec<i32> = vec![0; (duration * SAMPLE_RATE as f64) as usize];
    arr
}

fn real(frequency: f32) -> f64 {
    frequency as f64 / SAMPLE_RATE as f64
}

/// Sine wave generator. Returns samples ready to write.
fn sine(x: f64) -> f64 {
    x.sin()
}

/// Square wave generator. Returns samples ready to write.
fn square(x: f64) -> f64 {
    babalcore::square_wave(x)
}

/// Silence wave generator. Returns null samples ready to write.
pub fn silence(duration: f64) -> Vec<i32> {
    genvec(duration)
}

pub fn signal(_type: Signal, duration: f64, frequency: f32) -> Vec<i32> {
    _type.generate(duration, frequency)
}
