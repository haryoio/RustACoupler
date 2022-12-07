use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "PePoModem",
    version = "0.1.0",
    author = "Haryoiro <mizusecocolte@gmail.com>"
)]
pub struct Args {
    #[clap(short = 'a', long = "address", default_value = "0")]
    pub address: u8,

    #[clap(short = 'i', long = "input", default_value = "default")]
    pub input_device:  Option<String>,
    #[clap(short = 'o', long = "output", default_value = "default")]
    pub output_device: Option<String>,

    #[clap(short = 's', long = "samplerate", default_value = "44100")]
    pub samplerate: u32,
    #[clap(short = 'b', long = "baudrate", default_value = "100")]
    pub baudrate:   u16,

    #[clap(short = 'c', long = "carrier", default_value = "1200")]
    pub carrier:   u32,
    #[clap(short = 'd', long = "deviation", default_value = "1200")]
    pub deviation: u32,

    #[clap(short = 'r', long = "resend")]
    pub resending: bool,
}
