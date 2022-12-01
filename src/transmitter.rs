use std::{
    cell::RefCell,
    f32::consts::PI,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ascii::encode_u8;
use config::ModemConfig;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use itertools_num::ItertoolsNum;
use ringbuf::HeapRb;
use utils::repeat;

use crate::{ascii, config, hamming::Hamming::get_hamming_code, utils, ModulationMethod, USFD};

#[derive(Clone)]
pub struct Transmitter {
    pub(crate) config: ModemConfig,
}

impl Transmitter {
    pub fn new(config: ModemConfig) -> Transmitter {
        return Transmitter { config };
    }

    fn encode(&self, data: &str) -> Vec<u8> {
        let bin = encode_u8(data);
        let bin = get_hamming_code(bin);
        let bin = self.add_syn(bin.clone());
        bin
    }

    pub fn fsk(&mut self, data: &str) -> Vec<f32> {
        let bin = self.encode(data);

        let buf = match self.config.modulation_method {
            ModulationMethod::QFSK => self.set_tone_qfsk(&bin),
            ModulationMethod::BFSK => self.set_tone_bfsk(&bin),
        };

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
        let d = data.clone();
        let mut res = vec![];
        for a in d.chunks(2) {
            let b = a[0];
            let c = a[1];
            match (b, c) {
                (0, 0) => res.push(self.config.low_freq),
                (0, 1) => res.push(self.config.low_freq + 400f32),
                (1, 0) => res.push(self.config.low_freq + 1200f32),
                (1, 1) => res.push(self.config.low_freq + 2400f32),
                _ => res.push(0.0),
            }
        }
        res
    }

    pub fn play(&self, data: &[f32]) {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .expect("failed to find output device");

        // println!("output device: {:?}", output_device.name());

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream_config = StreamConfig {
            channels:    1,
            sample_rate: cpal::SampleRate(self.config.samplerate as u32),
            buffer_size: cpal::BufferSize::Fixed(self.config.latency() as u32),
        };

        let out_ring = HeapRb::<f32>::new(data.len() * 2);
        let (mut out_producer, mut out_consumer) = out_ring.split();

        for i in 0..data.len() {
            out_producer.push(data[i]).expect("");
        }

        let status = Arc::new(Mutex::new(false));

        let in_status = Arc::clone(&status);
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                if let Some(t) = out_consumer.pop() {
                    *sample = 0.1 * (t / i16::MAX as f32);
                } else {
                    *sample = 0.0;
                    *in_status.lock().expect("cant lock mutex") = true;
                }
            }
        };

        let output_stream = output_device
            .build_output_stream(&stream_config, output_data_fn, err_fn)
            .expect("failed to build output stream");

        output_stream.play().expect("cannot start output stream");
        loop {
            if *status.lock().expect("cant lock mutex") {
                break;
            }
        }
    }
}
