use anyhow::Result;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
pub(crate) fn resample(source: Vec<f32>, from: f64, to: f64) -> Result<Vec<f32>> {
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<f32>::new(to / from, 2.0, params, 1124, 1)?;

    let wave_in = source;
    const BUFSIZE: usize = 1024;
    Ok(wave_in
        .chunks_exact(BUFSIZE)
        .map(|win| -> Result<[f32; BUFSIZE]> {
            let mut wout = [[0f32; BUFSIZE]];
            resampler
                .process_into_buffer(&[win], &mut wout, None)
                .unwrap();
            Ok(wout[0].to_owned())
        })
        .collect::<Result<Vec<[f32; BUFSIZE]>>>()?
        .concat())
}
