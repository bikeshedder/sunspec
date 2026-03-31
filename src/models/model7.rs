//! Secure Write Response Model (DRAFT 1)
/// Secure Write Response Model (DRAFT 1)
///
/// Include a digital signature over the response
///
/// Detail: Used in conjunction with a Secure Write Request
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
    /// Detail: Shall be advanced for each response
    pub seq: u16,
    /// Alarm
    ///
    /// Bitmask alarm code
    pub alm: Alm,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Detail: For future proof
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Detail: The value of N must be at least 4 (64 bits)
    pub n: u16,
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Model7 {
    pub const RQ_SEQ: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const STS: crate::Point<Self, Sts> = crate::Point::new(1, 1, false);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(2, 2, false);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const ALM: crate::Point<Self, Alm> = crate::Point::new(6, 1, false);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(8, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(9, 1, true);
    fn has_invalid_points(&self) -> bool {
        Self::RQ_SEQ.is_invalid(&self.rq_seq)
            || Self::STS.is_invalid(&self.sts)
            || Self::TS.is_invalid(&self.ts)
            || Self::MS.is_invalid(&self.ms)
            || Self::SEQ.is_invalid(&self.seq)
            || Self::ALM.is_invalid(&self.alm)
            || Self::ALG.is_invalid(&self.alg)
            || Self::N.is_invalid(&self.n)
            || self
                .repeating
                .iter()
                .any(|group| group.has_invalid_points())
    }
}
impl crate::Group for Model7 {
    const LEN: u16 = 10;
}
impl Model7 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                rq_seq: Self::RQ_SEQ.from_data(data)?,
                sts: Self::STS.from_data(data)?,
                ts: Self::TS.from_data(data)?,
                ms: Self::MS.from_data(data)?,
                seq: Self::SEQ.from_data(data)?,
                alm: Self::ALM.from_data(data)?,
                alg: Self::ALG.from_data(data)?,
                n: Self::N.from_data(data)?,
                repeating,
            },
        ))
    }
}
/// Status
///
/// Status of last write operation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Sts {
    #[allow(missing_docs)]
    Success,
    /// Detail: The signature was not valid
    Ds,
    /// Detail: One or more registers were not writable by this role
    Acl,
    /// Detail: Offset out of range or missing from multi-register value
    Off,
    /// Detail: Value is out of acceptable range
    Val,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Sts {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::Ds,
            2 => Self::Acl,
            3 => Self::Off,
            4 => Self::Val,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Success => 0,
            Self::Ds => 1,
            Self::Acl => 2,
            Self::Off => 3,
            Self::Val => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Sts {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Alarm
///
/// Bitmask alarm code
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Alm {
    #[allow(missing_docs)]
    None,
    /// Detail: Tampered
    Alm,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Alm {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Alm,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Alm => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Alm {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Detail: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Alg {
    /// Detail: For test purposes only
    None,
    #[allow(missing_docs)]
    AesGmac64,
    #[allow(missing_docs)]
    Ecc256,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Alg {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::AesGmac64,
            2 => Self::Ecc256,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::AesGmac64 => 1,
            Self::Ecc256 => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Alg {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    /// DS
    ///
    /// Digital Signature
    pub ds: u16,
}
#[allow(missing_docs)]
impl Repeating {
    pub const DS: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    fn has_invalid_points(&self) -> bool {
        Self::DS.is_invalid(&self.ds)
    }
}
impl crate::Group for Repeating {
    const LEN: u16 = 1;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                ds: Self::DS.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Model7 {
    const ID: u16 = 7;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m7
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        if model.has_invalid_points() {
            Err(crate::ParseError::InvalidPointData(
                crate::InvalidPointData { model },
            ))
        } else {
            Ok(model)
        }
    }
}
