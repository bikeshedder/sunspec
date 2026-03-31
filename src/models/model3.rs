//! Secure Dataset Read Request
/// Secure Dataset Read Request
///
/// Request a digital signature over a specified set of data registers
///
/// Detail: Used in conjunction with Secure Dataset Read Response Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model3 {
    /// X
    ///
    /// Number of registers being requested
    ///
    /// Detail: A max of 50 registers are allowed
    pub x: u16,
    /// Offset1
    ///
    /// Offset of value to read
    pub off1: u16,
    #[allow(missing_docs)]
    pub off2: u16,
    #[allow(missing_docs)]
    pub off3: u16,
    #[allow(missing_docs)]
    pub off4: u16,
    #[allow(missing_docs)]
    pub off5: u16,
    #[allow(missing_docs)]
    pub off6: u16,
    #[allow(missing_docs)]
    pub off7: u16,
    #[allow(missing_docs)]
    pub off8: u16,
    #[allow(missing_docs)]
    pub off9: u16,
    #[allow(missing_docs)]
    pub off10: u16,
    #[allow(missing_docs)]
    pub off11: u16,
    #[allow(missing_docs)]
    pub off12: u16,
    #[allow(missing_docs)]
    pub off13: u16,
    #[allow(missing_docs)]
    pub off14: u16,
    #[allow(missing_docs)]
    pub off15: u16,
    #[allow(missing_docs)]
    pub off16: u16,
    #[allow(missing_docs)]
    pub off17: u16,
    #[allow(missing_docs)]
    pub off18: u16,
    #[allow(missing_docs)]
    pub off19: u16,
    #[allow(missing_docs)]
    pub off20: u16,
    #[allow(missing_docs)]
    pub off21: u16,
    #[allow(missing_docs)]
    pub off22: u16,
    #[allow(missing_docs)]
    pub off23: u16,
    #[allow(missing_docs)]
    pub off24: u16,
    #[allow(missing_docs)]
    pub off25: u16,
    #[allow(missing_docs)]
    pub off26: u16,
    #[allow(missing_docs)]
    pub off27: u16,
    #[allow(missing_docs)]
    pub off28: u16,
    #[allow(missing_docs)]
    pub off29: u16,
    #[allow(missing_docs)]
    pub off30: u16,
    #[allow(missing_docs)]
    pub off31: u16,
    #[allow(missing_docs)]
    pub off32: u16,
    #[allow(missing_docs)]
    pub off33: u16,
    #[allow(missing_docs)]
    pub off34: u16,
    #[allow(missing_docs)]
    pub off35: u16,
    #[allow(missing_docs)]
    pub off36: u16,
    #[allow(missing_docs)]
    pub off37: u16,
    #[allow(missing_docs)]
    pub off38: u16,
    #[allow(missing_docs)]
    pub off39: u16,
    #[allow(missing_docs)]
    pub off40: u16,
    #[allow(missing_docs)]
    pub off41: u16,
    #[allow(missing_docs)]
    pub off42: u16,
    #[allow(missing_docs)]
    pub off43: u16,
    #[allow(missing_docs)]
    pub off44: u16,
    #[allow(missing_docs)]
    pub off45: u16,
    #[allow(missing_docs)]
    pub off46: u16,
    #[allow(missing_docs)]
    pub off47: u16,
    #[allow(missing_docs)]
    pub off48: u16,
    #[allow(missing_docs)]
    pub off49: u16,
    #[allow(missing_docs)]
    pub off50: u16,
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
    /// Sequence number of request
    ///
    /// Detail: Shall be advanced for each request
    pub seq: u16,
    /// Role
    ///
    /// Digital Signature ID
    ///
    /// Detail: User's role id 0-5
    pub role: u16,
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
impl Model3 {
    pub const X: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const OFF1: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const OFF2: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const OFF3: crate::Point<Self, u16> = crate::Point::new(3, 1, true);
    pub const OFF4: crate::Point<Self, u16> = crate::Point::new(4, 1, true);
    pub const OFF5: crate::Point<Self, u16> = crate::Point::new(5, 1, true);
    pub const OFF6: crate::Point<Self, u16> = crate::Point::new(6, 1, true);
    pub const OFF7: crate::Point<Self, u16> = crate::Point::new(7, 1, true);
    pub const OFF8: crate::Point<Self, u16> = crate::Point::new(8, 1, true);
    pub const OFF9: crate::Point<Self, u16> = crate::Point::new(9, 1, true);
    pub const OFF10: crate::Point<Self, u16> = crate::Point::new(10, 1, true);
    pub const OFF11: crate::Point<Self, u16> = crate::Point::new(11, 1, true);
    pub const OFF12: crate::Point<Self, u16> = crate::Point::new(12, 1, true);
    pub const OFF13: crate::Point<Self, u16> = crate::Point::new(13, 1, true);
    pub const OFF14: crate::Point<Self, u16> = crate::Point::new(14, 1, true);
    pub const OFF15: crate::Point<Self, u16> = crate::Point::new(15, 1, true);
    pub const OFF16: crate::Point<Self, u16> = crate::Point::new(16, 1, true);
    pub const OFF17: crate::Point<Self, u16> = crate::Point::new(17, 1, true);
    pub const OFF18: crate::Point<Self, u16> = crate::Point::new(18, 1, true);
    pub const OFF19: crate::Point<Self, u16> = crate::Point::new(19, 1, true);
    pub const OFF20: crate::Point<Self, u16> = crate::Point::new(20, 1, true);
    pub const OFF21: crate::Point<Self, u16> = crate::Point::new(21, 1, true);
    pub const OFF22: crate::Point<Self, u16> = crate::Point::new(22, 1, true);
    pub const OFF23: crate::Point<Self, u16> = crate::Point::new(23, 1, true);
    pub const OFF24: crate::Point<Self, u16> = crate::Point::new(24, 1, true);
    pub const OFF25: crate::Point<Self, u16> = crate::Point::new(25, 1, true);
    pub const OFF26: crate::Point<Self, u16> = crate::Point::new(26, 1, true);
    pub const OFF27: crate::Point<Self, u16> = crate::Point::new(27, 1, true);
    pub const OFF28: crate::Point<Self, u16> = crate::Point::new(28, 1, true);
    pub const OFF29: crate::Point<Self, u16> = crate::Point::new(29, 1, true);
    pub const OFF30: crate::Point<Self, u16> = crate::Point::new(30, 1, true);
    pub const OFF31: crate::Point<Self, u16> = crate::Point::new(31, 1, true);
    pub const OFF32: crate::Point<Self, u16> = crate::Point::new(32, 1, true);
    pub const OFF33: crate::Point<Self, u16> = crate::Point::new(33, 1, true);
    pub const OFF34: crate::Point<Self, u16> = crate::Point::new(34, 1, true);
    pub const OFF35: crate::Point<Self, u16> = crate::Point::new(35, 1, true);
    pub const OFF36: crate::Point<Self, u16> = crate::Point::new(36, 1, true);
    pub const OFF37: crate::Point<Self, u16> = crate::Point::new(37, 1, true);
    pub const OFF38: crate::Point<Self, u16> = crate::Point::new(38, 1, true);
    pub const OFF39: crate::Point<Self, u16> = crate::Point::new(39, 1, true);
    pub const OFF40: crate::Point<Self, u16> = crate::Point::new(40, 1, true);
    pub const OFF41: crate::Point<Self, u16> = crate::Point::new(41, 1, true);
    pub const OFF42: crate::Point<Self, u16> = crate::Point::new(42, 1, true);
    pub const OFF43: crate::Point<Self, u16> = crate::Point::new(43, 1, true);
    pub const OFF44: crate::Point<Self, u16> = crate::Point::new(44, 1, true);
    pub const OFF45: crate::Point<Self, u16> = crate::Point::new(45, 1, true);
    pub const OFF46: crate::Point<Self, u16> = crate::Point::new(46, 1, true);
    pub const OFF47: crate::Point<Self, u16> = crate::Point::new(47, 1, true);
    pub const OFF48: crate::Point<Self, u16> = crate::Point::new(48, 1, true);
    pub const OFF49: crate::Point<Self, u16> = crate::Point::new(49, 1, true);
    pub const OFF50: crate::Point<Self, u16> = crate::Point::new(50, 1, true);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(51, 2, true);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(53, 1, true);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(54, 1, true);
    pub const ROLE: crate::Point<Self, u16> = crate::Point::new(55, 1, true);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(56, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(57, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::X.is_invalid(&self.x)
            || Self::OFF1.is_invalid(&self.off1)
            || Self::OFF2.is_invalid(&self.off2)
            || Self::OFF3.is_invalid(&self.off3)
            || Self::OFF4.is_invalid(&self.off4)
            || Self::OFF5.is_invalid(&self.off5)
            || Self::OFF6.is_invalid(&self.off6)
            || Self::OFF7.is_invalid(&self.off7)
            || Self::OFF8.is_invalid(&self.off8)
            || Self::OFF9.is_invalid(&self.off9)
            || Self::OFF10.is_invalid(&self.off10)
            || Self::OFF11.is_invalid(&self.off11)
            || Self::OFF12.is_invalid(&self.off12)
            || Self::OFF13.is_invalid(&self.off13)
            || Self::OFF14.is_invalid(&self.off14)
            || Self::OFF15.is_invalid(&self.off15)
            || Self::OFF16.is_invalid(&self.off16)
            || Self::OFF17.is_invalid(&self.off17)
            || Self::OFF18.is_invalid(&self.off18)
            || Self::OFF19.is_invalid(&self.off19)
            || Self::OFF20.is_invalid(&self.off20)
            || Self::OFF21.is_invalid(&self.off21)
            || Self::OFF22.is_invalid(&self.off22)
            || Self::OFF23.is_invalid(&self.off23)
            || Self::OFF24.is_invalid(&self.off24)
            || Self::OFF25.is_invalid(&self.off25)
            || Self::OFF26.is_invalid(&self.off26)
            || Self::OFF27.is_invalid(&self.off27)
            || Self::OFF28.is_invalid(&self.off28)
            || Self::OFF29.is_invalid(&self.off29)
            || Self::OFF30.is_invalid(&self.off30)
            || Self::OFF31.is_invalid(&self.off31)
            || Self::OFF32.is_invalid(&self.off32)
            || Self::OFF33.is_invalid(&self.off33)
            || Self::OFF34.is_invalid(&self.off34)
            || Self::OFF35.is_invalid(&self.off35)
            || Self::OFF36.is_invalid(&self.off36)
            || Self::OFF37.is_invalid(&self.off37)
            || Self::OFF38.is_invalid(&self.off38)
            || Self::OFF39.is_invalid(&self.off39)
            || Self::OFF40.is_invalid(&self.off40)
            || Self::OFF41.is_invalid(&self.off41)
            || Self::OFF42.is_invalid(&self.off42)
            || Self::OFF43.is_invalid(&self.off43)
            || Self::OFF44.is_invalid(&self.off44)
            || Self::OFF45.is_invalid(&self.off45)
            || Self::OFF46.is_invalid(&self.off46)
            || Self::OFF47.is_invalid(&self.off47)
            || Self::OFF48.is_invalid(&self.off48)
            || Self::OFF49.is_invalid(&self.off49)
            || Self::OFF50.is_invalid(&self.off50)
            || Self::TS.is_invalid(&self.ts)
            || Self::MS.is_invalid(&self.ms)
            || Self::SEQ.is_invalid(&self.seq)
            || Self::ROLE.is_invalid(&self.role)
            || Self::ALG.is_invalid(&self.alg)
            || Self::N.is_invalid(&self.n)
            || self
                .repeating
                .iter()
                .any(|group| group.has_invalid_points())
    }
}
impl crate::Group for Model3 {
    const LEN: u16 = 58;
}
impl Model3 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                x: Self::X.from_data(data)?,
                off1: Self::OFF1.from_data(data)?,
                off2: Self::OFF2.from_data(data)?,
                off3: Self::OFF3.from_data(data)?,
                off4: Self::OFF4.from_data(data)?,
                off5: Self::OFF5.from_data(data)?,
                off6: Self::OFF6.from_data(data)?,
                off7: Self::OFF7.from_data(data)?,
                off8: Self::OFF8.from_data(data)?,
                off9: Self::OFF9.from_data(data)?,
                off10: Self::OFF10.from_data(data)?,
                off11: Self::OFF11.from_data(data)?,
                off12: Self::OFF12.from_data(data)?,
                off13: Self::OFF13.from_data(data)?,
                off14: Self::OFF14.from_data(data)?,
                off15: Self::OFF15.from_data(data)?,
                off16: Self::OFF16.from_data(data)?,
                off17: Self::OFF17.from_data(data)?,
                off18: Self::OFF18.from_data(data)?,
                off19: Self::OFF19.from_data(data)?,
                off20: Self::OFF20.from_data(data)?,
                off21: Self::OFF21.from_data(data)?,
                off22: Self::OFF22.from_data(data)?,
                off23: Self::OFF23.from_data(data)?,
                off24: Self::OFF24.from_data(data)?,
                off25: Self::OFF25.from_data(data)?,
                off26: Self::OFF26.from_data(data)?,
                off27: Self::OFF27.from_data(data)?,
                off28: Self::OFF28.from_data(data)?,
                off29: Self::OFF29.from_data(data)?,
                off30: Self::OFF30.from_data(data)?,
                off31: Self::OFF31.from_data(data)?,
                off32: Self::OFF32.from_data(data)?,
                off33: Self::OFF33.from_data(data)?,
                off34: Self::OFF34.from_data(data)?,
                off35: Self::OFF35.from_data(data)?,
                off36: Self::OFF36.from_data(data)?,
                off37: Self::OFF37.from_data(data)?,
                off38: Self::OFF38.from_data(data)?,
                off39: Self::OFF39.from_data(data)?,
                off40: Self::OFF40.from_data(data)?,
                off41: Self::OFF41.from_data(data)?,
                off42: Self::OFF42.from_data(data)?,
                off43: Self::OFF43.from_data(data)?,
                off44: Self::OFF44.from_data(data)?,
                off45: Self::OFF45.from_data(data)?,
                off46: Self::OFF46.from_data(data)?,
                off47: Self::OFF47.from_data(data)?,
                off48: Self::OFF48.from_data(data)?,
                off49: Self::OFF49.from_data(data)?,
                off50: Self::OFF50.from_data(data)?,
                ts: Self::TS.from_data(data)?,
                ms: Self::MS.from_data(data)?,
                seq: Self::SEQ.from_data(data)?,
                role: Self::ROLE.from_data(data)?,
                alg: Self::ALG.from_data(data)?,
                n: Self::N.from_data(data)?,
                repeating,
            },
        ))
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
impl crate::Model for Model3 {
    const ID: u16 = 3;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m3
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
