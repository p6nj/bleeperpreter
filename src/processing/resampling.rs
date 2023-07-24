use anyhow::Result;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
pub fn resample(source: Vec<f32>, from: f64, to: f64) -> Result<Vec<f32>> {
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<f32>::new(to / from, 2.0, params, 1024, 1)?;

    let waves_in = source;
    Ok(resampler
        .process(&vec![waves_in], None)?
        .get(0)
        .unwrap()
        .clone())
}
