pub mod data {
    use std::io::{Cursor, Read};

    pub trait FromBytes {
        fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Self;
    }

    impl FromBytes for i32 {
        fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Self {
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

    impl FromBytes for String {
        fn from_bytes(cursor: &mut Cursor<&[u8]>) -> String {
            let size: usize = i32::from_bytes(cursor) as usize;
            let mut buf = vec![0; size];

            if cursor.read_exact(&mut buf).is_err() {
                panic!("invalid string format: buffer underflow");
            }

            String::from_utf8_lossy(buf.as_slice()).into_owned()
        }
    }

    impl FromBytes for u16 {
        fn from_bytes(cursor: &mut Cursor<&[u8]>) -> u16 {
            let mut buf = [0; 2];

            if cursor.read_exact(&mut buf).is_err() {
                panic!("invalid u16 format: buffer underflow");
            }

            u16::from_be_bytes(buf)
        }
    }
}
