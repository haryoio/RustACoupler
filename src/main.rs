extern crate popemodem;

use std::{io::Error, sync::Arc, thread, time::Duration};

use popemodem::{
    config::ModemConfig,
    receiver::Receiver,
    transmitter::Transmitter,
    ModulationMethod,
};
use tokio::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc::channel, Mutex},
};
// アドレスとポートを指定
const SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    loop {
        let (mut socket, mut so) = listener.accept().await?;
        // let (tx, rx) = channel(3);

        // let rx = Arc::new(Mutex::new(rx));
        let socket = Arc::new(Mutex::new(socket));
        let socket3 = socket.clone();

        tokio::spawn(async move {
            loop {
                println!("spawn1");
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                socket3
                    .lock()
                    .await
                    .write_all("hello".as_bytes())
                    .await
                    .unwrap();
                socket3.lock().await.flush().await.unwrap();
            }
        });
        tokio::spawn(async move {
            println!("spawn2");
            let mut buf = vec![0; 1024];

            loop {
                match socket.lock().await.read(&mut buf).await {
                    // `Ok(0)` が返ってきたらリモート側が閉じられたことを意味する
                    Ok(0) => return,
                    Ok(n) => {
                        // データをソケットへとコピーする
                        let a = &buf[..n];
                        print!("{}", String::from_utf8(a.to_vec()).unwrap());
                    }
                    Err(_) => {
                        // 予期しないソケットエラーが発生した場合。
                        // ここで何かできることはさほどないので、処理を停止する
                        return;
                    }
                }
            }
        });
    }
}
