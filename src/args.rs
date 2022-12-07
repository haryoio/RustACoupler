use clap::{Parser, Subcommand};

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

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(name = "send")]
    Send {
        #[clap(short = 't', long = "target", default_value = "0")]
        target:  u8,
        #[clap(short = 'm', long = "message")]
        message: String,
    },
    #[clap(name = "receive")]
    Receive,
    #[clap(name = "list")]
    List {
        #[clap(short = 'i', long = "input")]
        input:  bool,
        #[clap(short = 'o', long = "output")]
        output: bool,
    },
}
