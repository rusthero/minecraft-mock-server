use std::io::{Cursor, Read};
use std::option::Option;

use serde::{Deserialize, Serialize};

pub trait ReadFromBytes {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self;
}

impl ReadFromBytes for i32 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        let mut result: i32 = 0;
        let mut shift: u32 = 0;

        loop {
            let mut byte = [0];
            if cursor.read_exact(&mut byte).is_err() {
                panic!("invalid varint format: buffer underflow");
            }

            let byte = byte[0];

            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                break;
            }

            if shift >= 32 {
                panic!("invalid varint format: overflow");
            }
        }

        result
    }
}

pub fn encode_var_int(value: i32) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut val = value as u32;

    loop {
        let mut byte = (val & 0b0111_1111) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0b1000_0000;
        }
        encoded.push(byte);
        if val == 0 {
            break;
        }
    }

    encoded
}

impl ReadFromBytes for String {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> String {
        let size: usize = i32::read_from(cursor) as usize;
        let mut buf = vec![0; size];

        if cursor.read_exact(&mut buf).is_err() {
            panic!("invalid string format: buffer underflow");
        }

        String::from_utf8_lossy(buf.as_slice()).into_owned()
    }
}

pub fn encode_string(string: &str) -> Vec<u8> {
    let mut encoded = encode_var_int(string.len() as i32);
    encoded.extend_from_slice(string.as_bytes());
    encoded
}

impl ReadFromBytes for u16 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> u16 {
        let mut buf = [0; 2];

        if cursor.read_exact(&mut buf).is_err() {
            panic!("invalid u16 format: buffer underflow");
        }

        u16::from_be_bytes(buf)
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    text: String,
    #[serde(default, skip_serializing_if = "is_default")]
    bold: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    italic: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    underlined: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    strikethrough: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    obfuscated: bool,
    #[serde(default = "Chat::default_font", skip_serializing_if = "is_default")]
    font: String,
    #[serde(default = "Chat::default_color", skip_serializing_if = "is_default")]
    color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_event: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_event: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<Vec<Chat>>,
}

impl Chat {
    fn default_font() -> String {
        String::from("minecraft:default")
    }

    fn default_color() -> String {
        String::from("reset")
    }

    pub fn literal(text: &str) -> Chat {
        Chat {
            text: String::from(text),
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            font: Self::default_font(),
            color: Self::default_color(),
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: None,
        }
    }
}
