use std::f32::consts::PI;

use itertools_num::ItertoolsNum;

use crate::utils::repeat;

//continuous phase frequency shift keying
pub fn cpfsk(data: Vec<f32>, samplerate: u32, latency: usize) -> Vec<f32> {
    let res = repeat(data, latency);
    res.iter()
        .map(|d| (d * PI / (samplerate as f32 / 2.0)))
        .cumsum()
        .collect::<Vec<f32>>()
        .iter()
        .map(|d| d.sin() * i16::MAX as f32)
        .collect::<Vec<f32>>()
}

pub fn gfsk(data: Vec<f32>, samplerate: u32, baudrate: usize) -> Vec<f32> {
    let latency: usize = (1.0 / baudrate as f32 * samplerate as f32) as usize;

    let res = repeat(data, latency);

    let mut out = Vec::new();
    let mut t = 0.0;
    for d in res {
        t += 1.0 / samplerate as f32;
        let sample = (d * PI * 2.0 * t).sin();
        out.push(sample * i16::MAX as f32);
    }

    out
}

/// generate binary frequency shift keying wave
/// Example:
/// ```rust
/// let samplerate = 44100;
/// let baudrate = 100;
/// let latency = 1.0 / baudrate * samplerate;
/// let high_freq = 440.0;
/// let low_freq = 220.0;
///
/// let bin = vec![0, 1, 0, 1, 0, 1, 0, 1];
/// let wave = bfsk(&bin, samplerate, latency, high_freq, low_freq);
/// ```
pub fn bfsk(
    bin: &Vec<u8>,
    career: f32,
    deviation: f32,
    samplerate: u32,
    latency: usize,
) -> Vec<f32> {
    let mut buf = vec![];
    // println!("bin: {:?}", bin);
    for b in bin {
        match b {
            0 => buf.push(career),
            1 => buf.push(career + deviation),
            _ => buf.push(0.0),
        }
    }

    cpfsk(buf, samplerate, latency)
}

/// generate quadrature frequency shift keying wave
/// Example:
/// ```rust
/// let samplerate = 44100;
/// let baudrate = 100;
/// let latency = 1.0 / baudrate * samplerate;

/// let low_freq = 220.0;
/// let bin = vec![0, 1, 0, 1, 0, 1, 0, 1];
/// let wave = qfsk(&bin, low_freq, samplerate, latency);
/// ```
pub fn qfsk(bin: &[u8], career: f32, samplerate: u32, baudrate: usize) -> Vec<f32> {
    let mut buf = vec![];
    for a in bin.chunks(2) {
        let b = a[0];
        let c = a[1];
        match (b, c) {
            (0, 0) => buf.push(career),
            (0, 1) => buf.push(career + 400f32),
            (1, 0) => buf.push(career + 1200f32),
            (1, 1) => buf.push(career + 2400f32),
            _ => buf.push(0.0),
        }
    }

    cpfsk(buf, samplerate, baudrate)
}

/// generate phase shift keying wave
/// Example:
pub fn psk(data: Vec<u8>, samplerate: u32, latency: usize) -> Vec<f32> {
    let mut bin = vec![];
    let zeros = vec![0, 0, 0, 0, 0, 0, 0, 0];
    bin.extend(zeros.clone());
    bin.extend(zeros);
    bin.extend(data);
    let freq = 1000f32;
    let amp = i16::MAX as f32;
    let mut buf = Vec::new();
    let t = 1.0 / samplerate as f32;
    let mut phase = 0f32;
    // 1bit につき latency 個のサンプルを生成する
    for b in bin {
        for _ in 0..latency {
            phase += 2.0 * PI * freq * t;
            if b == 1 {
                phase += PI;
            }
            buf.push(phase.sin() * amp);
        }
    }
    buf
}
