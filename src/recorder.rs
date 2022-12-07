use std::{
    collections::VecDeque,
    future::Future,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender, SyncSender},
        Arc,
        Mutex,
        RwLock,
    },
    thread,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
    Stream,
    StreamConfig,
};
use futures::{future::BoxFuture, FutureExt};
use ringbuf::{Consumer, HeapRb, SharedRb};
use tokio::task;

use crate::{
    bytes::decode_u8,
    config::ModemConfig,
    datalink::frame::Datalink,
    modem::{
        demodulator::detect_bfsk,
        filter::{fftfreq, lowpass_filter},
        protocol::Protocol,
    },
    physical::frame::Physical,
    Status,
    ISFD,
};

type Fp = Box<dyn FnMut(Vec<f32>, Arc<RwLock<AtomicBool>>) -> BoxFuture<'static, ()> + Send + Sync>;
pub struct Recorder {
    samplerate: u32,
    latency:    u32,
    channels:   u16,
    device:     Option<String>,
    running:    Arc<RwLock<AtomicBool>>,
    roop:       bool,
}
pub struct RecorderStream {
    pub consumer: Receiver<Vec<f32>>,
    pub stream:   Arc<Mutex<Stream>>,
    // running:      Arc<RwLock<AtomicBool>>,
    // cb:       Fp,
}

async fn callback<C>(data: Vec<f32>, run: Arc<RwLock<AtomicBool>>, mut operation: C) -> ()
where
    C: FnMut(Vec<f32>, Arc<RwLock<AtomicBool>>) -> BoxFuture<'static, ()>,
{
    operation(data, run).await;
}

impl Recorder {
    pub fn new(samplerate: u32, latency: u32, channels: u16) -> Self {
        Recorder {
            samplerate,
            latency,
            channels,
            device: None,
            running: Arc::new(RwLock::new(AtomicBool::new(false))),
            roop: false,
        }
    }

    pub fn device(&mut self, device: &str) {
        self.device = Some(device.to_string());
    }

    // pub fn record(&mut self, producer: Sender<Vec<f32>>) -> RecorderStream {
    pub fn record(&mut self, config: ModemConfig) {
        let host = cpal::default_host();
        let input_device = match self.device {
            None => host.default_input_device(),
            Some(ref device_name) => {
                let mut res: Option<Device> = None;
                for i in host.input_devices().unwrap() {
                    if *device_name == i.name().unwrap() {
                        res = Some(i);
                        break;
                    }
                }
                res
            }
        }
        .expect("input device not found");

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream_config = StreamConfig {
            channels:    self.channels,
            sample_rate: cpal::SampleRate(self.samplerate),
            buffer_size: cpal::BufferSize::Fixed(self.latency),
        };

        // let ring = HeapRb::<Vec<f32>>::new(self.latency as usize);
        // let (producer, consumer) = mpsc::channel::<Vec<f32>>();
        let (producer, consumer) = mpsc::channel();

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // println!("data: {:?}", data);
            if let Err(e) = producer.send(data.to_vec()) {
                eprintln!("error: {}", e);
            }
        };
        let stream = input_device
            .build_input_stream(&stream_config, input_data_fn, err_fn)
            .expect("failed to build input stream");
        // 録音開始
        self.running.write().unwrap().store(true, Ordering::Relaxed);
        stream.play().expect("failed to play stream");
        println!("recording...");

        let mut recent_bin: VecDeque<i8> = vec![-1; 8].into();
        let mut input_data: VecDeque<i8> = vec![].into();

        // recorder_stream.stream.lock().unwrap().play().unwrap();
        // let consumer = recorder_stream.consumer;

        let mut frame_length = 0;
        let mut phy_frame: Option<Physical> = None;
        let mut status = Status::LISTENING;

        println!("start demodulatoin");
        loop {
            if let Ok(sample) = consumer.recv() {
                // println!("recv");
                // let sample = lowpass_filter(&sample, self.samplerate as f32, 3000.0);

                let resfreq = fftfreq(sample, self.samplerate as f32).unwrap();

                // if resfreq == 10900.0 {
                //     continue;
                // }
                // println!("resfreq: {}", resfreq);

                let bit = detect_bfsk(
                    resfreq,
                    config.carrier,
                    config.deviation,
                    config.threshold as f32,
                );
                // print!("{}", bit);

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
                        // println!("bin: {:?}", recent_bin);
                        if bin == ISFD {
                            status = Status::RECEIVING;
                            println!("SFD detected");
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
                            println!("frame: {:?}", frame);
                            frame_length = frame.length as usize;
                            phy_frame = Some(frame);
                            input_data.clear();
                        }
                        if frame_length > 0 {
                            frame_length -= 1;
                        }
                        if frame_length == 0 && phy_frame.is_some() {
                            status = Status::ANSWER;
                            println!("frame received");
                        }
                        // recent_bin.clear();
                    }
                    Status::ANSWER => {
                        input_data.push_back(bit);
                        let mut frame = vec![];
                        for b in input_data.iter() {
                            frame.push(*b as u8);
                        }
                        let proto = Datalink::from_bytes(&frame).unwrap();
                        if proto.detect_checksum() {
                            println!("checksum ok");
                        } else {
                            println!("checksum error");
                        }
                        let data = decode_u8(proto.data);
                        println!("data: {:?}", data);

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

    // async fn pause(&mut self) {
    //     self.running
    //         .blocking_write()
    //         .store(false, Ordering::Relaxed);
    // }
}

#[cfg(test)]
mod recorder_test {
    use super::*;
    use crate::synthesizer::save_wave;

    #[tokio::test]
    async fn test_recorder() {
        let samplerate = 44100;
        let baudrate = 100;
        let latency = 1.0 / baudrate as f32 * samplerate as f32;
        let mut record = Recorder::new(samplerate, latency as u32, 1);

        let num_samples = 1000;
        let mut count = 0;
        // let mut samples = Arc::new(Mutex::new(vec![]));
        let (producer, consumer) = mpsc::sync_channel::<Vec<f32>>(2);
        record.record(ModemConfig::default());
        // stream.stream.lock().unwrap().play().unwrap();
        let mut samples = vec![];
        loop {
            if let Ok(data) = consumer.recv() {
                println!("data: {:?}", data);
                samples.extend(data);
                count += 1;
                if count > num_samples {
                    break;
                }
            }
        }
        // stream.stream.lock().unwrap().pause().unwrap();
        save_wave("test.wav", samples, samplerate);
    }
}
