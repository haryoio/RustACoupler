use std::{
    path::Path,
    sync::{mpsc::channel, Arc},
    thread,
};

use crate::{config::ModemConfig, receiver};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FrameCount, StreamConfig,
};

use criterion_plot::prelude::*;

use feos_dft::Axis;
use itertools_num::linspace;
use nalgebra::ComplexField;
use ringbuf::HeapRb;
use rustfft::algorithm::Dft;
use rustfft::num_complex::Complex;
use rustfft::Fft;
use rustfft::FftDirection;
use rustfft::FftPlannerAvx;

#[derive(Clone)]
pub struct Receiver {
    pub(crate) config: ModemConfig,
}

impl Receiver {
    pub fn new(config: ModemConfig) -> Receiver {
        return Receiver { config };
    }

    pub fn run(&mut self) -> Result<(), String> {
        let threshold = 300;

        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .expect("failed to find input device");

        let frame = self.config.latency as f32 * self.config.samplerate as f32;
        println!("{}", frame);
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream_config = StreamConfig {
            channels: self.config.channels as u16,
            sample_rate: cpal::SampleRate(44100 as u32),
            buffer_size: cpal::BufferSize::Fixed(
                (self.config.latency as usize).try_into().unwrap(),
            ),
        };
        // let stream_config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();

        // The buffer to share samples
        let ring = HeapRb::<Vec<f32>>::new(frame as usize * 2);
        let (mut producer, mut consumer) = ring.split();

        // for i in 0..self.config.latency as usize {
        //     let bb = vec![0.0; self.config.latency as usize];
        //     producer.push(bb).unwrap();
        // }

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            producer.push(data.to_vec()).unwrap();
        };

        let input_stream =
            match input_device.build_input_stream(&stream_config, input_data_fn, err_fn) {
                Ok(stream) => stream,
                Err(err) => panic!("{}", err),
            };

        input_stream.play().unwrap();

        let config = self.config.clone();

        let mut res = vec![];

        let fft_axis = linspace::<f32>(
            0.0,
            self.config.samplerate,
            self.config.latency.floor() as usize,
        );
        println!("axis {:?}", fft_axis);
        // let buflen = 512;
        let handle = thread::spawn(move || {
            loop {
                while let Some(buffer) = consumer.pop() {
                    let mut data: Vec<Complex<f32>> = to_c32(&buffer);
                    let dft = Dft::new(config.latency as usize, FftDirection::Inverse);
                    dft.process(&mut data);
                    // let (max_index, _max) = data.iter().enumerate().fold(
                    //     (usize::MIN, f32::MIN),
                    //     |(i_a, a), (i_b, &b)| {
                    //         if b.norm() > a {
                    //             (i_b, b.norm())
                    //         } else {
                    //             (i_a, a)
                    //         }
                    //     },
                    // );
                    let d = data
                        .iter()
                        .map(|i| &Complex::new(10f32, 0f32) * (i * 32767.0).norm())
                        .collect::<Vec<_>>();
                    println!("{:?}", d);

                    // println!("max ind {}", max_index);
                    // let resfreq = max_index * 100;
                    // if resfreq >= (self.config.low_freq as usize - threshold)
                    //     && resfreq <= self.config.low_freq as usize + threshold
                    // {
                    //     res.push(0);
                    // } else if resfreq >= (self.config.high_freq as usize - threshold)
                    //     && resfreq <= self.config.high_freq as usize + threshold
                    // {
                    //     res.push(1);
                    // } else {
                    res.push(-1);
                    // }
                    if res.len() == 8 {
                        // println!("{:?}", res);
                        res.clear();
                    }
                }
            }
        });
        handle.join().unwrap();
        return Ok(());
    }
}

fn to_c32(slice: &[f32]) -> Vec<Complex<f32>> {
    slice.iter().map(|&re| Complex::new(re, 0.0)).collect()
}

// pub fn plot_vector(y_values: Vec<f32>, dataname: &'static str, filename: &'static str, log: bool) {
//     let x_values = linspace::<f32>(0.0, y_values.len() as f32, y_values.len()).collect::<Vec<_>>();

//     // Make a new Figure to plot our vector:
//     let mut f = Figure::new();
//     let pp = Path::new(filename);
//     println!("{:?}", pp);
//     // Configure settings for the output of the plot:
//     f.set(Font("monospace"));
//     f.set(FontSize(16.0));

//     f.set(Output(Path::new(filename)));
//     f.set(Size(1000, 400));

//     // If log, set y axis to log mode:
//     f.configure(Axis::BottomX, |a| {
//         a.set(Scale::Linear).set(Range::Limits(1.0, 210.0))
//     });
//     f.configure(Axis::LeftY, |a| {
//         a.set(Scale::Linear).set(Range::Limits(-200.0, 200.0))
//     });

//     // Configure the key for the plot
//     f.configure(Key, |k| {
//         k.set(Boxed::Yes)
//             .set(Position::Inside(Vertical::Top, Horizontal::Left))
//     });

//     // Plot the vector (in memory):
//     f.plot(
//         Lines {
//             x: x_values,
//             y: y_values,
//         },
//         |l| {
//             l.set(Color::Rgb(255, 0, 0))
//                 .set(Label(dataname))
//                 .set(LineType::Solid)
//         },
//     );

//     // Spit out the plot to a .svg file:
//     f.save(pp);
// }
