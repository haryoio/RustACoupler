use std::{
    mem::MaybeUninit,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use cpal::{
    platform::Stream as PlatformStream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use ringbuf::{HeapRb, Producer, SharedRb};

pub struct Speaker {
    samplerate: u32,
    latency:    u32,
    channels:   u16,
    device:     Option<String>,
}

type StreamProducer<T> = Producer<T, Arc<SharedRb<T, Vec<MaybeUninit<T>>>>>;

#[cfg(target_os = "linux")]
pub struct SpeakerStream {
    pub producer: StreamProducer<f32>,
    pub stream:   Arc<Mutex<PlatformStream>>,
}
#[cfg(target_os = "macos")]
pub struct SpeakerStream {
    pub producer: StreamProducer<f32>,
    pub stream:   Arc<Mutex<PlatformStream>>,
}

impl Speaker {
    pub fn new(samplerate: u32, latency: u32, channels: u16) -> Self {
        Speaker {
            samplerate,
            latency,
            channels,
            device: None,
        }
    }

    pub fn set_device(&mut self, device: &str) {
        self.device = Some(device.to_string());
    }

    pub fn play(&mut self, wave: Vec<f32>) {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .expect("failed to find output device");

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let stream_config = StreamConfig {
            channels:    self.channels,
            sample_rate: cpal::SampleRate(self.samplerate),
            buffer_size: cpal::BufferSize::Fixed(self.latency),
        };

        let out_ring =
            HeapRb::<Vec<f32>>::new((self.samplerate * self.latency).try_into().unwrap());
        let (mut out_producer, mut out_consumer) = out_ring.split();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if let Some(samples) = out_consumer.pop() {
                for (sample, wav) in data.iter_mut().zip(samples) {
                    *sample = 0.5 * (wav / i16::MAX as f32);
                }
            } else {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
            }
        };

        let stream = output_device
            .build_output_stream(&stream_config, output_data_fn, err_fn)
            .expect("failed to build output stream");
        stream.play().unwrap();

        let time_to_wait = &(1.0 / self.samplerate as f64);

        loop {
            for samples in wave.chunks(self.latency as usize) {
                if let Err(e) = out_producer.push(samples.to_vec()) {
                    println!("Error: {:?}", e);
                }
                sleep(Duration::from_secs_f64(self.latency as f64 * time_to_wait));
            }
            sleep(Duration::from_secs_f64(self.latency as f64 * time_to_wait));
            break;
        }
    }
}

#[cfg(test)]
mod speaker_tests {
    use super::*;
    use crate::synthesizer::ocillator;

    #[test]
    fn test_speaker() {
        let mut speaker = Speaker::new(44100, 1024, 1);
        let mut ocillator440 = ocillator(44100, 440.0);
        let mut ocillator220 = ocillator(44100, 220.0);

        let mut waves = vec![];
        for _ in 0..100 {
            for _ in 0..44100 {
                waves.push(ocillator440.next().unwrap());
            }
            for _ in 0..44100 {
                waves.push(ocillator220.next().unwrap());
            }
        }
        println!("{}", waves.len());

        speaker.play(waves);
    }
}
