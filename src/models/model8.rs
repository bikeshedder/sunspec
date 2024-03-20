//! Get Device Security Certificate

/// Get Device Security Certificate
///
/// Security model for PKI
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model8 {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    pub fmt: Fmt,
    /// N
    ///
    /// Number of registers to follow for the certificate
    pub n: u16,
}

#[allow(missing_docs)]

impl Model8 {
    pub const FMT: crate::PointDef<Self, Fmt> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
}

impl crate::Model for Model8 {
    const ID: u16 = 8;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            fmt: Self::FMT.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
}

#[doc = "Format\n\nX.509 format of the certificate. DER or PEM."]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Fmt {
    #[doc = ""]
    None = 0,
    #[doc = ""]
    X509Pem = 1,
    #[doc = ""]
    X509Der = 2,
}
impl crate::Value for Fmt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Fmt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Fmt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
