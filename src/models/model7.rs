//! Secure Write Response Model (DRAFT 1)
/// Secure Write Response Model (DRAFT 1)
///
/// Include a digital signature over the response
///
/// Notes: Used in conjunction with a Secure Write Request
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model7 {
    /// Request Sequence
    ///
    /// Sequence number from the request
    pub rq_seq: u16,
    /// Status
    ///
    /// Status of last write operation
    pub sts: Sts,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Notes: Shall be advanced for each response
    pub seq: u16,
    /// Alarm
    ///
    /// Bitmask alarm code
    pub alm: Alm,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Notes: For future proof
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}
#[allow(missing_docs)]
impl Model7 {
    pub const RQ_SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STS: crate::PointDef<Self, Sts> = crate::PointDef::new(1, 1, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const ALM: crate::PointDef<Self, Alm> = crate::PointDef::new(6, 1, false);
    pub const ALG: crate::PointDef<Self, Alg> = crate::PointDef::new(8, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
}
impl crate::Model for Model7 {
    const ID: u16 = 7;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            rq_seq: Self::RQ_SEQ.from_data(data)?,
            sts: Self::STS.from_data(data)?,
            ts: Self::TS.from_data(data)?,
            ms: Self::MS.from_data(data)?,
            seq: Self::SEQ.from_data(data)?,
            alm: Self::ALM.from_data(data)?,
            alg: Self::ALG.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m7
    }
}
/// Status
///
/// Status of last write operation
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Sts {
    #[allow(missing_docs)]
    Success = 0,
    /// Notes: The signature was not valid
    Ds = 1,
    /// Notes: One or more registers were not writable by this role
    Acl = 2,
    /// Notes: Offset out of range or missing from multi-register value
    Off = 3,
    /// Notes: Value is out of acceptable range
    Val = 4,
}
impl crate::Value for Sts {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Sts> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Sts::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Alarm
///
/// Bitmask alarm code
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Alm {
    #[allow(missing_docs)]
    None = 0,
    /// Notes: Tampered
    Alm = 1,
}
impl crate::Value for Alm {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Alm> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Alm::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Notes: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Alg {
    /// Notes: For test purposes only
    None = 0,
    #[allow(missing_docs)]
    AesGmac64 = 1,
    #[allow(missing_docs)]
    Ecc256 = 2,
}
impl crate::Value for Alg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Alg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Alg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
