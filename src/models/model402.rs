//! String Combiner (Advanced)

/// String Combiner (Advanced)
///
/// An advanced string combiner
///
/// Notes: This model is SUPERSEDED by model 404
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model402 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Power scale factor
    pub dcw_sf: Option<i16>,
    /// Energy scale factor
    pub dc_wh_sf: i16,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dca_max: Option<u16>,
    /// N
    ///
    /// Number of Inputs
    pub n: Option<u16>,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: Evt,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evt_vnd: Option<EvtVnd>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dcpr: Option<u16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dc_wh: u32,
}

#[allow(missing_docs)]

impl Model402 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DC_AHR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const DC_WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const DCA_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const N: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(7, 2, false);
    pub const EVT_VND: crate::PointDef<Self, Option<EvtVnd>> = crate::PointDef::new(9, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const DC_AHR: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
    pub const DCV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const DCW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
    pub const DCPR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, false);
    pub const DC_WH: crate::PointDef<Self, u32> = crate::PointDef::new(18, 2, false);
}

impl crate::Model for Model402 {
    const ID: u16 = 402;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dc_wh_sf: Self::DC_WH_SF.from_data(data)?,
            dca_max: Self::DCA_MAX.from_data(data)?,
            n: Self::N.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evt_vnd: Self::EVT_VND.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dc_ahr: Self::DC_AHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcpr: Self::DCPR.from_data(data)?,
            dc_wh: Self::DC_WH.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "Event\n\nBitmask value.  Events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct Evt : u32 { # [doc = ""] const LowVoltage = 1 ; # [doc = ""] const LowPower = 2 ; # [doc = ""] const LowEfficiency = 4 ; # [doc = ""] const Current = 8 ; # [doc = ""] const Voltage = 16 ; # [doc = ""] const Power = 32 ; # [doc = ""] const Pr = 64 ; # [doc = ""] const Disconnected = 128 ; # [doc = ""] const FuseFault = 256 ; # [doc = ""] const CombinerFuseFault = 512 ; # [doc = ""] const CombinerCabinetOpen = 1024 ; # [doc = ""] const Temp = 2048 ; # [doc = ""] const Groundfault = 4096 ; # [doc = ""] const ReversedPolarity = 8192 ; # [doc = ""] const Incompatible = 16384 ; # [doc = ""] const CommError = 32768 ; # [doc = ""] const InternalError = 65536 ; # [doc = ""] const Theft = 131072 ; # [doc = ""] const ArcDetected = 262144 ; } }
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

bitflags::bitflags! { # [doc = "Vendor Event\n\nBitmask value.  Vendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVnd : u32 { } }
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
