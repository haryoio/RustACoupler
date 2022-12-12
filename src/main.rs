extern crate popemodem;
use std::{
    io::{self, BufRead, BufReader},
    sync::{mpsc, Arc, RwLock},
    thread,
};

use byteorder::WriteBytesExt;
use clap::Parser;
use popemodem::{
    args::{parse_args, Args},
    datalink::frame::{Datalink, FrameType},
    error::Error,
    modem::{protocol::Protocol, Modem},
};

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let src = args.address;

    let connections: Arc<RwLock<Vec<Datalink>>> = Arc::new(RwLock::new(Vec::new()));

    let config = parse_args(args)?;
    // config.modulation_format = ModulationFormat::QFSK;
    // let config = config;

    let mut modem_tasks = vec![];
    let connections_1 = connections.clone();
    let receiver_config = config.clone();
    modem_tasks.push(thread::spawn(move || {
        let mut receivers_tasks = vec![];
        let (tx, rx) = mpsc::channel();
        receivers_tasks.push(thread::spawn(move || {
            let mut modem = Modem::new(receiver_config);
            modem.record(tx);
        }));
        receivers_tasks.push(thread::spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    if let Ok(mut d) = connections_1.write() {
                        d.push(data);
                    }
                }
            }
        }));
        for task in receivers_tasks {
            task.join().expect("Failed to join task");
        }
    }));

    let connections_2 = connections.clone();
    let transmitter_config = config.clone();

    modem_tasks.push(thread::spawn(move || {
        let (send, recv) = mpsc::channel();

        let mut transmitters_tasks = vec![];

        let sender = send.clone();
        transmitters_tasks.push(thread::spawn(move || {
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            let reader = BufReader::new(stdin);

            let mut lines = reader.lines();

            loop {
                for line in lines.by_ref() {
                    stdout.write_i8(0).unwrap();
                    let line = line.unwrap();
                    let protocol = Protocol::new(&line, src, 255, 0, FrameType::Data);
                    sender.send(protocol).unwrap();
                }
            }
        }));
        let sender = send.clone();
        transmitters_tasks.push(thread::spawn(move || {
            // // ack
            // loop {
            //     if let Some(frame) = connections_2.write().unwrap().pop() {
            //         let protocol = Protocol::new(
            //             "",
            //             frame.destination_address,
            //             src,
            //             frame.sequence_number,
            //             FrameType::Acknowledgement,
            //         );
            //         sender.send(protocol).unwrap();
            //     }
            // }
        }));

        let mut modem = Modem::new(transmitter_config);
        while let Ok(frame) = recv.recv() {
            modem.transmit(frame.to_bytes());
        }

        for task in transmitters_tasks {
            task.join().expect("Failed to join task");
        }
    }));

    for task in modem_tasks {
        task.join().expect("Failed to join task");
    }

    Ok(())
}
