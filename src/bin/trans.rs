extern crate rust_a_coupler;
use crate::rust_a_coupler::binary::encode_u8;

use hound::{self, WavWriter};
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use portaudio as pa;
use rust_a_coupler::config::ModemConfig;
use rust_a_coupler::hamming::Hamming::calc_parity;
use rust_a_coupler::receiver::Receiver;
use rust_a_coupler::transmitter::Transmitter;
use rust_a_coupler::utils::repeat;
use std::i16;
use std::{f32::consts::PI, sync::mpsc};

enum Status {
    WAIT,
    READY,
}

fn main() -> Result<(), pa::Error> {
    let config = ModemConfig::default();
    let mut trans = Transmitter::new(config);
    let data = trans.modulation("hello");
    trans.play(&data);
    return Ok(());
}
