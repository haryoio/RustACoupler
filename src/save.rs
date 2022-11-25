pub fn save_wav(filename: &str, data: Vec<f32>, sr: u32) {
    let spec = hound::WavSpec {
        channels:        1,
        sample_rate:     sr as u32,
        bits_per_sample: 32,
        sample_format:   hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for i in data.iter() {
        writer.write_sample(0.5 * (*i / i16::MAX as f32)).unwrap();
    }
}
