//! Solar Module
/// Type alias for [`SolarModuleFloat`].
pub type Model501 = SolarModuleFloat;
/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Detail: Float
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SolarModuleFloat {
    /// Status
    ///
    /// Enumerated value.  Module Status Code
    pub stat: Stat,
    /// Vendor Status
    ///
    /// Module Vendor Status Code
    pub stat_vend: Option<u16>,
    /// Events
    ///
    /// Bitmask value.  Module Event Flags
    pub evt: Evt,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    pub evt_vend: Option<EvtVend>,
    /// Control
    ///
    /// Module Control
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    pub ctl_vend: Option<u32>,
    /// Control Value
    ///
    /// Module Control Value
    pub ctl_val: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    pub tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    pub out_a: Option<f32>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub out_v: Option<f32>,
    /// Output Energy
    ///
    /// Output Energy
    pub out_wh: Option<f32>,
    /// Output Power
    ///
    /// Output Power
    pub out_w: Option<f32>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<f32>,
    /// Input Current
    ///
    /// Input Current
    pub in_a: Option<f32>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub in_v: Option<f32>,
    /// Input Energy
    ///
    /// Input Energy
    pub in_wh: Option<f32>,
    /// Input Power
    ///
    /// Input Power
    pub in_w: Option<f32>,
}
#[allow(missing_docs)]
impl SolarModuleFloat {
    pub const STAT: crate::Point<Self, Stat> = crate::Point::new(0, 1, false);
    pub const STAT_VEND: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(2, 2, false);
    pub const EVT_VEND: crate::Point<Self, Option<EvtVend>> = crate::Point::new(4, 2, false);
    pub const CTL: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const CTL_VEND: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, true);
    pub const CTL_VAL: crate::Point<Self, Option<i32>> = crate::Point::new(9, 2, true);
    pub const TMS: crate::Point<Self, Option<u32>> = crate::Point::new(11, 2, false);
    pub const OUT_A: crate::Point<Self, Option<f32>> = crate::Point::new(13, 2, false);
    pub const OUT_V: crate::Point<Self, Option<f32>> = crate::Point::new(15, 2, false);
    pub const OUT_WH: crate::Point<Self, Option<f32>> = crate::Point::new(17, 2, false);
    pub const OUT_W: crate::Point<Self, Option<f32>> = crate::Point::new(19, 2, false);
    pub const TMP: crate::Point<Self, Option<f32>> = crate::Point::new(21, 2, false);
    pub const IN_A: crate::Point<Self, Option<f32>> = crate::Point::new(23, 2, false);
    pub const IN_V: crate::Point<Self, Option<f32>> = crate::Point::new(25, 2, false);
    pub const IN_WH: crate::Point<Self, Option<f32>> = crate::Point::new(27, 2, false);
    pub const IN_W: crate::Point<Self, Option<f32>> = crate::Point::new(29, 2, false);
}
impl crate::Group for SolarModuleFloat {
    const LEN: u16 = 31;
}
impl SolarModuleFloat {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                stat: Self::STAT.from_data(data)?,
                stat_vend: Self::STAT_VEND.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vend: Self::EVT_VEND.from_data(data)?,
                ctl: Self::CTL.from_data(data)?,
                ctl_vend: Self::CTL_VEND.from_data(data)?,
                ctl_val: Self::CTL_VAL.from_data(data)?,
                tms: Self::TMS.from_data(data)?,
                out_a: Self::OUT_A.from_data(data)?,
                out_v: Self::OUT_V.from_data(data)?,
                out_wh: Self::OUT_WH.from_data(data)?,
                out_w: Self::OUT_W.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                in_a: Self::IN_A.from_data(data)?,
                in_v: Self::IN_V.from_data(data)?,
                in_wh: Self::IN_WH.from_data(data)?,
                in_w: Self::IN_W.from_data(data)?,
            },
        ))
    }
}
/// Status
///
/// Enumerated value.  Module Status Code
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Stat {
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
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Stat {
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
            10 => Self::Other,
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
            Self::Other => 10,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Stat {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Events"] #[doc = " "] #[doc = " Bitmask value.  Module Event Flags"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt : u32 {
    #[allow(missing_docs)] const GroundFault = 1; #[allow(missing_docs)] const
    InputOverVoltage = 2; #[allow(missing_docs)] const Reserved2 = 4;
    #[allow(missing_docs)] const DcDisconnect = 8; #[allow(missing_docs)] const Reserved4
    = 16; #[allow(missing_docs)] const Reserved5 = 32; #[allow(missing_docs)] const
    ManualShutdown = 64; #[allow(missing_docs)] const OverTemperature = 128;
    #[allow(missing_docs)] const Reserved8 = 256; #[allow(missing_docs)] const Reserved9
    = 512; #[allow(missing_docs)] const Reserved10 = 1024; #[allow(missing_docs)] const
    Reserved11 = 2048; #[allow(missing_docs)] const BlownFuse = 4096;
    #[allow(missing_docs)] const UnderTemperature = 8192; #[allow(missing_docs)] const
    MemoryLoss = 16384; #[allow(missing_docs)] const ArcDetection = 32768;
    #[allow(missing_docs)] const TheftDetection = 65536; #[allow(missing_docs)] const
    OutputOverCurrent = 131072; #[allow(missing_docs)] const OutputOverVoltage = 262144;
    #[allow(missing_docs)] const OutputUnderVoltage = 524288; #[allow(missing_docs)]
    const TestFailed = 1048576; }
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
bitflags::bitflags! {
    #[doc = " Vendor Module Event Flags"] #[doc = " "] #[doc = " Vendor specific flags"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVend : u32 {}
}
impl crate::Value for EvtVend {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVend {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for SolarModuleFloat {
    const ID: u16 = 501;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m501
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
