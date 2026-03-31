//! Basic Aggregator
/// Type alias for [`Aggregator`].
pub type Model2 = Aggregator;
/// Basic Aggregator
///
/// Aggregates a collection of models for a given model id
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Aggregator {
    /// AID
    ///
    /// Aggregated model id
    pub aid: u16,
    /// N
    ///
    /// Number of aggregated models
    pub n: u16,
    /// UN
    ///
    /// Update Number.  Incrementing number each time the mapping is changed.  If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before.  Otherwise the entire model must be read to refresh the changes
    pub un: u16,
    /// Status
    ///
    /// Enumerated status code
    pub st: St,
    /// Vendor Status
    ///
    /// Vendor specific status code
    pub st_vnd: Option<u16>,
    /// Event Code
    ///
    /// Bitmask event code
    pub evt: Evt,
    /// Vendor Event Code
    ///
    /// Vendor specific event code
    pub evt_vnd: Option<EvtVnd>,
    /// Control
    ///
    /// Control register for all aggregated devices
    pub ctl: Option<Ctl>,
    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    pub ctl_vnd: Option<u32>,
    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    pub ctl_vl: Option<u32>,
}
#[allow(missing_docs)]
impl Aggregator {
    pub const AID: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const UN: crate::Point<Self, u16> = crate::Point::new(2, 1, false);
    pub const ST: crate::Point<Self, St> = crate::Point::new(3, 1, false);
    pub const ST_VND: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(5, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(7, 2, false);
    pub const CTL: crate::Point<Self, Option<Ctl>> = crate::Point::new(9, 1, false);
    pub const CTL_VND: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2, false);
    pub const CTL_VL: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
}
impl crate::Group for Aggregator {
    const LEN: u16 = 14;
}
impl Aggregator {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                aid: Self::AID.from_data(data)?,
                n: Self::N.from_data(data)?,
                un: Self::UN.from_data(data)?,
                st: Self::ST.from_data(data)?,
                st_vnd: Self::ST_VND.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                ctl: Self::CTL.from_data(data)?,
                ctl_vnd: Self::CTL_VND.from_data(data)?,
                ctl_vl: Self::CTL_VL.from_data(data)?,
            },
        ))
    }
}
/// Status
///
/// Enumerated status code
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum St {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    On,
    #[allow(missing_docs)]
    Full,
    #[allow(missing_docs)]
    Fault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for St {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Off,
            2 => Self::On,
            3 => Self::Full,
            4 => Self::Fault,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 1,
            Self::On => 2,
            Self::Full => 3,
            Self::Fault => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for St {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Event Code"] #[doc = " "] #[doc = " Bitmask event code"] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
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
    #[doc = " Vendor Event Code"] #[doc = " "] #[doc = " Vendor specific event code"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd : u32 {}
}
impl crate::Value for EvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Control
///
/// Control register for all aggregated devices
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Ctl {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Automatic,
    #[allow(missing_docs)]
    ForceOff,
    #[allow(missing_docs)]
    Test,
    #[allow(missing_docs)]
    Throttle,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Ctl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Automatic,
            2 => Self::ForceOff,
            3 => Self::Test,
            4 => Self::Throttle,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Automatic => 1,
            Self::ForceOff => 2,
            Self::Test => 3,
            Self::Throttle => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Ctl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Aggregator {
    const ID: u16 = 2;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m2
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
