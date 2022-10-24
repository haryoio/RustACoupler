use hound;
use std::f32::consts::PI;
use std::i16;

fn main() {
    let sr = 44100;
    let dur = 5;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let mut count = 1;
    for t in (0..(dur * sr)).map(|i| i as f32 / sr as f32) {
        let freq = if count == 1 {
            count = 0;
            1200f32
        } else {
            count = 1;
            2400f32
        };

        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
