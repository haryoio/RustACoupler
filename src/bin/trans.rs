extern crate popemodem;
use std::{
    io::{self, BufRead, BufReader},
    sync::{mpsc, RwLock},
    thread,
};

use popemodem::{
    bytes::decode_u8,
    config::ModemConfig,
    datalink::frame::Datalink,
    error::Error,
    modem::Modem,
};

static mut CONNECTIONS: RwLock<Vec<Datalink>> = RwLock::new(Vec::new());

// #[tokio::main]
fn main() -> Result<(), Error> {
    // let args: Vec<String> = env::args().collect();

    let mut n_tasks = vec![];

    n_tasks.push(thread::spawn(|| {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let config = ModemConfig::default();
            let mut modem = Modem::new(config);
            modem.record(tx);
        })
        .join()
        .unwrap();
        loop {
            if let Ok(data) = rx.recv() {
                unsafe { CONNECTIONS.write().unwrap().push(data) };
            }
        }
    }));

    n_tasks.push(thread::spawn(move || {
        println!("Starting modem");

        let (send, recv) = mpsc::channel();

        let mut tasks = vec![];

        let sender = send.clone();
        tasks.push(thread::spawn(move || {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            let mut lines = reader.lines();

            loop {
                for line in lines.by_ref() {
                    let line = line.unwrap();
                    sender.send((line, 255, 0)).unwrap();
                }
            }
        }));

        tasks.push(thread::spawn(move || {
            loop {
                unsafe {
                    CONNECTIONS.read().unwrap().iter().for_each(|c| {
                        if c.sequence_number == 0 {
                            send.send((
                                decode_u8(c.data.clone()),
                                c.source_address,
                                c.sequence_number + 1,
                            ))
                            .unwrap();
                        }
                    });
                }
            }
        }));

        let config = ModemConfig::default();
        let modem = Modem::new(config);
        while let Ok((line, dst, seq)) = recv.recv() {
            modem.transmit(&line, dst, seq);
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
