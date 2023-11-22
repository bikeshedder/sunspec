//! Communication Interface Header

/// Communication Interface Header
///
/// To be included first for a complete interface description
#[derive(Debug)]
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
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(0, 1, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const TYP: crate::PointDef<Self, Option<Typ>> = crate::PointDef::new(2, 1, false);
}

impl crate::Model for Model10 {
    const ID: u16 = 10;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            st: Self::ST.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
        })
    }
}

#[doc = "Interface Status\n\nOverall interface status"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum St {
    #[doc = ""]
    Down = 0,
    #[doc = ""]
    Up = 1,
    #[doc = ""]
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

#[doc = "Physical Access Type\n\nEnumerated value.  Type of physical media"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum Typ {
    #[doc = ""]
    Unknown = 0,
    #[doc = ""]
    Internal = 1,
    #[doc = ""]
    TwistedPair = 2,
    #[doc = ""]
    Fiber = 3,
    #[doc = ""]
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
