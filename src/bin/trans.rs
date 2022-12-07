extern crate popemodem;
use std::{
    io::{self, BufRead, BufReader},
    sync::{mpsc, Arc, RwLock},
    thread,
};

use byteorder::WriteBytesExt;
use popemodem::{
    config::ModemConfig,
    datalink::frame::{Datalink, FrameType},
    error::Error,
    modem::{protocol::Protocol, Modem},
};

fn main() -> Result<(), Error> {
    // let args: Vec<String> = env::args().collect();
    let connections: Arc<RwLock<Vec<Datalink>>> = Arc::new(RwLock::new(Vec::new()));
    let src = 10;

    let mut n_tasks = vec![];
    let connections_1 = connections.clone();
    n_tasks.push(thread::spawn(move || {
        let mut nn_tasks = vec![];
        let (tx, rx) = mpsc::channel();
        nn_tasks.push(thread::spawn(move || {
            let config = ModemConfig::default();
            let mut modem = Modem::new(config);
            modem.record(tx);
        }));
        nn_tasks.push(thread::spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    if let Ok(mut d) = connections_1.write() {
                        d.push(data);
                    }
                }
            }
        }));
        for task in nn_tasks {
            task.join().expect("Failed to join task");
        }
    }));

    let connections_2 = connections.clone();
    n_tasks.push(thread::spawn(move || {
        println!("Starting transmitter...");
        let (send, recv) = mpsc::channel();

        let mut tasks = vec![];

        let sender = send.clone();
        tasks.push(thread::spawn(move || {
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
        tasks.push(thread::spawn(move || {
            loop {
                if let Some(frame) = connections_2.write().unwrap().pop() {
                    let protocol = Protocol::new(
                        "",
                        frame.destination_address,
                        src,
                        frame.sequence_number,
                        FrameType::Acknowledgement,
                    );
                    sender.send(protocol).unwrap();
                }
            }
        }));

        let config = ModemConfig::default();
        let modem = Modem::new(config);
        while let Ok(frame) = recv.recv() {
            modem.transmit(frame.to_bytes());
        }

        for task in tasks {
            task.join().expect("Failed to join task");
        }
    }));

    for task in n_tasks {
        task.join().expect("Failed to join task");
    }

    Ok(())
}
