use std::f32::consts::PI;

use ascii::encode_u8;
use config::ModemConfig;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use portaudio as pa;
use utils::repeat;

use crate::{ascii, config, hamming::Hamming::get_hamming_code, utils, USFD};

#[derive(Clone)]
pub struct Transmitter {
    pub(crate) config: ModemConfig,
}

impl Transmitter {
    pub fn new(config: ModemConfig) -> Transmitter {
        let mut config = config;
        // config.samplerate = config.samplerate / 2f32;
        return Transmitter { config };
    }

    fn encode(&self, data: &str) -> Vec<u8> {
        let bin = encode_u8(data);
        let bin = get_hamming_code(bin);
        let bin = self.add_syn(bin.clone());
        bin
    }

    pub fn qfsk(&mut self, data: &str) -> Vec<f32> {
        let bin = self.encode(data);
        let buf = self.set_tone_qfsk(&bin);
        let symbol_freqs = repeat(buf, self.config.latency() as usize);
        symbol_freqs
            .iter()
            .map(|d| (d * PI / (self.config.samplerate / 2.0)))
            .cumsum()
            .collect::<Vec<f32>>()
            .iter()
            .map(|d| d.sin() * self.config.amplitude)
            .collect::<Vec<f32>>()
    }

    pub fn bfsk(&mut self, data: &str) -> Vec<f32> {
        let bin = self.encode(data);
        let buf = self.set_tone_bfsk(&bin);

        let symbol_freqs = repeat(buf, self.config.latency() as usize);
        symbol_freqs
            .iter()
            .map(|d| (d * PI / (self.config.samplerate / 2.0)))
            .cumsum()
            .collect::<Vec<f32>>()
            .iter()
            .map(|d| d.sin() * self.config.amplitude)
            .collect::<Vec<f32>>()
    }

    fn add_syn(&self, bin: Vec<u8>) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend(USFD);

        buf.extend(bin.clone());

        buf.extend(USFD);
        buf
    }

    fn set_tone_bfsk(&mut self, data: &Vec<u8>) -> Vec<f32> {
        let d = data.clone();
        let mut res = vec![];
        for b in d {
            match b {
                0 => res.push(self.config.low_freq),
                1 => res.push(self.config.high_freq),
                _ => res.push(0.0),
            }
        }
        res
    }

    fn set_tone_qfsk(&mut self, data: &Vec<u8>) -> Vec<f32> {
        let mut d = data.clone();
        let mut res = vec![];
        while !d.is_empty() {
            let mut two_bit = vec![];
            two_bit.push(d.pop().unwrap());
            two_bit.push(d.pop().unwrap());
            match *two_bit {
                [0, 0] => res.push(self.config.low_freq),
                [0, 1] => res.push(self.config.low_freq * 1.5),
                [1, 0] => res.push(self.config.low_freq * 2.0),
                [1, 1] => res.push(self.config.low_freq * 2.5),
                _ => res.push(0.0),
            }
        }
        res
    }

    pub fn play(&self, data: &[f32]) {
        let pa = pa::PortAudio::new().unwrap();
        let device = pa.default_output_device().unwrap();
        let latency: f64 = 1.0 / self.config.samplerate as f64;

        let output_params =
            pa::StreamParameters::<f32>::new(device, self.config.channels, true, latency);

        pa.is_output_format_supported(output_params, self.config.samplerate.into())
            .unwrap();
        let settings = pa::OutputStreamSettings::new(
            output_params,
            self.config.samplerate.into(),
            self.config.latency().ceil() as u32,
        );

        let mut stream = pa.open_blocking_stream(settings).unwrap();

        let mut wav_buffer_iter = data.iter();
        let mut len = data.len();
        stream.start().unwrap();
        loop {
            let n_write_samples = self.config.latency() as usize;

            let res = stream.write(self.config.latency() as u32, |output| {
                for i in 0..n_write_samples {
                    match wav_buffer_iter.next() {
                        Some(t) => {
                            len -= 1;
                            output[i] = 0.5 * (*t as f32 / i16::MAX as f32);
                        }
                        None => len = 0,
                    };
                }
            });
            match res {
                Ok(_) => {
                    if len == 0 {
                        break;
                    }
                }
                Err(e) => break,
            }
        }
    }
}
