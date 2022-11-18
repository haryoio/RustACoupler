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

impl Default for ModemConfig {
    fn default() -> Self {
        let mut conf = Self {
            samplerate: 44100f32,
            baudrate: 100,
            latency: Default::default(),
            low_freq: 220f32,
            high_freq: 440f32,
            threshold: 300f32,
            amplitude: i16::MAX as f32,
            channels: 1,
            role: Role::Receiver,
        };
        conf.latency = 1.0 / conf.baudrate as f64 * conf.samplerate as f64;
        conf
    }
}
