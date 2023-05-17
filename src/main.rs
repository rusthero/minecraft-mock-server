use std::io::{ErrorKind, Result};
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

use minecraft_server::data::{read_string, read_u16, read_var_int};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    println!("listening for clients (protocol v1.19.4)");

    loop {
        let (stream, addr) = listener.accept().await?;
        handle_client(stream, addr).await?;
    }
}

async fn handle_client(stream: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("new client: {addr:?}");

    loop {
        stream.readable().await?;

        let mut buf = [0; 4096];
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) if n >= 2 => {
                let mut buf = buf.to_vec();

                let length = read_var_int(&mut buf);
                let id = read_var_int(&mut buf);

                match id {
                    0 => {
                        println!("received handshake packet");
                        if length == 1 {
                            println!("it is empty");
                        } else {
                            process_handshake(buf).await;
                        }
                    }
                    _ => {
                        println!("received {n} bytes of unknown packet");
                    }
                }

                println!("packet length: {length}, id: {id:x?}")
            }
            Ok(_) => {
                println!("packet too small")
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

async fn process_handshake(mut buf: Vec<u8>) {
    let pro_ver = read_var_int(&mut buf);
    let srv_addr = read_string(&mut buf);
    let srv_port = read_u16(&mut buf);
    let next_state = read_var_int(&mut buf);

    println!("protocol version: {pro_ver}, server address: {srv_addr}, server port: {srv_port}, next state: {next_state}");
}
