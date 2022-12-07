use std::{
    mem::MaybeUninit,
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use cpal::{
    platform::Stream as PlatformStream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use ringbuf::{HeapRb, Producer, SharedRb};
use tokio::sync::Mutex;

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
        // self.cb = cb;
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

        let out_ring = HeapRb::<f32>::new((self.samplerate * self.latency).try_into().unwrap());
        let (mut out_producer, mut out_consumer) = out_ring.split();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // println!("o");
            for sample in data.iter_mut() {
                if let Some(t) = out_consumer.pop() {
                    // println!("send");
                    *sample = 0.5 * (t / i16::MAX as f32);
                } else {
                    *sample = 0.0;
                }
            }
        };

        let stream = output_device
            .build_output_stream(&stream_config, output_data_fn, err_fn)
            .expect("failed to build output stream");
        stream.play().unwrap();

        let start = Instant::now();

        let time_to_wait = &(1.0 / self.samplerate as f64);

        while start
            .elapsed()
            .as_secs_f64()
            .lt(&(time_to_wait * wave.len() as f64))
        {
            for w in &wave {
                if let Err(e) = out_producer.push(*w) {
                    println!("error: {}", e);
                }
            }
            sleep(Duration::from_millis(1));
        }

        // loop {
        //     counter += 1;
        //     if counter >= waves_len {
        //         println!("done send");
        //         break;
        //     }
        // }

        // stream.pause().unwrap();
        // drop(stream);
    }

    // pub fn pause(&mut self) {
    //     self.running
    //         .blocking_write()
    //         .store(false, Ordering::Relaxed);
    // }
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
