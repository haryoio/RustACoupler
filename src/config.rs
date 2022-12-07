use crate::ModulationMethod;

#[derive(Clone, Debug)]
pub enum Role {
    Transmitter,
    Receiver,
}

#[derive(Clone, Debug)]
pub struct ModemConfig {
    pub samplerate:        u32,
    pub baudrate:          u16,
    pub carrier:           f32,
    pub deviation:         f32,
    pub threshold:         u32,
    pub amplitude:         f32,
    pub channels:          u8,
    pub role:              Role,
    pub modulation_method: ModulationMethod,
}

impl Default for ModemConfig {
    fn default() -> Self {
        Self {
            samplerate:        44100,
            baudrate:          100,
            carrier:           1200f32,
            deviation:         2400f32,
            threshold:         200,
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

    pub fn set_samplerate(&mut self, samplerate: u32) {
        self.samplerate = samplerate;
    }
    pub fn set_baudrate(&mut self, baudrate: u16) {
        self.baudrate = baudrate;
    }
    pub fn set_carrior(&mut self, carrier: f32) {
        self.carrier = carrier;
    }
    pub fn set_deviation(&mut self, deviation: f32) {
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
    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }
    pub fn set_modulation_method(&mut self, modulation_method: ModulationMethod) {
        self.modulation_method = modulation_method;
    }
}
