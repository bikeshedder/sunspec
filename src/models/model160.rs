//! Multiple MPPT Inverter Extension Model
/// Type alias for [`Mppt`].
pub type Model160 = Mppt;
/// Multiple MPPT Inverter Extension Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Mppt {
    /// Current Scale Factor
    pub dca_sf: Option<i16>,
    /// Voltage Scale Factor
    pub dcv_sf: Option<i16>,
    /// Power Scale Factor
    pub dcw_sf: Option<i16>,
    /// Energy Scale Factor
    pub dcwh_sf: Option<i16>,
    /// Global Events
    pub evt: Option<Evt>,
    /// Number of Modules
    pub n: Option<u16>,
    /// Timestamp Period
    pub tms_per: Option<u16>,
    #[allow(missing_docs)]
    pub module: Vec<Module>,
}
#[allow(missing_docs)]
impl Mppt {
    pub const DCA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(0, 1, false);
    pub const DCV_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const DCW_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const DCWH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const EVT: crate::Point<Self, Option<Evt>> = crate::Point::new(4, 2, false);
    pub const N: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const TMS_PER: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    fn has_invalid_points(&self) -> bool {
        self.module.iter().any(|group| group.has_invalid_points())
    }
}
impl crate::Group for Mppt {
    const LEN: u16 = 8;
}
impl Mppt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, module) = Module::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                dca_sf: Self::DCA_SF.from_data(data)?,
                dcv_sf: Self::DCV_SF.from_data(data)?,
                dcw_sf: Self::DCW_SF.from_data(data)?,
                dcwh_sf: Self::DCWH_SF.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                n: Self::N.from_data(data)?,
                tms_per: Self::TMS_PER.from_data(data)?,
                module,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Global Events"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Evt : u32 { #[allow(missing_docs)] const GroundFault = 1;
    #[allow(missing_docs)] const InputOverVoltage = 2; #[allow(missing_docs)] const
    Reserved2 = 4; #[allow(missing_docs)] const DcDisconnect = 8; #[allow(missing_docs)]
    const Reserved4 = 16; #[allow(missing_docs)] const CabinetOpen = 32;
    #[allow(missing_docs)] const ManualShutdown = 64; #[allow(missing_docs)] const
    OverTemp = 128; #[allow(missing_docs)] const Reserved8 = 256; #[allow(missing_docs)]
    const Reserved9 = 512; #[allow(missing_docs)] const Reserved10 = 1024;
    #[allow(missing_docs)] const Reserved11 = 2048; #[allow(missing_docs)] const
    BlownFuse = 4096; #[allow(missing_docs)] const UnderTemp = 8192;
    #[allow(missing_docs)] const MemoryLoss = 16384; #[allow(missing_docs)] const
    ArcDetection = 32768; #[allow(missing_docs)] const Reserved16 = 65536;
    #[allow(missing_docs)] const Reserved17 = 131072; #[allow(missing_docs)] const
    Reserved18 = 262144; #[allow(missing_docs)] const Reserved19 = 524288;
    #[allow(missing_docs)] const TestFailed = 1048576; #[allow(missing_docs)] const
    InputUnderVoltage = 2097152; #[allow(missing_docs)] const InputOverCurrent = 4194304;
    }
}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Module {
    /// Input ID
    pub id: Option<u16>,
    /// Input ID String
    pub id_str: Option<String>,
    /// DC Current
    pub dca: Option<u16>,
    /// DC Voltage
    pub dcv: Option<u16>,
    /// DC Power
    pub dcw: Option<u16>,
    /// Lifetime Energy
    pub dcwh: Option<u32>,
    /// Timestamp
    pub tms: Option<u32>,
    /// Temperature
    pub tmp: Option<i16>,
    /// Operating State
    pub dc_st: Option<ModuleDcSt>,
    /// Module Events
    pub dc_evt: Option<ModuleDcEvt>,
}
#[allow(missing_docs)]
impl Module {
    pub const ID: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const ID_STR: crate::Point<Self, Option<String>> = crate::Point::new(1, 8, false);
    pub const DCA: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, false);
    pub const DCV: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const DCW: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const DCWH: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
    pub const TMS: crate::Point<Self, Option<u32>> = crate::Point::new(14, 2, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const DC_ST: crate::Point<Self, Option<ModuleDcSt>> = crate::Point::new(17, 1, false);
    pub const DC_EVT: crate::Point<Self, Option<ModuleDcEvt>> = crate::Point::new(18, 2, false);
    fn has_invalid_points(&self) -> bool {
        false
    }
}
impl crate::Group for Module {
    const LEN: u16 = 20;
}
impl Module {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                id: Self::ID.from_data(data)?,
                id_str: Self::ID_STR.from_data(data)?,
                dca: Self::DCA.from_data(data)?,
                dcv: Self::DCV.from_data(data)?,
                dcw: Self::DCW.from_data(data)?,
                dcwh: Self::DCWH.from_data(data)?,
                tms: Self::TMS.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                dc_st: Self::DC_ST.from_data(data)?,
                dc_evt: Self::DC_EVT.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Module as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Module::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Operating State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ModuleDcSt {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Sleeping,
    #[allow(missing_docs)]
    Starting,
    #[allow(missing_docs)]
    Mppt,
    #[allow(missing_docs)]
    Throttled,
    #[allow(missing_docs)]
    ShuttingDown,
    #[allow(missing_docs)]
    Fault,
    #[allow(missing_docs)]
    Standby,
    #[allow(missing_docs)]
    Test,
    #[allow(missing_docs)]
    Reserved10,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ModuleDcSt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Off,
            2 => Self::Sleeping,
            3 => Self::Starting,
            4 => Self::Mppt,
            5 => Self::Throttled,
            6 => Self::ShuttingDown,
            7 => Self::Fault,
            8 => Self::Standby,
            9 => Self::Test,
            10 => Self::Reserved10,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 1,
            Self::Sleeping => 2,
            Self::Starting => 3,
            Self::Mppt => 4,
            Self::Throttled => 5,
            Self::ShuttingDown => 6,
            Self::Fault => 7,
            Self::Standby => 8,
            Self::Test => 9,
            Self::Reserved10 => 10,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ModuleDcSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Module Events"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct ModuleDcEvt : u32 { #[allow(missing_docs)] const GroundFault = 1;
    #[allow(missing_docs)] const InputOverVoltage = 2; #[allow(missing_docs)] const
    Reserved2 = 4; #[allow(missing_docs)] const DcDisconnect = 8; #[allow(missing_docs)]
    const Reserved4 = 16; #[allow(missing_docs)] const CabinetOpen = 32;
    #[allow(missing_docs)] const ManualShutdown = 64; #[allow(missing_docs)] const
    OverTemp = 128; #[allow(missing_docs)] const Reserved8 = 256; #[allow(missing_docs)]
    const Reserved9 = 512; #[allow(missing_docs)] const Reserved10 = 1024;
    #[allow(missing_docs)] const Reserved11 = 2048; #[allow(missing_docs)] const
    BlownFuse = 4096; #[allow(missing_docs)] const UnderTemp = 8192;
    #[allow(missing_docs)] const MemoryLoss = 16384; #[allow(missing_docs)] const
    ArcDetection = 32768; #[allow(missing_docs)] const Reserved16 = 65536;
    #[allow(missing_docs)] const Reserved17 = 131072; #[allow(missing_docs)] const
    Reserved18 = 262144; #[allow(missing_docs)] const Reserved19 = 524288;
    #[allow(missing_docs)] const TestFailed = 1048576; #[allow(missing_docs)] const
    InputUnderVoltage = 2097152; #[allow(missing_docs)] const InputOverCurrent = 4194304;
    }
}
impl crate::Value for ModuleDcEvt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModuleDcEvt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for Mppt {
    const ID: u16 = 160;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m160
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
