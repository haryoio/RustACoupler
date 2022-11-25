use portaudio::stream::Mode;

use crate::{
    ascii,
    hamming,
    receiver::Receiver,
    transmitter::Transmitter,
    utils,
    ModulationMethod,
};

#[derive(Clone, Debug)]
pub enum Role {
    Transmitter,
    Receiver,
}

#[derive(Clone, Debug)]
pub struct ModemConfig {
    pub samplerate:        f32,
    pub baudrate:          u16,
    pub low_freq:          f32,
    pub high_freq:         f32,
    pub threshold:         f32,
    pub amplitude:         f32,
    pub channels:          i32,
    pub role:              Role,
    pub modulation_method: ModulationMethod,
}

impl Default for ModemConfig {
    fn default() -> Self {
        Self {
            samplerate:        44100f32,
            baudrate:          100,
            low_freq:          1200f32,
            high_freq:         2400f32,
            threshold:         50f32,
            amplitude:         i16::MAX as f32,
            channels:          1,
            role:              Role::Receiver,
            modulation_method: ModulationMethod::BFSK,
        }
    }
}

impl ModemConfig {
    pub fn latency(&self) -> f32 {
        (1.0 / self.baudrate as f32 * self.samplerate as f32).floor()
    }
}
