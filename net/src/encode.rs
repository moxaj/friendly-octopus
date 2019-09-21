use std::io::Cursor;
use std::marker::PhantomData;

use bincode;
use bytes::{Buf, BufMut, BytesMut};
use serde;
use tokio::codec::{Decoder, Encoder};

#[derive(PartialEq, Eq, Debug)]
enum DecodeState {
    Head,
    Body(u16),
}

pub struct BincodeDecoder<T> {
    decode_state: DecodeState,
    _phantom_data: PhantomData<T>,
}

impl<T> BincodeDecoder<T> {
    pub fn new() -> Self {
        BincodeDecoder {
            decode_state: DecodeState::Head,
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Decoder for BincodeDecoder<T>
    where for<'de> T: serde::Deserialize<'de> {
    type Item = T;
    type Error = bincode::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode_state {
            DecodeState::Head => {
                if src.len() < 2 {
                    Ok(None)
                } else {
                    let body_length = Cursor::new(&mut *src).get_u16_le();
                    src.advance(2);
                    src.reserve(body_length as usize);
                    self.decode_state = DecodeState::Body(body_length);
                    self.decode(src)
                }
            }
            DecodeState::Body(body_length) => {
                if src.len() < body_length as usize {
                    Ok(None)
                } else {
                    bincode::deserialize_from(Cursor::new(src.split_to(body_length as usize))).map(|value| {
                        self.decode_state = DecodeState::Head;
                        Some(value)
                    })
                }
            }
        }
    }
}

pub struct BincodeEncoder<T> {
    _phantom_data: PhantomData<T>,
}

impl<T> BincodeEncoder<T> {
    pub fn new() -> Self {
        BincodeEncoder {
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Encoder for BincodeEncoder<T>
    where T: serde::Serialize {
    type Item = T;
    type Error = bincode::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len_pre = dst.len() as u64;
        dst.put_u16_le(0);
        bincode::serialize_into(dst.writer(), &item)?;
        let len_post = dst.len() as u64;
        let mut cursor = Cursor::new(&mut *dst);
        cursor.set_position(len_pre);
        cursor.put_u16_le((len_post - len_pre - 2) as u16);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use bytes::{BufMut, BytesMut};
    use tokio::codec::{Decoder, Encoder};

    use crate::encode::{BincodeDecoder, BincodeEncoder, DecodeState};

    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
    struct Value {
        a: u16,
        b: Vec<u8>,
    }

    #[test]
    fn test_encode() {
        let mut buffer = BytesMut::new();
        let mut codec = BincodeEncoder::<Value>::new();

        assert_eq!(codec.encode(Value { a: 13, b: vec![0, 1, 2, 3, 4] }, &mut buffer).unwrap(), ());
        assert_eq!(&buffer[..], &[15, 0, 13, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4]);

        assert_eq!(codec.encode(Value { a: 10, b: vec![] }, &mut buffer).unwrap(), ());
        assert_eq!(&buffer[..], &[15, 0, 13, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_decode() {
        let mut buffer = BytesMut::new();
        let mut codec = BincodeDecoder::<Value>::new();

        buffer.extend(&[15, 0, 13, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4]);
        assert_eq!(codec.decode(&mut buffer).unwrap(), Some(Value { a: 13, b: vec![0, 1, 2, 3, 4] }), "value is correctly decoded");
        assert_eq!(codec.decode_state, DecodeState::Head, "awaiting body length");
        assert_eq!(codec.decode(&mut buffer).unwrap(), None, "no value can be decoded yet");

        buffer.put_u8(10);
        buffer.put_u8(0);
        assert_eq!(codec.decode(&mut buffer).unwrap(), None, "no value can be decoded yet");
        assert_eq!(codec.decode_state, DecodeState::Body(10), "awaiting body");

        buffer.put_u8(10);
        buffer.put_u8(0);
        buffer.put_u8(0);
        buffer.put_u8(0);
        buffer.put_u8(0);
        buffer.put_u8(0);
        assert_eq!(codec.decode(&mut buffer).unwrap(), None, "no value can be decoded yet");
        assert_eq!(codec.decode_state, DecodeState::Body(10), "awaiting body");

        buffer.put_u8(0);
        buffer.put_u8(0);
        buffer.put_u8(0);
        assert_eq!(codec.decode(&mut buffer).unwrap(), None, "no value can be decoded yet");
        assert_eq!(codec.decode_state, DecodeState::Body(10), "awaiting body");

        buffer.put_u8(0);
        assert_eq!(codec.decode(&mut buffer).unwrap(), Some(Value { a: 10, b: vec![] }), "value is correctly decoded");
        assert_eq!(codec.decode_state, DecodeState::Head, "awaiting body length");
        assert_eq!(codec.decode(&mut buffer).unwrap(), None, "no value can be decoded yet");
    }
}