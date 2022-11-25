extern crate rust_a_coupler;
use std::{f32::consts::PI, i16, sync::mpsc};

use hound::{self, WavWriter};
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use portaudio as pa;
use rust_a_coupler::{
    config::ModemConfig,
    receiver::Receiver,
    transmitter::Transmitter,
    utils::repeat,
    ModulationMethod,
};

use crate::rust_a_coupler::ascii::encode_u8;

enum Status {
    WAIT,
    READY,
}
fn main() -> Result<(), pa::Error> {
    let mut config = ModemConfig::default();
    // config.modulation_method = ModulationMethod::QFSK;
    let mut recv = Receiver::new(config);

    recv.run().unwrap();
    return Ok(());
}
