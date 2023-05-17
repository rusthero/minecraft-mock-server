use std::io::{Cursor, ErrorKind, Result};
use std::net::SocketAddr;

use tokio::net::TcpStream;

use minecraft_server::data::FromBytes;

use crate::packet::{HandshakePacket, Packet};

pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

impl From<(TcpStream, SocketAddr)> for Client {
    fn from(conn: (TcpStream, SocketAddr)) -> Self {
        Client {
            stream: conn.0,
            addr: conn.1,
        }
    }
}

impl Client {
    pub async fn handle_handshaking(&self) -> Result<()> {
        loop {
            self.stream.readable().await?;

            let mut buf = [0; 128];
            match self.stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) if n >= 2 => {
                    let packet = Packet::from(buf.as_slice());
                    if packet.id != 0x0 || packet.length <= 1 {
                        // TODO Investigate packet length being 1 when handshaking
                        continue;
                    }
                    let packet = HandshakePacket::from(packet);
                    println!("HANDSHAKE: Protocol Version: {}, Server Address: {}, Server Port: {}, Next State: {}", 
                             packet.protocol_version, packet.server_address, packet.server_port, packet.next_state);
                    // TODO Initialize HandshakingClient and handle the login
                    break;
                }
                Ok(_) => {}
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
}

struct HandshakingClient<'a> {
    client: &'a Client,
}

impl<'a> From<&'a Client> for HandshakingClient<'a> {
    fn from(client: &'a Client) -> Self {
        HandshakingClient { client }
    }
}
