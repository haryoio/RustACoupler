use std::f32::consts::PI;

use num::Complex;
use rustfft::FftPlanner;

#[inline]
fn gaussian(sigma: f32, x: f32) -> f32 {
    let exp = -1.0 * (x.powf(2.0) / (2.0 * sigma).powf(2.0));
    let div = (2.0 * PI * sigma.powf(2.0)).sqrt();
    (1.0 / div) * exp.exp()
}

#[inline]
fn gaussian_kernel(samples: isize, sigma: f32) -> Vec<f32> {
    let mut v = Vec::new();

    let double_center = samples % 2 == 0;
    let samples = if double_center { samples - 1 } else { samples };
    let steps = (samples - 1) / 2;
    let step_size = (3.0 * sigma) / steps as f32;

    for i in (1..=steps).rev() {
        v.push(gaussian(sigma, i as f32 * step_size * -1.0));
    }

    v.push(gaussian(sigma, 0.0));
    if double_center {
        v.push(gaussian(sigma, 0.0));
    }

    for i in 1..=steps {
        v.push(gaussian(sigma, i as f32 * step_size));
    }

    v
}

#[inline]
pub fn gaussian_smooth(values: &[f32], sigma: f32, samples: isize) -> Vec<f32> {
    let mut out = Vec::new();
    let kernel = gaussian_kernel(samples, sigma);
    let sample_side = samples / 2;
    let ubound = values.len();
    for i in 0..ubound {
        let mut sample = 0.0;
        let mut sample_ctr = 0;
        for j in (i as isize - sample_side)..=(i as isize + sample_side) {
            if j > 0 && j < ubound as isize {
                let sample_weight_index = sample_side + (j - i as isize);
                sample += kernel[sample_weight_index as usize] * values[j as usize];
                sample_ctr += 1;
            }
        }
        let smoothed = sample / sample_ctr as f32;
        out.push(smoothed);
    }
    out
}
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

// #[inline]
pub fn fftfreq(data: Vec<f32>, samplerate: f32) -> Option<f32> {
    let num_samples = data.len();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);
    let mut signal = data
        .iter()
        .map(|x| Complex::new(*x, -x))
        .collect::<Vec<_>>();

    fft.process(&mut signal[..]);

    let max_peak = signal
        .iter()
        .take(num_samples / 4)
        .enumerate()
        .max_by_key(|&(_, freq)| freq.norm() as u32);

    if let Some((i, _)) = max_peak {
        let bin = samplerate / num_samples as f32;
        Some(i as f32 * bin)
    } else {
        None
    }
}
