use std::io::{ErrorKind, Result};
use std::net::SocketAddr;
use std::str::FromStr;

use tokio::net::TcpStream;
use uuid::Uuid;

use crate::data::Chat;
use crate::packet::{
    HandshakePacket, Packet, StatusResponsePacket, StatusResponsePlayers,
    StatusResponsePlayersSample, StatusResponseVersion,
};

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
    pub async fn handle_handshaking(self) -> Result<()> {
        loop {
            self.stream.readable().await?;

            let mut buf = [0; 4096];
            match self.stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) if n >= 2 => {
                    let packet = Packet::from(buf.as_slice());
                    // Confirm if packet is an handshake packet
                    if packet.id != 0x0 || packet.length <= 1 {
                        // TODO Investigate packet length being 1 when handshaking
                        continue;
                    }
                    let packet = HandshakePacket::from(packet);
                    // TODO Check if packet data is valid
                    let client = HandshakingClient { client: self };
                    println!("HANDSHAKE: Protocol Version: {}, Server Address: {}, Server Port: {}, Next State: {}", packet.protocol_version, packet.server_address, packet.server_port, packet.next_state);

                    if packet.next_state == 1 {
                        client.handle_status().await;
                    } else if packet.next_state == 2 {
                        client.handle_login().await;
                    }

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

struct HandshakingClient {
    client: Client,
}

impl HandshakingClient {
    async fn handle_login(self) {}
    async fn handle_status(self) {
        Packet::from(StatusResponsePacket {
            version: StatusResponseVersion {
                name: String::from("1.19.4"),
                protocol: 762,
            },
            players: StatusResponsePlayers {
                max: 20,
                online: 1,
                sample: vec![StatusResponsePlayersSample {
                    name: String::from("Notch"),
                    id: Uuid::from_str("069a79f444e94726a5befca90e38aaf5").unwrap(),
                }],
            },
            description: Chat::literal("A Minecraft Server written in Rust"),
            favicon: None,
            enforces_secure_chat: false,
        })
        .send(&self.client.stream)
        .await;

        PingingClient {
            client: self.client,
        }
        .handle_ping()
        .await
        .expect("failed handling pinging");
    }
}

struct PingingClient {
    client: Client,
}

impl PingingClient {
    pub async fn handle_ping(self) -> Result<()> {
        loop {
            let mut buf = [0; 4096];
            // TODO Implement some kind of timeout (Notchian server waits for 30 seconds)
            match self.client.stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    // Send incoming ping request back as pong response
                    self.client
                        .stream
                        .try_write(buf.as_slice())
                        .expect("failed sending pong response");
                    //Packet::from(buf.as_slice()).send(&self.client.stream).await;
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
