use std::f32::consts::PI;

use num::Complex;
use rustfft::FftPlanner;

#[inline]
pub fn lowpass_filter(data: &Vec<f32>, samplerate: f32, f0: f32) -> Vec<f32> {
    use biquad::*;
    let sr = samplerate.hz();
    let f0 = f0.hz();

    let coeffs =
        Coefficients::<f32>::from_params(Type::SinglePoleLowPass, sr, f0, Q_BUTTERWORTH_F32)
            .unwrap();
    let mut biquad1 = DirectForm1::<f32>::new(coeffs);
    let mut res = vec![];
    for sample in data {
        res.push(biquad1.run(*sample));
    }
    res
}
#[inline]
pub fn highpass_filter(data: &Vec<f32>, samplerate: f32, f0: f32) -> Vec<f32> {
    use biquad::*;
    let sr = samplerate.hz();
    let f0 = f0.hz();

    let coeffs =
        Coefficients::<f32>::from_params(Type::HighPass, sr, f0, Q_BUTTERWORTH_F32).unwrap();
    let mut biquad1 = DirectForm1::<f32>::new(coeffs);
    let mut res = vec![];
    for sample in data {
        res.push(biquad1.run(*sample));
    }
    res
}

// #[inline]
pub fn fft(data: Vec<f32>) -> Vec<Complex<f32>> {
    let num_samples = data.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);
    let mut signal = data
        .iter()
        .map(|x| Complex::new(*x, *x))
        .collect::<Vec<_>>();

    fft.process(&mut signal[..]);
    signal
}

pub fn fftfreq(data: &[f32], samplerate: u32) -> Option<f32> {
    let num_samples = data.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);
    let mut signal = data
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<_>>();

    fft.process(&mut signal[..]);

    let max_peak = signal
        .iter()
        .take(num_samples / 4)
        .enumerate()
        .max_by_key(|&(_, freq)| freq.norm() as u32);

    if let Some((i, _)) = max_peak {
        let bin = samplerate as f32 / num_samples as f32;
        Some(i as f32 * bin)
    } else {
        None
    }
}
