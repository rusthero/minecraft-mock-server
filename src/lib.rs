// TODO Create structs for types
pub mod data {
    use std::string::FromUtf8Error;

    pub fn read_var_int(buf: &mut Vec<u8>) -> i32 {
        let mut result: i32 = 0;
        let mut shift: u32 = 0;
        let mut index: usize = 0;

        while let Some(byte) = buf.get(index).copied() {
            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;
            index += 1;

            if byte & 0x80 == 0 {
                break;
            }

            if shift >= 32 {
                panic!("invalid varint format");
            }
        }

        buf.drain(..index);
        result
    }

    pub fn read_string(buf: &mut Vec<u8>) -> String {
        let size: usize = read_var_int(buf) as usize;
        //let result = String::from_utf8(buf[..size].to_owned());
        let result = String::from_utf8_lossy(&buf[..size]).into_owned();

        buf.drain(..size);
        result
    }

    pub fn read_u16(buf: &mut Vec<u8>) -> u16 {
        let result = u16::from_be_bytes(buf[..2].try_into().unwrap());

        buf.drain(..2);
        result
    }
}
