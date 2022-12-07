extern crate popemodem;

use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::{self, sleep},
};

// アドレスとポートを指定
const SERVER_ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    let (socket, mut _so) = listener.accept().unwrap();
    // let (tx, rx) = channel(3);

    // let rx = Arc::new(Mutex::new(rx));
    let socket = Arc::new(Mutex::new(socket));
    let socket3 = socket.clone();
    let mut handles = vec![];
    handles.push(thread::spawn(move || {
        let mut buf = vec![0; 1024];

        match socket.lock().unwrap().read(&mut buf) {
            Ok(0) => {}
            Ok(n) => {
                let a = &buf[..n];
                print!("{}", String::from_utf8(a.to_vec()).unwrap());
            }
            Err(_) => {}
        }
    }));
    handles.push(thread::spawn(move || {
        loop {
            sleep(tokio::time::Duration::from_millis(10));
            socket3.lock().unwrap().write_all(".".as_bytes()).unwrap();
            socket3.lock().unwrap().flush().unwrap();
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}
