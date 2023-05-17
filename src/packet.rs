use std::io::Cursor;

use minecraft_server::data::FromBytes;

pub struct Packet {
    pub length: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

impl From<&[u8]> for Packet {
    fn from(buf: &[u8]) -> Self {
        let mut cursor = Cursor::new(buf);

        Packet {
            length: i32::from_bytes(&mut cursor),
            id: i32::from_bytes(&mut cursor),
            data: cursor.get_ref()[cursor.position() as usize..].to_vec(),
        }
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
            protocol_version: i32::from_bytes(&mut cursor),
            server_address: String::from_bytes(&mut cursor),
            server_port: u16::from_bytes(&mut cursor),
            next_state: i32::from_bytes(&mut cursor),
        }
    }
}
