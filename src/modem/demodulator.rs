use std::{f32::consts::PI, sync::Arc};

use nalgebra::{Complex, UnitComplex};
use num::complex::Complex32;

use super::filter::fft;
use crate::config::Band;

pub fn which_band(freq: f32, bands: Vec<Band>) -> i8 {
    let mut res = -1;
    for band in bands.iter() {
        let bit = detect_bfsk(
            freq,
            band.carrier() as f32,
            band.deviation() as f32,
            band.threshold() as f32,
        );
        if bit >= 0 {
            // println!("{:?}", band);
            res = bit;
            break;
        }
    }
    res
}

pub fn detect_bfsk(freq: f32, carrier: f32, deviation: f32, threshold: f32) -> i8 {
    let in_low_range =
        move |res: f32| -> bool { res >= (carrier - threshold) && res <= carrier + threshold };

    let in_high_range = move |res: f32| -> bool {
        res >= (carrier + deviation - threshold) && res <= carrier + deviation + threshold
    };

    if in_low_range(freq) {
        0
    } else if in_high_range(freq) {
        1
    } else {
        -1
    }
}

pub fn detect_qfsk(freq: f32, carrier: f32, deviation: f32, threshold: f32) -> (i8, i8) {
    let freq_1 = carrier;
    let freq_2 = carrier + deviation;
    let freq_3 = carrier + deviation * 2.0;
    let freq_4 = carrier + deviation * 3.0;
    let freq_is_in_range = move |res: f32, carrier: f32| -> bool {
        res >= (carrier - threshold) && res <= freq_1 + threshold
    };
    if freq_is_in_range(freq, freq_1) {
        (0, 0)
    } else if freq_is_in_range(freq, freq_2) {
        (0, 1)
    } else if freq_is_in_range(freq, freq_3) {
        (1, 0)
    } else if freq_is_in_range(freq, freq_4) {
        (1, 1)
    } else {
        (-1, -1)
    }
}
