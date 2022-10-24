pub mod binary;
pub mod config;
pub mod hamming;
pub mod utils;

use binary::encode_u8;
use config::{AMP, ONE_FREQ, ZERO_FREQ};
use hound::{self, WavWriter};
use iter_num_tools::arange;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use secded::{SecDed64, SecDedCodec, SecDedDynamic};
use std::{f32::consts::PI, fs::File};
use std::{i16, io::BufWriter};
use utils::repeat;

struct AcousticCouplerConfig {
    sample_rate: f32,
    baud_rate: u16,
    period: f32,
    zero_freq: f32,
    one_freq: f32,
    amplitude: f32,
    filename: String,
}

impl Default for AcousticCouplerConfig {
    fn default() -> Self {
        let mut conf = Self {
            sample_rate: 44100f32,
            baud_rate: 100,
            period: Default::default(),
            zero_freq: 220f32,
            one_freq: 440f32,
            amplitude: i16::MAX as f32,
            filename: "data.wav".to_string(),
        };
        conf.period = 1.0 / conf.baud_rate as f32 * conf.sample_rate;
        conf
    }
}

struct ACCBuilder {
    config: AcousticCouplerConfig,
}

impl ACCBuilder {
    pub fn new() -> Self {
        Self {
            config: AcousticCouplerConfig::default(),
        }
    }
    pub fn sample_rate(mut self, rate: f32) -> ACCBuilder {
        self.config.sample_rate = rate;
        self
    }
    pub fn baud_rate(mut self, rate: u16) -> ACCBuilder {
        self.config.baud_rate = rate;
        self
    }
    pub fn freqs(mut self, zero: f32, one: f32) -> ACCBuilder {
        self.config.zero_freq = zero;
        self.config.one_freq = one;
        self
    }
    pub fn amplitude(mut self, amp: f32) -> ACCBuilder {
        self.config.amplitude = amp;
        self
    }
    pub fn filename(mut self, name: String) -> ACCBuilder {
        self.config.filename = format!("{}.wav", name);
        self
    }
    pub fn build(mut self) -> AcousticCoupler {
        self.config.period = 1.0 / self.config.baud_rate as f32 * self.config.sample_rate;
        return AcousticCoupler {
            config: self.config,
        };
    }
}

struct AcousticCoupler {
    config: AcousticCouplerConfig,
}

impl AcousticCoupler {
    pub fn builder() -> ACCBuilder {
        return ACCBuilder::new();
    }

    pub fn send(&mut self, data: &str) {
        let bin = encode_u8(data);

        let mut buf = vec![];
        for b in bin {
            buf.push(self.one_or_zero(&b));
        }
        let symbol_freqs = repeat(buf, self.config.period as usize);
        let delta_phi: Vec<f32> = symbol_freqs
            .iter()
            .map(|d| (d * PI / (self.config.sample_rate / 2.0)))
            .cumsum()
            .collect_vec();

        let signal = delta_phi.iter().map(|d| d.sin() * self.config.amplitude);

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.config.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&self.config.filename, spec).unwrap();

        for sample in signal {
            writer.write_sample(sample as i16).unwrap();
        }
    }

    fn add_syn(bin: &[u8]) {}

    fn one_or_zero(&mut self, data: &u8) -> f32 {
        if data == &0 {
            self.config.zero_freq
        } else {
            self.config.one_freq
        }
    }
}

fn main() {
    let mut ac = AcousticCoupler::builder()
        .sample_rate(44100.0)
        .baud_rate(300)
        .amplitude(i16::MAX as f32)
        .freqs(3000f32, 6000f32)
        .filename("sin".to_string())
        .build();

    // ac.send("connitiha-asdfasdfaasdfasdfasdfadfadfadfadfadfadf");
}
