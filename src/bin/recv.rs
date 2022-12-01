extern crate popemodem;

use std::io::Error;

use popemodem::{config::ModemConfig, receiver::Receiver, ModulationMethod};

fn main() -> Result<(), Error> {
    let mut config = ModemConfig::default();
    config.modulation_method = ModulationMethod::QFSK;
    let mut recv = Receiver::new(config);

    recv.run().unwrap();
    return Ok(());
}
