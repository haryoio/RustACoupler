extern crate rust_a_coupler;
use std::{f32::consts::PI, i16, sync::mpsc};

use hound::{self, WavWriter};
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use pa::stream::Mode;
use portaudio as pa;
use rust_a_coupler::{
    config::ModemConfig,
    receiver::Receiver,
    transmitter::Transmitter,
    utils::repeat,
};

use crate::rust_a_coupler::ascii::encode_u8;

fn main() -> Result<(), pa::Error> {
    let config = ModemConfig::default();
    let mut ac = Transmitter::new(config);

    let config = ModemConfig::default();
    let mut ac = Receiver::new(config);

    return ac.run();
    // let data = ac.modulation("konnichiwa---asfdadf");
    // ac.play(&data);
}
