use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    config::{Band, ModemConfig},
    devices::{list_input_devices, list_output_devices},
    error::Result,
};

#[derive(Debug, Parser)]
#[clap(
    name = "PePoModem",
    version = "0.1.0",
    author = "Haryoiro <mizusecocolte@gmail.com>"
)]
pub struct Args {
    #[clap(short = 'a', long = "address", default_value = "0")]
    pub address: u8,

    #[clap(short = 'i', long = "input")]
    pub input_device:  Option<String>,
    #[clap(short = 'o', long = "output")]
    pub output_device: Option<String>,

    #[clap(short = 's', long = "samplerate")]
    pub samplerate: Option<u32>,
    #[clap(short = 'b', long = "baudrate")]
    pub baudrate:   Option<u16>,
    #[clap(short = 't', long = "threshold")]
    pub threshold:  Option<u32>,

    #[clap(short = 'c', long = "carrier")]
    pub carrier:   Option<u32>,
    #[clap(short = 'd', long = "deviation")]
    pub deviation: Option<u32>,

    #[arg(value_enum)]
    pub band: Option<Bands>,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(name = "list")]
    List {
        #[clap(subcommand)]
        command: ListSubCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ListSubCommand {
    Input,
    Output,
    SupportedFormats,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Bands {
    Band1,
    Band2,
    Band3,
}

pub fn parse_args(args: Args) -> Result<ModemConfig> {
    let mut config = ModemConfig::default();

    // bandが指定されている場合､samplerate, baudrate, carrierは指定できない
    if args.band.is_some()
        && (args.carrier.is_some() || args.deviation.is_some() || args.threshold.is_some())
    {
        println!("You can't specify a band and a carrier deviation or threshold at the same time");
        std::process::exit(1);
    }

    if args.input_device.is_some() {
        config.input_device = args.input_device;
    }
    if args.output_device.is_some() {
        config.output_device = args.output_device;
    }
    if args.samplerate.is_some() {
        config.samplerate = args.samplerate.unwrap();
    }
    if args.baudrate.is_some() {
        config.baudrate = args.baudrate.unwrap();
    }
    if args.carrier.is_some() {
        config.carrier = args.carrier.unwrap();
    }
    if args.deviation.is_some() {
        config.deviation = args.deviation.unwrap();
    }
    if args.threshold.is_some() {
        config.threshold = args.threshold.unwrap();
    }

    if args.band.is_some() {
        let band = Band::from(args.band.unwrap());
        config.threshold = band.threshold;
        config.deviation = band.deviation;
        config.carrier = band.carrier;
    }

    if args.command.is_some() {
        let command = args.command.unwrap();
        match command {
            Command::List { command } => {
                match command {
                    ListSubCommand::Input => {
                        if let Err(e) = list_input_devices() {
                            println!("Error: {}", e);
                            process::exit(1)
                        }
                    }
                    ListSubCommand::Output => {
                        if let Err(e) = list_output_devices() {
                            println!("Error: {}", e);
                            process::exit(1)
                        }
                    }
                    ListSubCommand::SupportedFormats => {}
                }
            }
        }
        process::exit(0);
    }
    Ok(config)
}
