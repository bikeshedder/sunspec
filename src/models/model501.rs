//! Solar Module
/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Float
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model501 {
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
impl Model501 {
    pub const STAT: crate::PointDef<Self, Stat> = crate::PointDef::new(0, 1, false);
    pub const STAT_VEND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(2, 2, false);
    pub const EVT_VEND: crate::PointDef<Self, Option<EvtVend>> = crate::PointDef::new(4, 2, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const CTL_VEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, true);
    pub const CTL_VAL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(9, 2, true);
    pub const TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, false);
    pub const OUT_A: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(13, 2, false);
    pub const OUT_V: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(15, 2, false);
    pub const OUT_WH: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(17, 2, false);
    pub const OUT_W: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(19, 2, false);
    pub const TMP: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(21, 2, false);
    pub const IN_A: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(23, 2, false);
    pub const IN_V: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(25, 2, false);
    pub const IN_WH: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(27, 2, false);
    pub const IN_W: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(29, 2, false);
}
impl crate::Model for Model501 {
    const ID: u16 = 501;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
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
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m501
    }
}
/// Status
///
/// Enumerated value.  Module Status Code
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Stat {
    #[allow(missing_docs)]
    Off = 1,
    #[allow(missing_docs)]
    Sleeping = 2,
    #[allow(missing_docs)]
    Starting = 3,
    #[allow(missing_docs)]
    Mppt = 4,
    #[allow(missing_docs)]
    Throttled = 5,
    #[allow(missing_docs)]
    ShuttingDown = 6,
    #[allow(missing_docs)]
    Fault = 7,
    #[allow(missing_docs)]
    Standby = 8,
    #[allow(missing_docs)]
    Test = 9,
    #[allow(missing_docs)]
    Other = 10,
}
impl crate::Value for Stat {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Stat> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Stat::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
impl crate::Value for Option<Evt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
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
impl crate::Value for Option<EvtVend> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVend::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}
