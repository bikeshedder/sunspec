//! Secure Dataset Read Response
/// Secure Dataset Read Response
///
/// Compute a digital signature over a specified set of data registers
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model4 {
    /// Request Sequence
    ///
    /// Sequence number from the request
    pub rq_seq: u16,
    /// Status
    ///
    /// Status of last read operation
    pub sts: Sts,
    /// X
    ///
    /// Number of values from the request
    ///
    /// Detail: A max of 50 values are allocated
    pub x: u16,
    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Detail: Unused values shall return 0xFFFF (unimplemented)
    pub val1: u16,
    #[allow(missing_docs)]
    pub val2: u16,
    #[allow(missing_docs)]
    pub val3: u16,
    #[allow(missing_docs)]
    pub val4: u16,
    #[allow(missing_docs)]
    pub val5: u16,
    #[allow(missing_docs)]
    pub val6: u16,
    #[allow(missing_docs)]
    pub val7: u16,
    #[allow(missing_docs)]
    pub val8: u16,
    #[allow(missing_docs)]
    pub val9: u16,
    #[allow(missing_docs)]
    pub val10: u16,
    #[allow(missing_docs)]
    pub val11: u16,
    #[allow(missing_docs)]
    pub val12: u16,
    #[allow(missing_docs)]
    pub val13: u16,
    #[allow(missing_docs)]
    pub val14: u16,
    #[allow(missing_docs)]
    pub val15: u16,
    #[allow(missing_docs)]
    pub val16: u16,
    #[allow(missing_docs)]
    pub val17: u16,
    #[allow(missing_docs)]
    pub val18: u16,
    #[allow(missing_docs)]
    pub val19: u16,
    #[allow(missing_docs)]
    pub val20: u16,
    #[allow(missing_docs)]
    pub val21: u16,
    #[allow(missing_docs)]
    pub val22: u16,
    #[allow(missing_docs)]
    pub val23: u16,
    #[allow(missing_docs)]
    pub val24: u16,
    #[allow(missing_docs)]
    pub val25: u16,
    #[allow(missing_docs)]
    pub val26: u16,
    #[allow(missing_docs)]
    pub val27: u16,
    #[allow(missing_docs)]
    pub val28: u16,
    #[allow(missing_docs)]
    pub val29: u16,
    #[allow(missing_docs)]
    pub val30: u16,
    #[allow(missing_docs)]
    pub val31: u16,
    #[allow(missing_docs)]
    pub val32: u16,
    #[allow(missing_docs)]
    pub val33: u16,
    #[allow(missing_docs)]
    pub val34: u16,
    #[allow(missing_docs)]
    pub val35: u16,
    #[allow(missing_docs)]
    pub val36: u16,
    #[allow(missing_docs)]
    pub val37: u16,
    #[allow(missing_docs)]
    pub val38: u16,
    #[allow(missing_docs)]
    pub val39: u16,
    #[allow(missing_docs)]
    pub val40: u16,
    #[allow(missing_docs)]
    pub val41: u16,
    #[allow(missing_docs)]
    pub val42: u16,
    #[allow(missing_docs)]
    pub val43: u16,
    #[allow(missing_docs)]
    pub val44: u16,
    #[allow(missing_docs)]
    pub val45: u16,
    #[allow(missing_docs)]
    pub val46: u16,
    #[allow(missing_docs)]
    pub val47: u16,
    #[allow(missing_docs)]
    pub val48: u16,
    #[allow(missing_docs)]
    pub val49: u16,
    #[allow(missing_docs)]
    pub val50: u16,
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
impl Model4 {
    pub const RQ_SEQ: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const STS: crate::Point<Self, Sts> = crate::Point::new(1, 1, false);
    pub const X: crate::Point<Self, u16> = crate::Point::new(2, 1, false);
    pub const VAL1: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const VAL2: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const VAL3: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const VAL4: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const VAL5: crate::Point<Self, u16> = crate::Point::new(7, 1, false);
    pub const VAL6: crate::Point<Self, u16> = crate::Point::new(8, 1, false);
    pub const VAL7: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const VAL8: crate::Point<Self, u16> = crate::Point::new(10, 1, false);
    pub const VAL9: crate::Point<Self, u16> = crate::Point::new(11, 1, false);
    pub const VAL10: crate::Point<Self, u16> = crate::Point::new(12, 1, false);
    pub const VAL11: crate::Point<Self, u16> = crate::Point::new(13, 1, false);
    pub const VAL12: crate::Point<Self, u16> = crate::Point::new(14, 1, false);
    pub const VAL13: crate::Point<Self, u16> = crate::Point::new(15, 1, false);
    pub const VAL14: crate::Point<Self, u16> = crate::Point::new(16, 1, false);
    pub const VAL15: crate::Point<Self, u16> = crate::Point::new(17, 1, false);
    pub const VAL16: crate::Point<Self, u16> = crate::Point::new(18, 1, false);
    pub const VAL17: crate::Point<Self, u16> = crate::Point::new(19, 1, false);
    pub const VAL18: crate::Point<Self, u16> = crate::Point::new(20, 1, false);
    pub const VAL19: crate::Point<Self, u16> = crate::Point::new(21, 1, false);
    pub const VAL20: crate::Point<Self, u16> = crate::Point::new(22, 1, false);
    pub const VAL21: crate::Point<Self, u16> = crate::Point::new(23, 1, false);
    pub const VAL22: crate::Point<Self, u16> = crate::Point::new(24, 1, false);
    pub const VAL23: crate::Point<Self, u16> = crate::Point::new(25, 1, false);
    pub const VAL24: crate::Point<Self, u16> = crate::Point::new(26, 1, false);
    pub const VAL25: crate::Point<Self, u16> = crate::Point::new(27, 1, false);
    pub const VAL26: crate::Point<Self, u16> = crate::Point::new(28, 1, false);
    pub const VAL27: crate::Point<Self, u16> = crate::Point::new(29, 1, false);
    pub const VAL28: crate::Point<Self, u16> = crate::Point::new(30, 1, false);
    pub const VAL29: crate::Point<Self, u16> = crate::Point::new(31, 1, false);
    pub const VAL30: crate::Point<Self, u16> = crate::Point::new(32, 1, false);
    pub const VAL31: crate::Point<Self, u16> = crate::Point::new(33, 1, false);
    pub const VAL32: crate::Point<Self, u16> = crate::Point::new(34, 1, false);
    pub const VAL33: crate::Point<Self, u16> = crate::Point::new(35, 1, false);
    pub const VAL34: crate::Point<Self, u16> = crate::Point::new(36, 1, false);
    pub const VAL35: crate::Point<Self, u16> = crate::Point::new(37, 1, false);
    pub const VAL36: crate::Point<Self, u16> = crate::Point::new(38, 1, false);
    pub const VAL37: crate::Point<Self, u16> = crate::Point::new(39, 1, false);
    pub const VAL38: crate::Point<Self, u16> = crate::Point::new(40, 1, false);
    pub const VAL39: crate::Point<Self, u16> = crate::Point::new(41, 1, false);
    pub const VAL40: crate::Point<Self, u16> = crate::Point::new(42, 1, false);
    pub const VAL41: crate::Point<Self, u16> = crate::Point::new(43, 1, false);
    pub const VAL42: crate::Point<Self, u16> = crate::Point::new(44, 1, false);
    pub const VAL43: crate::Point<Self, u16> = crate::Point::new(45, 1, false);
    pub const VAL44: crate::Point<Self, u16> = crate::Point::new(46, 1, false);
    pub const VAL45: crate::Point<Self, u16> = crate::Point::new(47, 1, false);
    pub const VAL46: crate::Point<Self, u16> = crate::Point::new(48, 1, false);
    pub const VAL47: crate::Point<Self, u16> = crate::Point::new(49, 1, false);
    pub const VAL48: crate::Point<Self, u16> = crate::Point::new(50, 1, false);
    pub const VAL49: crate::Point<Self, u16> = crate::Point::new(51, 1, false);
    pub const VAL50: crate::Point<Self, u16> = crate::Point::new(52, 1, false);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(53, 2, false);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(55, 1, false);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(56, 1, false);
    pub const ALM: crate::Point<Self, Alm> = crate::Point::new(57, 1, false);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(58, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(59, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::RQ_SEQ.is_invalid(&self.rq_seq)
            || Self::STS.is_invalid(&self.sts)
            || Self::X.is_invalid(&self.x)
            || Self::VAL1.is_invalid(&self.val1)
            || Self::VAL2.is_invalid(&self.val2)
            || Self::VAL3.is_invalid(&self.val3)
            || Self::VAL4.is_invalid(&self.val4)
            || Self::VAL5.is_invalid(&self.val5)
            || Self::VAL6.is_invalid(&self.val6)
            || Self::VAL7.is_invalid(&self.val7)
            || Self::VAL8.is_invalid(&self.val8)
            || Self::VAL9.is_invalid(&self.val9)
            || Self::VAL10.is_invalid(&self.val10)
            || Self::VAL11.is_invalid(&self.val11)
            || Self::VAL12.is_invalid(&self.val12)
            || Self::VAL13.is_invalid(&self.val13)
            || Self::VAL14.is_invalid(&self.val14)
            || Self::VAL15.is_invalid(&self.val15)
            || Self::VAL16.is_invalid(&self.val16)
            || Self::VAL17.is_invalid(&self.val17)
            || Self::VAL18.is_invalid(&self.val18)
            || Self::VAL19.is_invalid(&self.val19)
            || Self::VAL20.is_invalid(&self.val20)
            || Self::VAL21.is_invalid(&self.val21)
            || Self::VAL22.is_invalid(&self.val22)
            || Self::VAL23.is_invalid(&self.val23)
            || Self::VAL24.is_invalid(&self.val24)
            || Self::VAL25.is_invalid(&self.val25)
            || Self::VAL26.is_invalid(&self.val26)
            || Self::VAL27.is_invalid(&self.val27)
            || Self::VAL28.is_invalid(&self.val28)
            || Self::VAL29.is_invalid(&self.val29)
            || Self::VAL30.is_invalid(&self.val30)
            || Self::VAL31.is_invalid(&self.val31)
            || Self::VAL32.is_invalid(&self.val32)
            || Self::VAL33.is_invalid(&self.val33)
            || Self::VAL34.is_invalid(&self.val34)
            || Self::VAL35.is_invalid(&self.val35)
            || Self::VAL36.is_invalid(&self.val36)
            || Self::VAL37.is_invalid(&self.val37)
            || Self::VAL38.is_invalid(&self.val38)
            || Self::VAL39.is_invalid(&self.val39)
            || Self::VAL40.is_invalid(&self.val40)
            || Self::VAL41.is_invalid(&self.val41)
            || Self::VAL42.is_invalid(&self.val42)
            || Self::VAL43.is_invalid(&self.val43)
            || Self::VAL44.is_invalid(&self.val44)
            || Self::VAL45.is_invalid(&self.val45)
            || Self::VAL46.is_invalid(&self.val46)
            || Self::VAL47.is_invalid(&self.val47)
            || Self::VAL48.is_invalid(&self.val48)
            || Self::VAL49.is_invalid(&self.val49)
            || Self::VAL50.is_invalid(&self.val50)
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
impl crate::Group for Model4 {
    const LEN: u16 = 60;
}
impl Model4 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                rq_seq: Self::RQ_SEQ.from_data(data)?,
                sts: Self::STS.from_data(data)?,
                x: Self::X.from_data(data)?,
                val1: Self::VAL1.from_data(data)?,
                val2: Self::VAL2.from_data(data)?,
                val3: Self::VAL3.from_data(data)?,
                val4: Self::VAL4.from_data(data)?,
                val5: Self::VAL5.from_data(data)?,
                val6: Self::VAL6.from_data(data)?,
                val7: Self::VAL7.from_data(data)?,
                val8: Self::VAL8.from_data(data)?,
                val9: Self::VAL9.from_data(data)?,
                val10: Self::VAL10.from_data(data)?,
                val11: Self::VAL11.from_data(data)?,
                val12: Self::VAL12.from_data(data)?,
                val13: Self::VAL13.from_data(data)?,
                val14: Self::VAL14.from_data(data)?,
                val15: Self::VAL15.from_data(data)?,
                val16: Self::VAL16.from_data(data)?,
                val17: Self::VAL17.from_data(data)?,
                val18: Self::VAL18.from_data(data)?,
                val19: Self::VAL19.from_data(data)?,
                val20: Self::VAL20.from_data(data)?,
                val21: Self::VAL21.from_data(data)?,
                val22: Self::VAL22.from_data(data)?,
                val23: Self::VAL23.from_data(data)?,
                val24: Self::VAL24.from_data(data)?,
                val25: Self::VAL25.from_data(data)?,
                val26: Self::VAL26.from_data(data)?,
                val27: Self::VAL27.from_data(data)?,
                val28: Self::VAL28.from_data(data)?,
                val29: Self::VAL29.from_data(data)?,
                val30: Self::VAL30.from_data(data)?,
                val31: Self::VAL31.from_data(data)?,
                val32: Self::VAL32.from_data(data)?,
                val33: Self::VAL33.from_data(data)?,
                val34: Self::VAL34.from_data(data)?,
                val35: Self::VAL35.from_data(data)?,
                val36: Self::VAL36.from_data(data)?,
                val37: Self::VAL37.from_data(data)?,
                val38: Self::VAL38.from_data(data)?,
                val39: Self::VAL39.from_data(data)?,
                val40: Self::VAL40.from_data(data)?,
                val41: Self::VAL41.from_data(data)?,
                val42: Self::VAL42.from_data(data)?,
                val43: Self::VAL43.from_data(data)?,
                val44: Self::VAL44.from_data(data)?,
                val45: Self::VAL45.from_data(data)?,
                val46: Self::VAL46.from_data(data)?,
                val47: Self::VAL47.from_data(data)?,
                val48: Self::VAL48.from_data(data)?,
                val49: Self::VAL49.from_data(data)?,
                val50: Self::VAL50.from_data(data)?,
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
/// Status of last read operation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Sts {
    #[allow(missing_docs)]
    Success,
    #[allow(missing_docs)]
    Ds,
    /// Detail: One or more registers were not writable by this role
    Acl,
    /// Detail: Offset out of range or missing from multi-register value
    Off,
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
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Success => 0,
            Self::Ds => 1,
            Self::Acl => 2,
            Self::Off => 3,
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
    pub const DS: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
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
impl crate::Model for Model4 {
    const ID: u16 = 4;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m4
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
