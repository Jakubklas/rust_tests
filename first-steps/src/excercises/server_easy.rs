use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpSocket}};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap();          //I keep getting the std::net library suggested by my IDE. WHat's the difference? Asynchronous workflows?
    println!("Server runnning...\n");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();          //I understand that listener.accept() accepts a connection request but where is the actual client that sent the request - or why do we accept befreo even sending the request later in the loop?
        println!("Client connected from address {}", addr);

        tokio::spawn(async move {           // Explain again why we explicitly have to MOVE, not just reference the futures when multi-threading
            let mut buf = vec![0u8; 1024];      //1MB of space in the 'phone line'? I.e. that's how big the requests can be?

            loop {

                let n = match socket.read(&mut buf).await {
                    Ok(0) => { println!("Client has disconnected..."); break },
                    Ok(n) => n,
                    Err(e) => { println!("Client Error {e}"); break },
                };

                let received = String::from_utf8_lossy(&buf[..n]);
                println!("Server has received: {received}");

                // Sending a response back to the client
                socket.write(b"Hmm, that's pretty mean dude...").await.unwrap();
            }
        
        });
    }

}