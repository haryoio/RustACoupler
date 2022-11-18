use std::sync::mpsc::channel;

use crate::config::ModemConfig;

use rustfft::algorithm::Dft;
use rustfft::num_complex::Complex;
use rustfft::Fft;
use rustfft::FftDirection;

use portaudio as pa;
#[derive(Clone)]
pub struct Receiver {
    pub(crate) config: ModemConfig,
}

impl Receiver {
    pub fn new(config: ModemConfig) -> Receiver {
        return Receiver { config };
    }

    pub fn run(&mut self) -> Result<(), pa::Error> {
        let chunk = (self.config.latency) as u32;
        let threshold = 300;

        // オーディオ初期化
        let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio");
        // マイク取得
        let mic_index = pa
            .default_input_device()
            .expect("Unable to get default device");

        let input_params =
            portaudio::StreamParameters::<f32>::new(mic_index, 1, true, self.config.latency);
        let input_settings =
            portaudio::InputStreamSettings::new(input_params, self.config.samplerate as f64, chunk);

        let (sender, receiver) = channel();

        let callback =
            move |portaudio::InputStreamCallbackArgs { buffer, .. }| match sender.send(buffer) {
                Ok(_) => portaudio::Continue,
                Err(_) => portaudio::Complete,
            };

        let mut stream = pa
            .open_non_blocking_stream(input_settings, callback)
            .expect("Unable to create stream");
        stream.start().expect("Unable to start stream");

        while stream.is_active().unwrap() {
            let mut res = vec![];
            while let Ok(buffer) = receiver.try_recv() {
                let mut data: Vec<Complex<f32>> = to_c32(buffer);

                let dft = Dft::new(self.config.latency as usize, FftDirection::Inverse);
                dft.process(&mut data);
                let (max_index, max) =
                    data.iter()
                        .enumerate()
                        .fold((usize::MIN, f32::MIN), |(i_a, a), (i_b, &b)| {
                            if b.norm() > a {
                                (i_b, b.norm())
                            } else {
                                (i_a, a)
                            }
                        });

                let resfreq = max_index * 100;
                if resfreq >= (self.config.low_freq as usize - threshold)
                    && resfreq <= self.config.low_freq as usize + threshold
                {
                    res.push(0);
                } else if resfreq >= (self.config.high_freq as usize - threshold)
                    && resfreq <= self.config.high_freq as usize + threshold
                {
                    res.push(1);
                } else {
                    res.push(-1);
                }
                if res.len() == 8 {
                    println!("{:?}", res);
                    res.clear();
                }
            }
        }
        Ok(())
    }
}

fn to_c32(slice: &[f32]) -> Vec<Complex<f32>> {
    slice.iter().map(|&re| Complex::new(re, 0.0)).collect()
}
