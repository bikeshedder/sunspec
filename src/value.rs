use std::{
    net::{Ipv4Addr, Ipv6Addr},
    string::FromUtf8Error,
};

use thiserror::Error;

/// This trait contains all the conversion methods needed for
/// working with points of the SunSpec models.
pub trait Value: Sized {
    // FIXME we currently don't distinguish between decode errors
    // and not-set values. SunSpec does define special values for
    // unsupported/unset data. To do this 100% correct we would need to
    // wrap all primitives into some kind of value container type
    // that disallows the use of those special values.
    /// Decode value from a given slice of u16
    fn decode(data: &[u16]) -> Result<Self, DecodeError>;
    /// Encode value into a u16 array
    fn encode(self) -> Box<[u16]>;
}

/// This trait marks points with a fixed size. All non-string
/// values are actually fixed size.
pub trait FixedSize: Value + PartialEq {
    /// The size of this value
    const SIZE: u16;
    /// The value when the point is not supported
    const INVALID: Self;
}

impl Value for u16 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[w0] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok(w0)
    }
    fn encode(self) -> Box<[u16]> {
        Box::new([self])
    }
}

impl FixedSize for u16 {
    const SIZE: u16 = 1;
    const INVALID: u16 = u16::MAX;
}

impl Value for u32 {
    fn decode(words: &[u16]) -> Result<Self, DecodeError> {
        let &[w1, w0] = words else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok((w1 as u32) << 16 | w0 as u32)
    }
    fn encode(self) -> Box<[u16]> {
        Box::new([(self >> 16) as u16, self as u16])
    }
}

impl FixedSize for u32 {
    const SIZE: u16 = 2;
    const INVALID: u32 = u32::MAX;
}

impl Value for u64 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[w3, w2, w1, w0] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok((w3 as u64) << 0x30 | (w2 as u64) << 0x20 | (w1 as u64) << 0x10 | w0 as u64)
    }
    fn encode(self) -> Box<[u16]> {
        Box::new([
            (self >> 0x30) as u16,
            (self >> 0x20) as u16,
            (self >> 0x10) as u16,
            self as u16,
        ])
    }
}

impl FixedSize for u64 {
    const SIZE: u16 = 4;
    const INVALID: u64 = u64::MAX;
}

impl Value for u128 {
    fn decode(words: &[u16]) -> Result<Self, DecodeError> {
        let &[w7, w6, w5, w4, w3, w2, w1, w0] = words else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok((w7 as u128) << 0x70
            | (w6 as u128) << 0x60
            | (w5 as u128) << 0x50
            | (w4 as u128) << 0x40
            | (w3 as u128) << 0x30
            | (w2 as u128) << 0x20
            | (w1 as u128) << 0x10
            | (w0 as u128))
    }
    fn encode(self) -> Box<[u16]> {
        Box::new([
            (self >> 0x70) as u16,
            (self >> 0x60) as u16,
            (self >> 0x50) as u16,
            (self >> 0x40) as u16,
            (self >> 0x30) as u16,
            (self >> 0x20) as u16,
            (self >> 0x10) as u16,
            self as u16,
        ])
    }
}

impl FixedSize for u128 {
    const SIZE: u16 = 8;
    const INVALID: u128 = u128::MAX;
}

impl Value for i16 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        Ok(u16::decode(data)? as Self)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}

impl FixedSize for i16 {
    const SIZE: u16 = 1;
    const INVALID: i16 = i16::MIN;
}

impl Value for i32 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        Ok(u32::decode(data)? as Self)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u32).encode()
    }
}

impl FixedSize for i32 {
    const SIZE: u16 = 2;
    const INVALID: i32 = i32::MIN;
}

impl Value for i64 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        Ok(u64::decode(data)? as Self)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u64).encode()
    }
}

impl FixedSize for i64 {
    const SIZE: u16 = 4;
    const INVALID: i64 = i64::MIN;
}

impl Value for f32 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[w1, w0] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok(f32::from_be_bytes([
            (w1 >> 8) as u8,
            w1 as u8,
            (w0 >> 8) as u8,
            w0 as u8,
        ]))
    }
    fn encode(self) -> Box<[u16]> {
        let [b3, b2, b1, b0] = self.to_be_bytes();
        Box::new([
            (b3 as u16) << 8 | (b2 as u16),
            (b1 as u16) << 8 | (b0 as u16),
        ])
    }
}

impl FixedSize for f32 {
    const SIZE: u16 = 2;
    const INVALID: f32 = f32::NAN;
}

impl Value for f64 {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[w3, w2, w1, w0] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok(f64::from_be_bytes([
            (w3 >> 8) as u8,
            w3 as u8,
            (w2 >> 8) as u8,
            w2 as u8,
            (w1 >> 8) as u8,
            w1 as u8,
            (w0 >> 8) as u8,
            w0 as u8,
        ]))
    }
    fn encode(self) -> Box<[u16]> {
        let [b7, b6, b5, b4, b3, b2, b1, b0] = self.to_be_bytes();
        Box::new([
            (b7 as u16) << 8 | (b6 as u16),
            (b5 as u16) << 8 | (b4 as u16),
            (b3 as u16) << 8 | (b2 as u16),
            (b1 as u16) << 8 | (b0 as u16),
        ])
    }
}

impl FixedSize for f64 {
    const SIZE: u16 = 4;
    const INVALID: f64 = f64::NAN;
}

fn encode_bytes(octets: &[u8]) -> Box<[u16]> {
    octets
        .chunks(2)
        .map(|chunk| match *chunk {
            [b1, b0] => (b1 as u16) << 8 | (b0 as u16),
            [b1] => (b1 as u16) << 8,
            _ => unreachable!(),
        })
        .collect()
}

impl Value for String {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let bytes = data
            .iter()
            .flat_map(|word| word.to_be_bytes())
            .take_while(|&b| b != 0)
            .collect::<Vec<_>>();
        Ok(String::from_utf8(bytes)?)
    }
    fn encode(self) -> Box<[u16]> {
        encode_bytes(self.as_bytes())
    }
}

impl Value for Ipv4Addr {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[w1, w0] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok(Ipv4Addr::new(
            (w1 >> 8) as u8,
            w1 as u8,
            (w0 >> 8) as u8,
            w0 as u8,
        ))
    }
    fn encode(self) -> Box<[u16]> {
        encode_bytes(&self.octets())
    }
}

impl FixedSize for Ipv4Addr {
    const SIZE: u16 = 2;
    const INVALID: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
}

impl Value for Ipv6Addr {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let &[a, b, c, d, e, f, g, h] = data else {
            return Err(DecodeError::OutOfBounds);
        };
        Ok(Ipv6Addr::new(a, b, c, d, e, f, g, h))
    }
    fn encode(self) -> Box<[u16]> {
        encode_bytes(&self.octets())
    }
}

impl FixedSize for Ipv6Addr {
    const SIZE: u16 = 8;
    const INVALID: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
}

impl Value for Option<String> {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        Ok(if data.iter().all(|w| *w == 0) {
            None
        } else {
            Some(String::decode(data)?)
        })
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            if value.is_empty() {
                Box::new([0x0080])
            } else {
                String::encode(value)
            }
        } else {
            Box::new([0x0000])
        }
    }
}

impl<T: FixedSize> Value for Option<T> {
    fn decode(data: &[u16]) -> Result<Self, DecodeError> {
        let value = T::decode(data)?;
        Ok((value != T::INVALID).then_some(value))
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            T::INVALID.encode()
        }
    }
}

/// This error is returned if there was an error decoding
/// the value of a given point.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum DecodeError {
    /// The value could not be decoded because the given data was not large
    /// enough.
    #[error("Out of bounds")]
    OutOfBounds,
    /// The given data was not valid UTF-8
    #[error("Invalid UTF-8 data")]
    Utf8(#[from] FromUtf8Error),
}

#[test]
fn test_u16() {
    assert_eq!(*0x0001i16.encode(), [0x1]);
    assert_eq!(u16::decode(&[0x1]), Ok(0x0001u16));
}

#[test]
fn test_u32() {
    assert_eq!(*0x00010002u32.encode(), [0x1, 0x2]);
    assert_eq!(u32::decode(&[0x1, 0x2]), Ok(0x00010002u32));
}

#[test]
fn test_u64() {
    assert_eq!(*0x0001000200030004u64.encode(), [0x1, 0x2, 0x3, 0x4]);
    assert_eq!(
        u64::decode(&[0x1, 0x2, 0x3, 0x4]),
        Ok(0x0001000200030004u64)
    );
}

#[test]
fn test_u128() {
    assert_eq!(
        *0x00010002000300040005000600070008u128.encode(),
        [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]
    );
    assert_eq!(
        u128::decode(&[0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]),
        Ok(0x00010002000300040005000600070008u128)
    );
}

#[test]
fn test_i16() {
    assert_eq!(*(-1i16).encode(), [0xffff]);
    assert_eq!(i16::decode(&[0xffff]), Ok(-1i16));
}

#[test]
fn test_i32() {
    assert_eq!(*(-1i32).encode(), [0xffff, 0xffff]);
    assert_eq!(i32::decode(&[0xffff, 0xffff]), Ok(-1i32));
}

#[test]
fn test_i64() {
    assert_eq!(*(-1i64).encode(), [0xffff, 0xffff, 0xffff, 0xffff]);
    assert_eq!(i64::decode(&[0xffff, 0xffff, 0xffff, 0xffff]), Ok(-1i64));
}

#[test]
fn test_f32() {
    assert_eq!(*(0.5f32).encode(), [0x3f00, 0x0000]);
    assert_eq!(f32::decode(&[0x3f00, 0x0000]), Ok(0.5f32));
}

#[test]
fn test_f64() {
    assert_eq!(*(0.5f64).encode(), [0x3fe0, 0x0000, 0x0000, 0x0000]);
    assert_eq!(f64::decode(&[0x3fe0, 0x0000, 0x0000, 0x0000]), Ok(0.5f64));
}

#[test]
fn test_string() {
    assert_eq!(*String::from("SunS").encode(), [0x5375, 0x6e53]);
    assert_eq!(String::decode(&[0x5375, 0x6e53]), Ok(String::from("SunS")));
    assert_eq!(*String::from("pad").encode(), [0x7061, 0x6400]);
    assert_eq!(String::decode(&[0x7061, 0x6400]), Ok(String::from("pad")));
    assert_eq!(
        String::decode(&[0x7061, 0x6400, 0x0000]),
        Ok(String::from("pad"))
    );
}
