use crate::{args::Bands, modem::modulator::ModulationFormat};

#[derive(Debug, Clone)]
pub struct ModemConfig {
    pub samplerate:        u32,
    pub baudrate:          u16,
    pub carrier:           u32,
    pub deviation:         u32,
    pub threshold:         u32,
    pub amplitude:         f32,
    pub channels:          u8,
    pub modulation_format: ModulationFormat,
    pub input_device:      Option<String>,
    pub output_device:     Option<String>,
}

impl Default for ModemConfig {
    fn default() -> Self {
        Self {
            samplerate:        44100,
            baudrate:          100,
            carrier:           4800,
            deviation:         300,
            threshold:         150,
            amplitude:         i16::MAX as f32,
            channels:          1,
            modulation_format: ModulationFormat::BFSK,
            input_device:      None,
            output_device:     None,
        }
    }
}

impl ModemConfig {
    pub fn latency(&self) -> f32 {
        (1.0 / self.baudrate as f32 * self.samplerate as f32).floor()
    }

    pub fn set_samplerate(&mut self, samplerate: u32) {
        self.samplerate = samplerate;
    }
    pub fn set_baudrate(&mut self, baudrate: u16) {
        self.baudrate = baudrate;
    }
    pub fn set_carrior(&mut self, carrier: u32) {
        self.carrier = carrier;
    }
    pub fn set_deviation(&mut self, deviation: u32) {
        self.deviation = deviation;
    }
    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
    }
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
    pub fn set_channels(&mut self, channels: u8) {
        self.channels = channels;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Band {
    pub carrier:   u32,
    pub deviation: u32,
    pub threshold: u32,
}
impl Band {
    /// 新規バンドを作成
    pub fn new(carrier: u32, deviation: u32, threshold: u32) -> Self {
        Self {
            carrier,
            deviation,
            threshold,
        }
    }
    pub fn carrier(&self) -> u32 {
        self.carrier
    }
    pub fn deviation(&self) -> u32 {
        self.deviation
    }
    pub fn threshold(&self) -> u32 {
        self.threshold
    }
}
pub static BAND1: Band = Band {
    carrier:   3300,
    deviation: 600,
    threshold: 300,
};
pub static BAND2: Band = Band {
    carrier:   4500,
    deviation: 600,
    threshold: 300,
};
pub static BAND3: Band = Band {
    carrier:   5700,
    deviation: 600,
    threshold: 300,
};

impl From<Bands> for Band {
    fn from(band: Bands) -> Self {
        match band {
            Bands::Band1 => BAND1,
            Bands::Band2 => BAND2,
            Bands::Band3 => BAND3,
        }
    }
}

impl Band {
    pub fn has_freq(&self, freq: f32) -> bool {
        let carrier = self.carrier as f32;
        let deviation = self.deviation as f32;
        let threshold = self.threshold as f32;
        let lower = carrier - threshold;
        let upper = carrier + deviation + threshold;
        freq >= lower && freq <= upper
    }
}
