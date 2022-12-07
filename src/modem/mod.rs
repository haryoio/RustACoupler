pub mod demodulator;
pub mod filter;
pub mod modulator;
pub mod protocol;
use std::{collections::VecDeque, sync::mpsc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use rand::Rng;

use self::{demodulator::detect_bfsk, protocol::Protocol};
use crate::{
    bytes::{decode_u8, encode_u8},
    config::ModemConfig,
    datalink::frame::{Datalink, FrameType},
    modem::filter::fftfreq,
    physical::frame::Physical,
    speaker::Speaker,
    Status,
    ISFD,
};

pub trait ModemTrait {
    fn new(samplerate: usize, baudrate: usize, channels: u16) -> Self;
    fn modulate(&self, data: &str) -> Vec<f32>;
    fn demodulate(&self, data: &[f32]) -> String;
    fn transmit(&self, samples: &[f32]);
    fn receive(&self, samples: &[f32]) -> String;
}
pub struct Modem {
    samplerate: u32,
    baudrate:   u16,
    channels:   u8,
    carrier:    f32,
    deviation:  f32,
    threshold:  f32,

    address: u8,
    // connections: Arc<RwLock<Vec<>>>
}

impl Modem {
    pub fn new(config: ModemConfig) -> Self {
        let carrier = config.carrier;
        let deviation = config.deviation;
        let samplerate = config.samplerate;
        let baudrate = config.baudrate;
        let channels = config.channels;
        let threshold = config.threshold as f32;
        let mut rng = rand::thread_rng();
        let address: u8 = rng.gen_range(0..254);

        Modem {
            samplerate,
            baudrate,
            channels,
            carrier,
            deviation,
            threshold,

            address,
        }
    }

    pub fn transmit(&self, symbols: Vec<u8>) {
        let carrier = self.carrier;
        let deviation = self.deviation;
        let samplerate = self.samplerate;
        let baudrate = self.baudrate;
        let channels = self.channels;
        let latency = 1.0 / baudrate as f32 * samplerate as f32;

        let samples = modulator::bfsk(
            &symbols,
            carrier,
            deviation,
            (samplerate as usize).try_into().unwrap(),
            latency as usize,
        );
        let mut speaker = Speaker::new(samplerate, baudrate as u32, channels as u16);

        speaker.play(samples);
    }

    pub fn record(&mut self, connection_tx: mpsc::Sender<Datalink>) {
        let carrier = self.carrier;
        let deviation = self.deviation;
        let samplerate = self.samplerate;
        let baudrate = self.baudrate;
        let channels = self.channels;
        let threshold = self.threshold;

        let latency = 1.0 / baudrate as f32 * samplerate as f32;

        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .expect("no input device available");

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream_config = StreamConfig {
            channels:    channels as u16,
            sample_rate: cpal::SampleRate(samplerate),
            buffer_size: cpal::BufferSize::Fixed(latency as u32),
        };

        let (producer, consumer) = mpsc::channel();

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if let Err(e) = producer.send(data.to_vec()) {
                eprintln!("error: {}", e);
            }
        };
        let stream = input_device
            .build_input_stream(&stream_config, input_data_fn, err_fn)
            .expect("failed to build input stream");
        // 録音開始

        stream.play().expect("failed to play stream");
        // println!("recording...");

        let mut recent_bin: VecDeque<i8> = vec![-1; 8].into();
        let mut input_data: VecDeque<i8> = vec![].into();

        let mut frame_length = 0;
        let mut phy_frame: Option<Physical> = None;
        let mut status = Status::LISTENING;

        // println!("start demodulatoin");
        loop {
            if let Ok(sample) = consumer.recv() {
                let resfreq = fftfreq(sample, self.samplerate as f32).unwrap();

                if resfreq == 10900.0 {
                    continue;
                }

                let bit = detect_bfsk(resfreq, carrier, deviation, threshold);
                recent_bin.push_back(bit);
                recent_bin.pop_front();

                let mut bin = [0i8; 8];
                for (i, b) in recent_bin.iter().enumerate() {
                    if i >= 8 {
                        break;
                    }
                    bin[i] = *b;
                }

                match status {
                    Status::LISTENING => {
                        if bin == ISFD {
                            status = Status::RECEIVING;
                        }
                    }
                    Status::RECEIVING => {
                        input_data.push_back(bit);

                        if input_data.len() == 36 && frame_length == 0 && phy_frame.is_none() {
                            let mut frame_arr = [0i8; 36];
                            for (i, b) in input_data.iter().enumerate() {
                                if i >= 36 {
                                    break;
                                }
                                frame_arr[i] = *b;
                            }
                            let frame_arr = frame_arr.map(|b| b as u8);
                            let frame = Physical::from_bytes(&frame_arr).unwrap();
                            // println!("frame: {:?}", frame);
                            frame_length = frame.length as usize;
                            phy_frame = Some(frame);
                            input_data.clear();
                        }
                        if frame_length > 0 {
                            frame_length = frame_length.saturating_sub(1);
                        }
                        if frame_length == 0 && phy_frame.is_some() {
                            status = Status::ANSWER;
                            // println!("frame received");
                        }
                    }
                    Status::ANSWER => {
                        input_data.push_back(bit);
                        let mut frame = vec![];
                        for b in input_data.iter() {
                            frame.push(*b as u8);
                        }
                        let proto = match Datalink::from_bytes(&frame) {
                            Ok(p) => p,
                            Err(e) => {
                                println!("error: {:?}", e);
                                continue;
                            }
                        };
                        if proto.detect_checksum() {
                            // println!("checksum ok");
                        } else {
                            println!("checksum error");
                        }
                        if proto.frame_type.is_acknowledgement() {
                            println!("acknowledgement");
                        } else if proto.frame_type.is_data() {
                            connection_tx.send(proto.clone()).unwrap();
                            let data = decode_u8(proto.data);
                            println!("<< {}", data);
                        }
                        phy_frame = None;
                        input_data.clear();
                        frame_length = 0;
                        status = Status::LISTENING;
                    }
                    Status::TRANSMITTING => {}
                }
            }
        }
    }
}
