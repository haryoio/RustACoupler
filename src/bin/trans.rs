extern crate popemodem;
use std::{env, f32::consts::PI, i16, io::Error, sync::mpsc, thread, time::Duration};

use hound::{self, WavWriter};
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use popemodem::{
    config::ModemConfig,
    receiver::Receiver,
    save::save_wav,
    transmitter::Transmitter,
    utils::repeat,
    ModulationMethod,
};

use crate::popemodem::ascii::encode_u8;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut send_data = "n";
    // if args.len() < 2 {
    //     println!("Usage: {} [send|recv]", args[0]);
    //     return Ok(());
    // }
    if args.len() >= 2 {
        send_data = &args[1];
    }

    let mut config = ModemConfig::default();
    config.modulation_method = ModulationMethod::QFSK;
    let mut trans = Transmitter::new(config.clone());
    let data = trans.fsk(&send_data);
    // let data = trans.bfsk(&send_data);
    // save_wav("bandpass.wav", data.clone(), config.samplerate as u32);
    trans.play(&data);

    return Ok(());
}
