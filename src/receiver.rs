use core::panic;
use std::{
    collections::VecDeque,
    f32::consts::PI,
    fs::File,
    io::{self, BufReader, Read, Write},
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};

use biquad::*;
use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use itertools::Itertools;
use nalgebra::Complex;
use portaudio as pa;
use ringbuf::HeapRb;
use rustfft::FftPlanner;

use crate::{
    ascii::decode_u8,
    config::ModemConfig,
    hamming::Hamming::correct_hamming_code,
    save::save_wav,
    ModulationMethod,
    Status,
    ANSWER_TONE,
    ISFD,
};

#[derive(Clone)]
pub struct Receiver {
    pub(crate) config: ModemConfig,
}

impl Receiver {
    pub fn new(config: ModemConfig) -> Receiver {
        return Receiver { config };
    }

    pub fn run(&mut self) -> Result<(), pa::Error> {
        let mut status = Arc::new(Mutex::new(Status::LISTENING));

        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .expect("failed to find input device");
        let output_device = host
            .default_output_device()
            .expect("failed to find output device");

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream_config = StreamConfig {
            channels:    1,
            sample_rate: cpal::SampleRate(self.config.samplerate as u32),
            buffer_size: cpal::BufferSize::Fixed(self.config.latency() as u32),
        };

        let ind = self.config.latency() as usize;
        let ring = HeapRb::<Vec<f32>>::new(ind);
        let (mut in_producer, mut in_consumer) = ring.split();

        let out_ring = HeapRb::<f32>::new(ind);
        let (mut out_producer, mut out_consumer) = out_ring.split();

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Err(e) = in_producer.push(data.to_vec()) {
                eprintln!("ERROR: {:?}", e);
            }
        };

        let output_status = Arc::clone(&status);
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                if let Some(da) = out_consumer.pop() {
                    *sample = da;
                } else {
                    *sample = 0.0;
                }
            }
        };

        let input_stream = input_device
            .build_input_stream(&stream_config, input_data_fn, err_fn)
            .expect("failed to build input stream");
        let output_stream = output_device
            .build_output_stream(&stream_config, output_data_fn, err_fn)
            .expect("failed to build output stream");

        input_stream.play().expect("cannot start input stream");
        output_stream.play().expect("cannot start output stream");

        let co = self.clone();
        let mut handles = vec![];

        let mut recent_bin: VecDeque<i8> = vec![-1; 8].into();
        let mut input_data = vec![];
        let status = Arc::clone(&status);
        handles.push(thread::spawn(move || {
            'main: loop {
                if let Some(sample) = in_consumer.pop() {
                    let resfreq = fftfreq(sample, co.config.samplerate)
                        .expect("failed to get max frequency.");
                    // println!("resfreq: {}", resfreq);

                    let bit = match co.config.modulation_method {
                        ModulationMethod::BFSK => {
                            let bit = co.detect_bfsk_carrier(resfreq);

                            recent_bin.push_back(bit);
                            recent_bin.pop_front();

                            vec![bit]
                        }
                        ModulationMethod::QFSK => {
                            let bits = co.detect_qfsk_carrier(resfreq);
                            recent_bin.pop_front();
                            recent_bin.pop_front();
                            recent_bin.push_back(bits[1]);
                            recent_bin.push_back(bits[0]);

                            bits
                        }
                    };
                    // println!("bit: {:?}", bit);

                    let status_l = *status.lock().unwrap();
                    match status_l {
                        Status::LISTENING => {
                            // println!("rec {:?}", recent_bin);
                            if co.check_syn(&recent_bin) {
                                recent_bin = vec![0; 8].into();
                                {
                                    *status.lock().unwrap() = Status::RECEIVING;
                                }
                            }
                        }
                        Status::RECEIVING => {
                            input_data.extend(bit);
                            if input_data.len() % 8 == 0 && co.check_syn(&recent_bin) {
                                let correct_data = correct_hamming_code(
                                    input_data.clone().iter().map(|d| *d as u8).collect_vec(),
                                );
                                // println!("{:?}", correct_data);
                                let decoded: String = decode_u8(correct_data).into_iter().collect();
                                println!("{}", decoded);
                                *status.lock().unwrap() = Status::ANSWER;
                                input_data.clear();
                            }
                        }
                        Status::ANSWER => {
                            let answer_wave =
                                make_sine_wave(ANSWER_TONE, 5.0, co.config.samplerate);
                            for sample in answer_wave {
                                if let Err(e) = out_producer.push(sample) {
                                    break;
                                }
                            }
                            *status.lock().unwrap() = Status::LISTENING;
                        }
                    }
                }
            }
        }));

        for handle in handles {
            handle.join().unwrap();
        }
        Ok(())
    }

    fn check_syn(&self, data: &VecDeque<i8>) -> bool {
        let mut ok_syn = true;
        let data = data.clone();
        for i in 0..8 {
            if ISFD[i] != data[i] {
                ok_syn = false;
                break;
            }
        }
        ok_syn
    }

    fn detect_qfsk_carrier(&self, freq: f32) -> Vec<i8> {
        let threshold = self.config.threshold.clone();
        let in_range = |res_freq: f32, target_freq: f32| -> bool {
            res_freq >= (target_freq - threshold) && res_freq <= target_freq + threshold
        };

        let z_z = self.config.low_freq;
        let z_o = self.config.low_freq * 1.5;
        let o_z = self.config.low_freq * 2.0;
        let o_o = self.config.low_freq * 2.5;

        if in_range(freq, z_z) {
            return vec![0, 0];
        } else if in_range(freq, z_o) {
            return vec![0, 1];
        } else if in_range(freq, o_z) {
            return vec![1, 0];
        } else if in_range(freq, o_o) {
            return vec![1, 1];
        } else {
            return vec![-1, -1];
        }
    }

    fn detect_bfsk_carrier(&self, freq: f32) -> i8 {
        let threshold = self.config.threshold.clone();
        let in_low_range = move |res: f32| -> bool {
            res >= (self.config.low_freq - threshold) && res <= self.config.low_freq + threshold
        };
        let in_high_range = move |res: f32| -> bool {
            res >= (self.config.high_freq - threshold) && res <= self.config.high_freq + threshold
        };

        if in_low_range(freq) {
            return 0;
        } else if in_high_range(freq) {
            return 1;
        } else {
            return -1;
        }
    }
}

pub fn fftfreq(data: Vec<f32>, sr: f32) -> Option<f32> {
    let num_samples = data.len() as usize;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_samples);
    let signal = data
        .iter()
        .map(|x| Complex::new(*x as f32, 0f32))
        .collect::<Vec<_>>();
    let mut spectrum = signal.clone();
    fft.process(&mut spectrum[..]);
    let max_peak = spectrum
        .iter()
        .take(num_samples / 4)
        .enumerate()
        .max_by_key(|&(_, freq)| freq.norm() as u32);
    if let Some((i, _)) = max_peak {
        let bin = sr / num_samples as f32;
        Some(i as f32 * bin)
    } else {
        None
    }
}
fn make_sine_wave(freq: f32, duration: f32, sr: f32) -> Vec<f32> {
    let mut wave = vec![];
    let samples = (sr * duration) as usize;
    for i in 0..samples {
        let t = i as f32 / sr;
        wave.push((freq * 2.0 * PI * t).sin());
    }
    wave
}
