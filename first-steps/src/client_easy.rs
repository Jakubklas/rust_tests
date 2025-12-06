use std::thread::spawn;

use tokio::net::TcpStream;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream};
use tokio::time::{Duration, sleep};


async fn handler() {
    // Creates a client socket (Why does it have to be muteable? Because of the cahning buffer???)
    let addr = "localhost:8000";
    let mut socket = TcpStream::connect(addr).await.unwrap();
    println!("Connected to the server at {addr}...");

    // Writing the message into the socket and sending it
    socket.write_all(b"Hey bro, your mama is so fat that she can't even fit through a door!").await.unwrap();

    // Create a bytes buffer (for 1024 cahrs max) to write the response into
    let mut buf = vec![0u8; 1024];
    // Read the response from the socket into the buffer & get its length
    let n = socket.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Server responsed: {}", response);

    sleep(Duration::from_secs(1)).await;

    socket.write_all(b"...oh yeah and she stinks too!!!").await.unwrap();
    let n = socket.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Hear hear! The Server has spoken!\n ====== {response} ======");

    sleep(Duration::from_secs(1)).await;
}


#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    for _ in 0..10 {
        // Collecting async results
        // let _ = tokio::join!( async {handler().await} );         

        // Multi-threading results
        let handle = tokio::spawn( async move {
                handler().await
            });
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}