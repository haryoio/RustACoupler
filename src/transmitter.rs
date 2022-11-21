use binary::encode_u8;

use config::ModemConfig;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use portaudio as pa;
use std::{
    f32::consts::PI,
    sync::{mpsc, Arc, Mutex},
};
use utils::repeat;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use cpal::{Data, Sample, SampleFormat};
use ringbuf::HeapRb;

use crate::{
    binary, config,
    hamming::{self, Hamming::get_hamming_code},
    utils, PREAMBLE, SFD,
};

#[derive(Clone)]
pub struct Transmitter {
    pub(crate) config: ModemConfig,
}

impl Transmitter {
    pub fn new(config: ModemConfig) -> Transmitter {
        return Transmitter { config };
    }

    pub fn encode(&mut self, data: &str) -> Vec<u8> {
        let bin = encode_u8(data);
        let bin = self.clone().add_syn(bin.clone());
        let bin = get_hamming_code(bin);
        bin
    }

    fn add_syn(self, bin: Vec<u8>) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(PREAMBLE);
        buf.extend(SFD);
        buf.extend(bin.clone());
        buf.extend(PREAMBLE);
        buf
    }

    fn one_or_zero(&mut self, data: &u8) -> f32 {
        if data == &0 {
            self.config.low_freq
        } else {
            self.config.high_freq
        }
    }

    // バイナリデータを変調する
    pub fn modulation(&mut self, bin: Vec<u8>) -> Vec<f32> {
        let mut buf = vec![];
        for b in bin {
            buf.push(self.one_or_zero(&b));
        }
        let symbol_freqs = repeat(buf, self.config.latency as usize);
        let delta_phi: Vec<f32> = symbol_freqs
            .iter()
            .map(|d| (d * PI / (self.config.samplerate / 2.0)))
            .cumsum()
            .collect_vec();

        delta_phi
            .iter()
            .map(|d| d.sin() * self.config.amplitude)
            .collect::<Vec<_>>()
    }

    pub fn send(&mut self, in_data: &str) {
        let in_data = self.encode(in_data);
        let in_data = self.modulation(in_data);
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = StreamConfig {
            channels: self.config.channels as u16,
            sample_rate: cpal::SampleRate((self.config.samplerate as usize) as u32),
            buffer_size: cpal::BufferSize::Default,
        };
        let ring = HeapRb::<f32>::new(in_data.len());
        let (mut producer, mut consumer) = ring.split();
        let (tx, rx) = mpsc::channel::<bool>();

        for sample in in_data {
            producer.push(sample).unwrap();
        }

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data {
                        *sample = match consumer.pop() {
                            Some(t) => 0.1 * (t as f32 / 32767.0),
                            None => {
                                tx.send(false).unwrap();
                                0.0
                            }
                        }
                    }
                },
                err_fn,
            )
            .unwrap();
        stream.play().unwrap();

        while let Ok(msg) = rx.recv() {
            if !msg {
                stream.pause().unwrap();
                break;
            }
        }
    }
}
