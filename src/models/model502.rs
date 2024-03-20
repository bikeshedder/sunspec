//! Solar Module

/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Integer
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model502 {
    /// Current scale factor
    pub a_sf: Option<i16>,
    /// Voltage scale factor
    pub v_sf: Option<i16>,
    /// Power scale factor
    pub w_sf: Option<i16>,
    /// Energy scale factor
    pub wh_sf: Option<i16>,
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
    pub out_a: Option<i16>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub out_v: Option<i16>,
    /// Output Energy
    ///
    /// Output Energy
    pub out_wh: Option<u32>,
    /// Output Power
    ///
    /// Output Power
    pub out_pw: Option<i16>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<i16>,
    /// Input Current
    ///
    /// Input Current
    pub in_a: Option<i16>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub in_v: Option<i16>,
    /// Input Energy
    ///
    /// Input Energy
    pub in_wh: Option<u32>,
    /// Input Power
    ///
    /// Input Power
    pub in_w: Option<i16>,
}

#[allow(missing_docs)]

impl Model502 {
    pub const A_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const W_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const WH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const STAT: crate::PointDef<Self, Stat> = crate::PointDef::new(4, 1, false);
    pub const STAT_VEND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(6, 2, false);
    pub const EVT_VEND: crate::PointDef<Self, Option<EvtVend>> = crate::PointDef::new(8, 2, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, true);
    pub const CTL_VEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, true);
    pub const CTL_VAL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(13, 2, true);
    pub const TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(15, 2, false);
    pub const OUT_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
    pub const OUT_V: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const OUT_WH: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(19, 2, false);
    pub const OUT_PW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const IN_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const IN_V: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(24, 1, false);
    pub const IN_WH: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(25, 2, false);
    pub const IN_W: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, false);
}

impl crate::Model for Model502 {
    const ID: u16 = 502;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
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
            out_pw: Self::OUT_PW.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            in_a: Self::IN_A.from_data(data)?,
            in_v: Self::IN_V.from_data(data)?,
            in_wh: Self::IN_WH.from_data(data)?,
            in_w: Self::IN_W.from_data(data)?,
        })
    }
}

#[doc = "Status\n\nEnumerated value.  Module Status Code"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Stat {
    #[doc = ""]
    Off = 1,
    #[doc = ""]
    Sleeping = 2,
    #[doc = ""]
    Starting = 3,
    #[doc = ""]
    Mppt = 4,
    #[doc = ""]
    Throttled = 5,
    #[doc = ""]
    ShuttingDown = 6,
    #[doc = ""]
    Fault = 7,
    #[doc = ""]
    Standby = 8,
    #[doc = ""]
    Test = 9,
    #[doc = ""]
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

bitflags::bitflags! { # [doc = "Events\n\nBitmask value.  Module Event Flags"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct Evt : u32 { # [doc = ""] const GroundFault = 1 ; # [doc = ""] const InputOverVoltage = 2 ; # [doc = ""] const Reserved2 = 4 ; # [doc = ""] const DcDisconnect = 8 ; # [doc = ""] const Reserved4 = 16 ; # [doc = ""] const Reserved5 = 32 ; # [doc = ""] const ManualShutdown = 64 ; # [doc = ""] const OverTemperature = 128 ; # [doc = ""] const Reserved8 = 256 ; # [doc = ""] const Reserved9 = 512 ; # [doc = ""] const Reserved10 = 1024 ; # [doc = ""] const Reserved11 = 2048 ; # [doc = ""] const BlownFuse = 4096 ; # [doc = ""] const UnderTemperature = 8192 ; # [doc = ""] const MemoryLoss = 16384 ; # [doc = ""] const ArcDetection = 32768 ; # [doc = ""] const TheftDetection = 65536 ; # [doc = ""] const OutputOverCurrent = 131072 ; # [doc = ""] const OutputOverVoltage = 262144 ; # [doc = ""] const OutputUnderVoltage = 524288 ; # [doc = ""] const TestFailed = 1048576 ; } }
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

bitflags::bitflags! { # [doc = "Vendor Module Event Flags\n\nVendor specific flags"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVend : u32 { } }
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
