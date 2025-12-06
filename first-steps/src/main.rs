use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener}};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap();          //I keep getting the std::net library suggested by my IDE. WHat's the difference? Asynchronous workflows?
    println!("Server runnning...\n");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();          //I understand that listener.accept() accepts a connection request but where is the actual client that sent the request - or why do we accept befreo even sending the request later in the loop?
        println!("Client connected from address {}", addr);

        tokio::spawn(async move {           // Explain again why we explicitly have to MOVE, not just reference the futures when multi-threading
            let mut buf = vec![0u8; 1024];      //1MB of space in the 'phone line'? I.e. that's how big the requests can be?

            let n: usize = socket.read(&mut buf).await.unwrap();     //Did we jsut tell the server socket to read whetever is in the buffer?
            let received = String::from_utf8_lossy(&buf[..n]);
            println!("Server has received: {received}");

            // Sending a response back to the client
            socket.write(b"All good, thanks for invoking this server!").await.unwrap();
        });
    }

}