use std::io::{Read, Write};
use std::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut ping_buf = [0; 512];
                _stream.read(&mut ping_buf).unwrap();
                let pong_str = "+PONG\r\n";
                _stream.write(&pong_str.as_bytes()).expect("Pong message err");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
