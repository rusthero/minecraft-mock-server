use std::io::{ErrorKind, Result};
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    println!("listening for clients");

    loop {
        let (stream, addr) = listener.accept().await?;
        handle_client(stream, addr).await?;
    }
}

async fn handle_client(stream: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("new client: {addr:?}");
    loop {
        stream.readable().await?;
        let mut buf = [0; 64];
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {n} bytes");
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
        println!("{buf:x?}");
    }

    Ok(())
}
