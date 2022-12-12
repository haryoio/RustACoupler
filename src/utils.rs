pub fn repeat(data: Vec<f32>, take: usize) -> Vec<f32> {
    let mut result = vec![];
    for one_of_data in data.iter() {
        for _ in 0..take {
            result.push(*one_of_data);
        }
    }
    result
}

pub fn ocillator(samplerate: u32, freq: f32) -> impl Iterator<Item = f32> {
    let mut t = 0.0;
    std::iter::from_fn(move || {
        t += 1.0 / samplerate as f32;
        Some((2.0 * std::f32::consts::PI * freq * t).sin() * i16::MAX as f32)
    })
}

pub fn save_wave(filename: &str, data: Vec<f32>, sr: u32) {
    let spec = hound::WavSpec {
        channels:        1,
        sample_rate:     sr,
        bits_per_sample: 32,
        sample_format:   hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for i in data.iter() {
        writer.write_sample(0.5 * (*i / i16::MAX as f32)).unwrap();
    }
}

pub fn read_wave(filename: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let mut data = Vec::new();
    for s in reader.samples::<f32>() {
        data.push(s.unwrap());
    }
    data
}
