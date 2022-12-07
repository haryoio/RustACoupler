#[derive(Clone, Debug)]
pub struct ModemConfig {
    pub samplerate: u32,
    pub baudrate:   u16,
    pub carrier:    f32,
    pub deviation:  f32,
    pub threshold:  u32,
    pub amplitude:  f32,
    pub channels:   u8,
}

enum CarrierFreq {
    FreqMin4650Car4800Max5250,
    FreqMin4050Car4200Max4650,
}

impl Default for ModemConfig {
    fn default() -> Self {
        Self {
            samplerate: 44100,
            baudrate:   100,
            carrier:    4800f32,
            deviation:  300f32,
            threshold:  150,
            amplitude:  i16::MAX as f32,
            channels:   1,
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
}
