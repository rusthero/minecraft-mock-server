use std::io::{ErrorKind, Result};

use tokio::net::TcpListener;

use crate::client::Client;

mod client;
mod packet;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    println!("Listening for clients (Protocol v1.19.4)");

    loop {
        let client = Client::from(listener.accept().await?);
        client.handle_handshaking().await?;
    }
}
