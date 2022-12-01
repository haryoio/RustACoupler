extern crate num;
extern crate rustfft;

use hound::WavReader;
use num::complex::Complex;
use popemodem::receiver::fftfreq;
use ringbuf::HeapRb;
use rustfft::FftPlanner;

fn find_peak_freqs(filename: &str) -> Vec<f32> {
    let mut reader = WavReader::open(filename).expect("Failed to open WAV file");
    let samplerate = 44100;
    let baudrate = 20;

    let bit_per_frames = samplerate / baudrate;

    let rb = HeapRb::<f32>::new(bit_per_frames * 2);
    // let (mut prod, mut cons) = rb.split();

    // for i in 0..bit_per_frames {
    //     prod.push(0.0).unwrap();
    // }

    let samples: Vec<f32> = reader.samples::<f32>().map(|s| s.unwrap()).collect();
    let sample_iter = samples.iter();

    let mut res = vec![];
    let mut buffer = vec![];
    for i in 0..sample_iter.len() {
        let buffer_len = buffer.len();
        if buffer_len >= bit_per_frames {
            res.push(fftfreq(buffer.clone(), samplerate as f32).unwrap());
            buffer.clear();
        } else {
            buffer.push(samples[i]);
        }
    }
    res
}

fn main() {
    let filename = "hello.wav";

    let peak = find_peak_freqs(&filename);

    // let mut res = vec![];
    // for resfreq in peak {
    //     let threshold = 300f32;
    //     if resfreq >= (1200f32 - threshold) && resfreq <= 1200f32 + threshold {
    //         res.push(0);
    //     } else if resfreq >= (2400f32 - threshold) && resfreq <= 2400f32 + threshold {
    //         res.push(1);
    //     } else {
    //         res.push(-1);
    //     }
    // }

    println!("{:?}", peak);
}
