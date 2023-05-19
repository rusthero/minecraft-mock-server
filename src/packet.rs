use std::io::{Cursor, Read};

use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use uuid::Uuid;

use crate::data::{encode_string, encode_var_int, Chat, ReadFromBytes};

pub struct Packet {
    pub length: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

impl From<&[u8]> for Packet {
    fn from(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);
        Packet {
            length: i32::read_from(&mut cursor),
            id: i32::read_from(&mut cursor),
            data: cursor.get_ref()[cursor.position() as usize..].to_vec(),
        }
    }
}

impl From<StatusResponsePacket> for Packet {
    fn from(packet: StatusResponsePacket) -> Self {
        let data = serde_json::to_string(&packet).unwrap();
        Packet::from_data(0x0i32, encode_string(&data))
    }
}

impl Packet {
    pub fn from_data(id: i32, data: Vec<u8>) -> Packet {
        // Calculate length of id as VarInt and add to data length
        let mut length = 0i32;
        let mut val = id;
        loop {
            length += 1;
            val >>= 7;

            if val == 0 {
                break;
            }
        }
        length += data.len() as i32;

        Packet { length, id, data }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        bytes.extend(encode_var_int((&self).length));
        bytes.extend(encode_var_int((&self).id));
        bytes.extend(&self.data);
        bytes
    }

    pub async fn send(&self, stream: &TcpStream) {
        stream
            .try_write(self.as_bytes().as_slice())
            .expect("failed sending packet");
    }
}

pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl From<Packet> for HandshakePacket {
    fn from(packet: Packet) -> Self {
        let mut cursor = Cursor::new(packet.data.as_slice());
        HandshakePacket {
            protocol_version: i32::read_from(&mut cursor),
            server_address: String::read_from(&mut cursor),
            server_port: u16::read_from(&mut cursor),
            next_state: i32::read_from(&mut cursor),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponsePacket {
    pub version: StatusResponseVersion,
    pub players: StatusResponsePlayers,
    pub description: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponseVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<StatusResponsePlayersSample>,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResponsePlayersSample {
    pub name: String,
    pub id: Uuid,
}
