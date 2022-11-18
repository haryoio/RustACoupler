use binary::encode_u8;

use config::ModemConfig;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use portaudio as pa;
use std::f32::consts::PI;
use utils::repeat;

use crate::{
    binary, config,
    hamming::{self, Hamming::get_hamming_code},
    utils,
};

#[derive(Clone)]
pub struct Transmitter {
    pub(crate) config: ModemConfig,
}

impl Transmitter {
    pub fn new(config: ModemConfig) -> Transmitter {
        return Transmitter { config };
    }

    pub fn modulation(&mut self, data: &str) -> Vec<f32> {
        let bin = encode_u8(data);
        let bin = self.clone().add_syn(bin.clone());
        let bin = get_hamming_code(bin);

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

    fn add_syn(self, bin: Vec<u8>) -> Vec<u8> {
        let mut buf = vec![];
        let syn: Vec<u8> = [0, 0, 0, 1, 0, 1, 1, 0].to_vec();
        buf.extend(syn.clone());
        buf.extend(syn.clone());
        buf.extend(bin.clone());
        buf.extend(syn.clone());
        buf
    }

    fn one_or_zero(&mut self, data: &u8) -> f32 {
        if data == &0 {
            self.config.low_freq
        } else {
            self.config.high_freq
        }
    }

    pub fn play(&self, data: &[f32]) {
        let pa = pa::PortAudio::new().unwrap();
        let device = pa.default_output_device().unwrap();
        let output_info = pa.device_info(device).unwrap();
        let latency = output_info.default_low_output_latency;

        let output_params =
            pa::StreamParameters::<f32>::new(device, self.config.channels, true, latency);

        pa.is_output_format_supported(output_params, self.config.samplerate.into())
            .unwrap();
        let settings =
            pa::OutputStreamSettings::new(output_params, self.config.samplerate.into(), 1024);

        let mut stream = pa.open_blocking_stream(settings).unwrap();

        let mut wav_buffer_iter = data.iter();
        let mut len = data.len();
        stream.start().unwrap();
        loop {
            let n_write_samples = 1024 as usize * self.config.channels as usize;

            let res = stream.write(1024 as u32, |output| {
                for i in 0..n_write_samples {
                    match wav_buffer_iter.next() {
                        Some(t) => {
                            len -= 1;
                            output[i] = 0.1 * (*t as f32 / 32767.0);
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
