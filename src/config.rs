use crate::{binary, hamming, receiver::Receiver, transmitter::Transmitter, utils};

#[derive(Clone, Debug)]
pub enum Role {
    Transmitter,
    Receiver,
}

#[derive(Clone, Debug)]
pub struct ModemConfig {
    pub samplerate: f32,
    pub baudrate: u16,
    pub latency: f64,
    pub low_freq: f32,
    pub high_freq: f32,
    pub threshold: f32,
    pub amplitude: f32,
    pub channels: i32,
    pub role: Role,
}

impl ModemConfig {
    pub fn get_input_rate(&self) -> ModemConfig {
        ModemConfig {
            samplerate: self.samplerate * 2f32,
            baudrate: self.baudrate,
            latency: (1.0 / self.baudrate as f64 * self.samplerate as f64 * 2f64),
            low_freq: self.low_freq,
            high_freq: self.high_freq,
            threshold: self.threshold,
            amplitude: self.amplitude,
            channels: self.channels,
            role: Role::Transmitter,
        }
    }
}

impl Default for ModemConfig {
    fn default() -> Self {
        let mut conf = Self {
            samplerate: 44100f32,
            baudrate: 300,
            latency: Default::default(),
            low_freq: 1200f32,
            high_freq: 2400f32,
            threshold: 300f32,
            amplitude: i16::MAX as f32,
            channels: 1,
            role: Role::Receiver,
        };
        conf.latency = 1.0 / conf.baudrate as f64 * conf.samplerate as f64;
        conf
    }
}
