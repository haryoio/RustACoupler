pub mod demodulator;
pub mod filter;
pub mod modulator;
pub mod protocol;
use std::{collections::VecDeque, sync::mpsc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
    StreamConfig,
};
use rand::Rng;

use self::{
    demodulator::{detect_bfsk, which_band},
    filter::{highpass_filter, lowpass_filter},
    modulator::ModulationFormat,
};
use crate::{
    bytes::decode_u8,
    config::{Band, ModemConfig, BAND1, BAND2, BAND3},
    datalink::frame::Datalink,
    modem::filter::fftfreq,
    physical::frame::Physical,
    speaker::Speaker,
    utils::save_wave,
    Status,
    ISFD,
};
#[derive(Debug, Clone)]
pub struct Modem {
    samplerate:        u32,
    baudrate:          u16,
    channels:          u8,
    carrier:           f32,
    deviation:         f32,
    threshold:         f32,
    modulation_format: ModulationFormat,

    address:       u8,
    input_device:  Option<String>,
    output_device: Option<String>,
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
        let modulation_format = config.modulation_format;
        let input_device = config.input_device;
        let output_device = config.output_device;

        Modem {
            samplerate,
            baudrate,
            channels,
            carrier: carrier as f32,
            deviation: deviation as f32,
            threshold,

            address,
            modulation_format,

            input_device,
            output_device,
        }
    }

    pub fn transmit(&mut self, symbols: Vec<u8>) {
        let (config, device) = self.init_output_device();
        let carrier = config.carrier;
        let deviation = config.deviation;
        let samplerate = config.samplerate;
        let baudrate = config.baudrate;
        let channels = config.channels;
        let latency = 1.0 / baudrate as f32 * samplerate as f32;

        let mut samples = vec![0f32; 7];
        samples.extend(modulator::bfsk(
            &symbols,
            carrier as f32,
            deviation as f32,
            samplerate as u32,
            latency as usize,
        ));

        if channels == 2 {
            samples = samples
                .iter()
                .flat_map(|s| vec![*s, *s])
                .collect::<Vec<f32>>();
        }
        // save_wave("upsample.wav", samples.clone(), samplerate, channels as u16);

        let mut speaker = Speaker::new(samplerate, baudrate as u32, channels as u16, device);
        speaker.play(samples);
    }

    fn init_input_device(&mut self) -> (Modem, Device) {
        let mut config = self.clone();
        let host = cpal::default_host();
        let device = match self.input_device {
            Some(ref device) => {
                let input_device = host
                    .input_devices()
                    .expect("no input device available")
                    .find(|d| d.name().unwrap() == *device)
                    .expect("no input device available");
                input_device
            }
            None => {
                let input_device = host.default_input_device().unwrap();
                input_device
            }
        };

        let default_config = device.default_input_config().unwrap();
        config.samplerate = default_config.sample_rate().0;
        config.channels = default_config.channels() as u8;
        config.output_device = Some(device.name().unwrap());

        (config, device)
    }
    fn init_output_device(&mut self) -> (Modem, Device) {
        let mut config = self.clone();
        let host = cpal::default_host();
        let device = match self.output_device {
            Some(ref device) => {
                let input_device = host
                    .output_devices()
                    .expect("no input device available")
                    .find(|d| d.name().unwrap() == *device)
                    .expect("no input device available");
                input_device
            }
            None => {
                let input_device = host.default_output_device().unwrap();
                input_device
            }
        };

        let default_config = device.default_output_config().unwrap();
        config.samplerate = default_config.sample_rate().0;
        config.channels = default_config.channels() as u8;
        config.output_device = Some(device.name().unwrap());

        (config, device)
    }

    pub fn record(&mut self, connection_tx: mpsc::Sender<Datalink>) {
        let (config, input_device) = self.init_input_device();
        let carrier = self.carrier;
        let deviation = self.deviation;
        let samplerate = config.samplerate;
        let baudrate = self.baudrate;
        let channels = config.channels as u8;
        let threshold = self.threshold;

        let latency = 1.0 / baudrate as f32 * samplerate as f32;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream_config = StreamConfig {
            channels:    channels as u16,
            sample_rate: cpal::SampleRate(samplerate),
            buffer_size: cpal::BufferSize::Fixed(latency as u32 * channels as u32),
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

        stream.play().expect("failed to play stream");

        let mut recent_bin: VecDeque<i8> = vec![-1; 8].into();
        let mut input_data: VecDeque<i8> = vec![].into();

        let mut frame_length = 0;
        let mut phy_frame: Option<Physical> = None;
        let mut status = Status::LISTENING;

        let current_band = Band::new(carrier as u32, deviation as u32, threshold as u32);

        loop {
            if let Ok(sample) = consumer.recv() {
                let sample = sample
                    .chunks(channels as usize)
                    .map(|c| c[0])
                    .collect::<Vec<f32>>();
                let sample = highpass_filter(&sample, samplerate as f32, 3000.0);
                // let sample = lowpass_filter(&sample, samplerate as f32, 5000.0);
                let resfreq = fftfreq(&sample, samplerate).expect("failed to fftfreq");
                if resfreq == 10900.0 {
                    continue;
                }
                // println!("{}", resfreq);

                // 自分のバンドに入っている周波数は無視｡
                if current_band.has_freq(resfreq) {
                    // print!("!");
                    // continue;
                }
                // if BAND2.has_freq(resfreq) {
                //     println!("band2 {}", resfreq);
                // }

                let bit = which_band(resfreq, vec![BAND1, BAND2, BAND3]);
                recent_bin.push_back(bit);
                recent_bin.pop_front();
                // print!(" bit {:2?} ", bit);

                match status {
                    Status::LISTENING => {
                        // println!("{:?}", recent_bin);
                        if recent_bin == ISFD {
                            println!("sfd received");
                            status = Status::RECEIVING;
                        }
                    }
                    Status::RECEIVING => {
                        input_data.push_back(bit);
                        if bit.is_negative() {
                            println!("{:?}", recent_bin);
                            phy_frame = None;
                            input_data.clear();
                            frame_length = 0;
                            status = Status::RESET;
                        }

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
                                status = Status::RESET;
                                continue;
                            }
                        };
                        if proto.detect_checksum() {
                            // println!("checksum ok");
                            println!("from: {}", proto.source_address);
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
                        status = Status::RESET;
                    }
                    Status::RESET => {
                        phy_frame = None;
                        input_data.clear();
                        frame_length = 0;
                        recent_bin = vec![-1; 8].into();
                        status = Status::LISTENING;
                    }
                }
            }
        }
    }
}

fn mono_to_stereo(samples: &[f32]) -> Vec<f32> {
    let mut stereo = vec![];
    for s in samples {
        stereo.push(*s);
        stereo.push(*s);
    }
    stereo
}
