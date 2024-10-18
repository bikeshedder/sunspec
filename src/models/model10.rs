//! Communication Interface Header
/// Communication Interface Header
///
/// To be included first for a complete interface description
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model10 {
    /// Interface Status
    ///
    /// Overall interface status
    pub st: St,
    /// Interface Control
    ///
    /// Overall interface control (TBD)
    pub ctl: Option<u16>,
    /// Physical Access Type
    ///
    /// Enumerated value.  Type of physical media
    pub typ: Option<Typ>,
}
#[allow(missing_docs)]
impl Model10 {
    pub const ST: crate::Point<Self, St> = crate::Point::new(0, 1, false);
    pub const CTL: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const TYP: crate::Point<Self, Option<Typ>> = crate::Point::new(2, 1, false);
}
impl crate::Model for Model10 {
    const ID: u16 = 10;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            st: Self::ST.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m10
    }
}
/// Interface Status
///
/// Overall interface status
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum St {
    #[allow(missing_docs)]
    Down = 0,
    #[allow(missing_docs)]
    Up = 1,
    #[allow(missing_docs)]
    Fault = 2,
}
impl crate::Value for St {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<St> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                St::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Physical Access Type
///
/// Enumerated value.  Type of physical media
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Typ {
    #[allow(missing_docs)]
    Unknown = 0,
    #[allow(missing_docs)]
    Internal = 1,
    #[allow(missing_docs)]
    TwistedPair = 2,
    #[allow(missing_docs)]
    Fiber = 3,
    #[allow(missing_docs)]
    Wireless = 4,
}
impl crate::Value for Typ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Typ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Typ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
