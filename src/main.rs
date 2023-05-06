#[warn(unused_imports)]
// use std::error::Error;
// use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut _stream, _)) => {
                tokio::spawn(async move {
                    stream_handler(_stream).await;
                });
            }
            Err(_e) => {
                eprintln!("Error processing incoming client");
            }
        };
    }

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut _stream) => {
    //             println!("accepted new connection");
    //             loop {
    //                 let mut ping_buf = [0; 512];
    //                 _stream.read(&mut ping_buf).unwrap();
    //                 if ping_buf.is_empty() {
    //                     println!("Connection closed!");
    //                     break;
    //                 }
    //                 let pong_str = "+PONG\r\n";
    //                 _stream.write(&pong_str.as_bytes()).expect("Pong message err");
    //             }
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

// function that handles an incoming ping
async fn stream_handler(mut stream : TcpStream) {
    println!("accepted new connection");
    loop {
        let mut ping_buf = [0; 512];
        match stream.read(&mut ping_buf).await {
            Ok(0) => break,
            Ok(_n) => (),
            Err(_e) => eprintln!("Err reading in stream handler {}", _e),
        }
        let pong_str = "+PONG\r\n";
        stream.write(&pong_str.as_bytes()).await.expect("Error writing output");
    }
    println!("Connection closed!");
}


// test run: echo -e "ping\nping" | redis-cli