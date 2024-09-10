//! Basic Aggregator
/// Basic Aggregator
///
/// Aggregates a collection of models for a given model id
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model2 {
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
impl Model2 {
    pub const AID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const UN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(3, 1, false);
    pub const ST_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(5, 2, false);
    pub const EVT_VND: crate::PointDef<Self, Option<EvtVnd>> = crate::PointDef::new(7, 2, false);
    pub const CTL: crate::PointDef<Self, Option<Ctl>> = crate::PointDef::new(9, 1, false);
    pub const CTL_VND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(10, 2, false);
    pub const CTL_VL: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
}
impl crate::Model for Model2 {
    const ID: u16 = 2;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
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
        })
    }
}
/// Status
///
/// Enumerated status code
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum St {
    #[allow(missing_docs)]
    Off = 1,
    #[allow(missing_docs)]
    On = 2,
    #[allow(missing_docs)]
    Full = 3,
    #[allow(missing_docs)]
    Fault = 4,
}
impl crate::Value for St {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<St> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                St::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
impl crate::Value for Option<EvtVnd> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd::from_bits_retain(value)))
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
/// Control
///
/// Control register for all aggregated devices
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Ctl {
    #[allow(missing_docs)]
    None = 0,
    #[allow(missing_docs)]
    Automatic = 1,
    #[allow(missing_docs)]
    ForceOff = 2,
    #[allow(missing_docs)]
    Test = 3,
    #[allow(missing_docs)]
    Throttle = 4,
}
impl crate::Value for Ctl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Ctl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Ctl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
