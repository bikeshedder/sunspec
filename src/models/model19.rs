//! PPP Link
/// PPP Link
///
/// Include this model to configure a Point-to-Point Protocol link
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model19 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// Rate
    ///
    /// Interface baud rate in bits per second
    pub rte: u32,
    /// Bits
    ///
    /// Number of data bits per character
    pub bits: u16,
    /// Parity
    ///
    /// Bitmask value.  Parity setting
    pub pty: Pty,
    /// Duplex
    ///
    /// Enumerated value.  Duplex mode
    pub dup: Option<Dup>,
    /// Flow Control
    ///
    /// Flow Control Method
    pub flw: Option<Flw>,
    /// Authentication
    ///
    /// Enumerated value.  Authentication method
    pub auth: Option<Auth>,
    /// Username
    ///
    /// Username for authentication
    pub usr_nam: Option<String>,
    /// Password
    ///
    /// Password for authentication
    pub pw: Option<String>,
}
#[allow(missing_docs)]
impl Model19 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const RTE: crate::Point<Self, u32> = crate::Point::new(4, 2, true);
    pub const BITS: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const PTY: crate::Point<Self, Pty> = crate::Point::new(7, 1, true);
    pub const DUP: crate::Point<Self, Option<Dup>> = crate::Point::new(8, 1, true);
    pub const FLW: crate::Point<Self, Option<Flw>> = crate::Point::new(9, 1, true);
    pub const AUTH: crate::Point<Self, Option<Auth>> = crate::Point::new(10, 1, false);
    pub const USR_NAM: crate::Point<Self, Option<String>> = crate::Point::new(11, 12, false);
    pub const PW: crate::Point<Self, Option<String>> = crate::Point::new(23, 6, false);
}
impl crate::Model for Model19 {
    const ID: u16 = 19;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            rte: Self::RTE.from_data(data)?,
            bits: Self::BITS.from_data(data)?,
            pty: Self::PTY.from_data(data)?,
            dup: Self::DUP.from_data(data)?,
            flw: Self::FLW.from_data(data)?,
            auth: Self::AUTH.from_data(data)?,
            usr_nam: Self::USR_NAM.from_data(data)?,
            pw: Self::PW.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m19
    }
}
/// Parity
///
/// Bitmask value.  Parity setting
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Pty {
    #[allow(missing_docs)]
    None = 0,
    #[allow(missing_docs)]
    Odd = 1,
    #[allow(missing_docs)]
    Even = 2,
}
impl crate::Value for Pty {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Pty> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Pty::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
/// Duplex
///
/// Enumerated value.  Duplex mode
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Dup {
    #[allow(missing_docs)]
    Full = 0,
    #[allow(missing_docs)]
    Half = 1,
}
impl crate::Value for Dup {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Dup> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Dup::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
/// Flow Control
///
/// Flow Control Method
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Flw {
    #[allow(missing_docs)]
    None = 0,
    #[allow(missing_docs)]
    Hw = 1,
    #[allow(missing_docs)]
    Xonxoff = 2,
}
impl crate::Value for Flw {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Flw> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Flw::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
/// Authentication
///
/// Enumerated value.  Authentication method
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Auth {
    #[allow(missing_docs)]
    None = 0,
    #[allow(missing_docs)]
    Pap = 1,
    #[allow(missing_docs)]
    Chap = 2,
}
impl crate::Value for Auth {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Auth> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Auth::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
