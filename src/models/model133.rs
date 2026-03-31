//! Basic Scheduling
/// Type alias for [`Schedule`].
pub type Model133 = Schedule;
/// Basic Scheduling
///
/// Basic Scheduling
///
/// Detail: Ref 2: 2.2.8
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Schedule {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    pub act_schd: ActSchd,
    /// ModEna
    ///
    /// Is basic scheduling active.
    pub mod_ena: ModEna,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    pub n_schd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    pub n_pts: u16,
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Schedule {
    pub const ACT_SCHD: crate::Point<Self, ActSchd> = crate::Point::new(0, 2, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(2, 1, true);
    pub const N_SCHD: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N_PTS: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
}
impl crate::Group for Schedule {
    const LEN: u16 = 6;
}
impl Schedule {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                act_schd: Self::ACT_SCHD.from_data(data)?,
                mod_ena: Self::MOD_ENA.from_data(data)?,
                n_schd: Self::N_SCHD.from_data(data)?,
                n_pts: Self::N_PTS.from_data(data)?,
                repeating,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " ActSchd"] #[doc = " "] #[doc = " Bitfield of active schedules"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ActSchd : u32 {
    #[allow(missing_docs)] const Sched1 = 1; #[allow(missing_docs)] const Sched2 = 2;
    #[allow(missing_docs)] const Sched3 = 4; #[allow(missing_docs)] const Sched4 = 8;
    #[allow(missing_docs)] const Sched5 = 16; #[allow(missing_docs)] const Sched6 = 32;
    #[allow(missing_docs)] const Sched7 = 64; #[allow(missing_docs)] const Sched8 = 128;
    #[allow(missing_docs)] const Sched9 = 256; #[allow(missing_docs)] const Sched10 =
    512; #[allow(missing_docs)] const Sched12 = 1024; #[allow(missing_docs)] const
    Sched13 = 2048; #[allow(missing_docs)] const Sched14 = 4096; #[allow(missing_docs)]
    const Sched15 = 8192; #[allow(missing_docs)] const Sched16 = 16384;
    #[allow(missing_docs)] const Sched17 = 32768; #[allow(missing_docs)] const Sched18 =
    65536; #[allow(missing_docs)] const Sched19 = 131072; #[allow(missing_docs)] const
    Sched20 = 262144; #[allow(missing_docs)] const Sched21 = 524288;
    #[allow(missing_docs)] const Sched22 = 2097152; #[allow(missing_docs)] const Sched23
    = 4194304; #[allow(missing_docs)] const Sched24 = 8388608; #[allow(missing_docs)]
    const Sched25 = 16777216; #[allow(missing_docs)] const Sched26 = 33554432;
    #[allow(missing_docs)] const Sched27 = 67108864; #[allow(missing_docs)] const Sched28
    = 134217728; #[allow(missing_docs)] const Sched29 = 268435456; #[allow(missing_docs)]
    const Sched30 = 536870912; #[allow(missing_docs)] const Sched31 = 1073741824;
    #[allow(missing_docs)] const Sched32 = 2147483648; }
}
impl crate::Value for ActSchd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ActSchd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc = " Is basic scheduling active."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ModEna : u16 {
    #[allow(missing_docs)] const Enabled = 1; }
}
impl crate::Value for ModEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    /// ActPts
    ///
    /// Number of active entries in schedule.
    pub act_pts: u16,
    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    pub str_tms: u32,
    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    pub rep_per: u16,
    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    pub intv_typ: RepeatingIntvTyp,
    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    pub x_typ: RepeatingXTyp,
    /// X_SF
    ///
    /// Scale factor for schedule range values.
    pub x_sf: i16,
    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    pub y_typ: RepeatingYTyp,
    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    pub y_sf: i16,
    /// X1
    ///
    /// Entry 1 range.
    pub x1: i32,
    /// Y1
    ///
    /// Entry 1 target.
    pub y1: i32,
    /// X2
    ///
    /// Entry 2 range.
    pub x2: Option<i32>,
    /// Y2
    ///
    /// Entry 2 target.
    pub y2: Option<i32>,
    /// X3
    ///
    /// Entry 3 range.
    pub x3: Option<i32>,
    /// Y3
    ///
    /// Entry 3 target.
    pub y3: Option<i32>,
    /// X4
    ///
    /// Entry 4 range.
    pub x4: Option<i32>,
    /// Y4
    ///
    /// Entry 4 target.
    pub y4: Option<i32>,
    /// X5
    ///
    /// Entry 15range.
    pub x5: Option<i32>,
    /// Y5
    ///
    /// Entry 5 target.
    pub y5: Option<i32>,
    /// X6
    ///
    /// Entry 6 range.
    pub x6: Option<i32>,
    /// Y6
    ///
    /// Entry 6 target.
    pub y6: Option<i32>,
    /// X7
    ///
    /// Entry 7 range.
    pub x7: Option<i32>,
    /// Y7
    ///
    /// Entry 7 target.
    pub y7: Option<i32>,
    /// X8
    ///
    /// Entry 8 range.
    pub x8: Option<i32>,
    /// Y8
    ///
    /// Entry 8 target.
    pub y8: Option<i32>,
    /// X9
    ///
    /// Entry 9 range.
    pub x9: Option<i32>,
    /// Y9
    ///
    /// Entry 9 target.
    pub y9: Option<i32>,
    /// X10
    ///
    /// Entry 10 range.
    pub x10: Option<i32>,
    /// Y10
    ///
    /// Entry 10 target.
    pub y10: Option<i32>,
    /// Nam
    ///
    /// Optional description for schedule.
    pub nam: Option<String>,
    /// WinTms
    ///
    /// Time window for schedule entry change.
    pub win_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    pub rmp_tms: Option<u16>,
    /// ActIndx
    ///
    /// Index of active entry in the active schedule.
    pub act_indx: u16,
}
#[allow(missing_docs)]
impl Repeating {
    pub const ACT_PTS: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const STR_TMS: crate::Point<Self, u32> = crate::Point::new(1, 2, true);
    pub const REP_PER: crate::Point<Self, u16> = crate::Point::new(3, 1, true);
    pub const INTV_TYP: crate::Point<Self, RepeatingIntvTyp> = crate::Point::new(4, 1, true);
    pub const X_TYP: crate::Point<Self, RepeatingXTyp> = crate::Point::new(5, 1, true);
    pub const X_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, true);
    pub const Y_TYP: crate::Point<Self, RepeatingYTyp> = crate::Point::new(7, 1, true);
    pub const Y_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, true);
    pub const X1: crate::Point<Self, i32> = crate::Point::new(9, 2, true);
    pub const Y1: crate::Point<Self, i32> = crate::Point::new(11, 2, true);
    pub const X2: crate::Point<Self, Option<i32>> = crate::Point::new(13, 2, true);
    pub const Y2: crate::Point<Self, Option<i32>> = crate::Point::new(15, 2, true);
    pub const X3: crate::Point<Self, Option<i32>> = crate::Point::new(17, 2, true);
    pub const Y3: crate::Point<Self, Option<i32>> = crate::Point::new(19, 2, true);
    pub const X4: crate::Point<Self, Option<i32>> = crate::Point::new(21, 2, true);
    pub const Y4: crate::Point<Self, Option<i32>> = crate::Point::new(23, 2, true);
    pub const X5: crate::Point<Self, Option<i32>> = crate::Point::new(25, 2, true);
    pub const Y5: crate::Point<Self, Option<i32>> = crate::Point::new(27, 2, true);
    pub const X6: crate::Point<Self, Option<i32>> = crate::Point::new(29, 2, true);
    pub const Y6: crate::Point<Self, Option<i32>> = crate::Point::new(31, 2, true);
    pub const X7: crate::Point<Self, Option<i32>> = crate::Point::new(33, 2, true);
    pub const Y7: crate::Point<Self, Option<i32>> = crate::Point::new(35, 2, true);
    pub const X8: crate::Point<Self, Option<i32>> = crate::Point::new(37, 2, true);
    pub const Y8: crate::Point<Self, Option<i32>> = crate::Point::new(39, 2, true);
    pub const X9: crate::Point<Self, Option<i32>> = crate::Point::new(41, 2, true);
    pub const Y9: crate::Point<Self, Option<i32>> = crate::Point::new(43, 2, true);
    pub const X10: crate::Point<Self, Option<i32>> = crate::Point::new(45, 2, true);
    pub const Y10: crate::Point<Self, Option<i32>> = crate::Point::new(47, 2, true);
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(49, 8, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(57, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(58, 1, true);
    pub const ACT_INDX: crate::Point<Self, u16> = crate::Point::new(59, 1, false);
}
impl crate::Group for Repeating {
    const LEN: u16 = 60;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                act_pts: Self::ACT_PTS.from_data(data)?,
                str_tms: Self::STR_TMS.from_data(data)?,
                rep_per: Self::REP_PER.from_data(data)?,
                intv_typ: Self::INTV_TYP.from_data(data)?,
                x_typ: Self::X_TYP.from_data(data)?,
                x_sf: Self::X_SF.from_data(data)?,
                y_typ: Self::Y_TYP.from_data(data)?,
                y_sf: Self::Y_SF.from_data(data)?,
                x1: Self::X1.from_data(data)?,
                y1: Self::Y1.from_data(data)?,
                x2: Self::X2.from_data(data)?,
                y2: Self::Y2.from_data(data)?,
                x3: Self::X3.from_data(data)?,
                y3: Self::Y3.from_data(data)?,
                x4: Self::X4.from_data(data)?,
                y4: Self::Y4.from_data(data)?,
                x5: Self::X5.from_data(data)?,
                y5: Self::Y5.from_data(data)?,
                x6: Self::X6.from_data(data)?,
                y6: Self::Y6.from_data(data)?,
                x7: Self::X7.from_data(data)?,
                y7: Self::Y7.from_data(data)?,
                x8: Self::X8.from_data(data)?,
                y8: Self::Y8.from_data(data)?,
                x9: Self::X9.from_data(data)?,
                y9: Self::Y9.from_data(data)?,
                x10: Self::X10.from_data(data)?,
                y10: Self::Y10.from_data(data)?,
                nam: Self::NAM.from_data(data)?,
                win_tms: Self::WIN_TMS.from_data(data)?,
                rmp_tms: Self::RMP_TMS.from_data(data)?,
                act_indx: Self::ACT_INDX.from_data(data)?,
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
/// SchdTyp
///
/// The repetition frequency for time-based schedules: no repeat=0
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum RepeatingIntvTyp {
    #[allow(missing_docs)]
    Onetime,
    #[allow(missing_docs)]
    Daily,
    #[allow(missing_docs)]
    Weekly,
    #[allow(missing_docs)]
    Monthly,
    #[allow(missing_docs)]
    Weekday,
    #[allow(missing_docs)]
    Holiday,
    #[allow(missing_docs)]
    Weekend,
    #[allow(missing_docs)]
    Yearly,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for RepeatingIntvTyp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Onetime,
            1 => Self::Daily,
            2 => Self::Weekly,
            3 => Self::Monthly,
            4 => Self::Weekday,
            5 => Self::Holiday,
            6 => Self::Weekend,
            7 => Self::Yearly,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Onetime => 0,
            Self::Daily => 1,
            Self::Weekly => 2,
            Self::Monthly => 3,
            Self::Weekday => 4,
            Self::Holiday => 5,
            Self::Weekend => 6,
            Self::Yearly => 7,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for RepeatingIntvTyp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// XTyp
///
/// The meaning of the X-values in the array.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum RepeatingXTyp {
    #[allow(missing_docs)]
    Unset,
    #[allow(missing_docs)]
    Time,
    #[allow(missing_docs)]
    Temp,
    #[allow(missing_docs)]
    Price,
    #[allow(missing_docs)]
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for RepeatingXTyp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unset,
            1 => Self::Time,
            2 => Self::Temp,
            3 => Self::Price,
            99 => Self::Other,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unset => 0,
            Self::Time => 1,
            Self::Temp => 2,
            Self::Price => 3,
            Self::Other => 99,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for RepeatingXTyp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// YTyp
///
/// The meaning of the Y-values in the array.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum RepeatingYTyp {
    #[allow(missing_docs)]
    Unset,
    #[allow(missing_docs)]
    WMax,
    #[allow(missing_docs)]
    Rsrvd2,
    #[allow(missing_docs)]
    Pf,
    #[allow(missing_docs)]
    Rsrvd4,
    #[allow(missing_docs)]
    WattPrice,
    #[allow(missing_docs)]
    VarPrice,
    #[allow(missing_docs)]
    Rsrvd7,
    #[allow(missing_docs)]
    VoltVarArray,
    #[allow(missing_docs)]
    WChaGra,
    #[allow(missing_docs)]
    WDisChaGra,
    #[allow(missing_docs)]
    VArAval,
    #[allow(missing_docs)]
    Schedule,
    #[allow(missing_docs)]
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for RepeatingYTyp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unset,
            1 => Self::WMax,
            2 => Self::Rsrvd2,
            3 => Self::Pf,
            4 => Self::Rsrvd4,
            5 => Self::WattPrice,
            6 => Self::VarPrice,
            7 => Self::Rsrvd7,
            8 => Self::VoltVarArray,
            9 => Self::WChaGra,
            10 => Self::WDisChaGra,
            11 => Self::VArAval,
            12 => Self::Schedule,
            99 => Self::Other,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unset => 0,
            Self::WMax => 1,
            Self::Rsrvd2 => 2,
            Self::Pf => 3,
            Self::Rsrvd4 => 4,
            Self::WattPrice => 5,
            Self::VarPrice => 6,
            Self::Rsrvd7 => 7,
            Self::VoltVarArray => 8,
            Self::WChaGra => 9,
            Self::WDisChaGra => 10,
            Self::VArAval => 11,
            Self::Schedule => 12,
            Self::Other => 99,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for RepeatingYTyp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Schedule {
    const ID: u16 = 133;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m133
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
