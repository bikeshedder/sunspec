//! String Combiner (Current)

/// String Combiner (Current)
///
/// A basic string combiner
///
/// Notes: This model is SUPERSEDED by model 403
#[derive(Debug)]
pub struct Model401 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dca_max: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
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
}

#[allow(missing_docs)]

impl Model401 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DC_AHR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const DCA_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(5, 2, false);
    pub const EVT_VND: crate::PointDef<Self, Option<EvtVnd>> = crate::PointDef::new(7, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const DC_AHR: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(10, 2, false);
    pub const DCV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(13, 1, false);
}

impl crate::Model for Model401 {
    const ID: u16 = 401;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dca_max: Self::DCA_MAX.from_data(data)?,
            n: Self::N.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evt_vnd: Self::EVT_VND.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dc_ahr: Self::DC_AHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "Event\n\nBitmask value.  Events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Evt : u32 { # [doc = ""] const LowVoltage = 1 ; # [doc = ""] const LowPower = 2 ; # [doc = ""] const LowEfficiency = 4 ; # [doc = ""] const Current = 8 ; # [doc = ""] const Voltage = 16 ; # [doc = ""] const Power = 32 ; # [doc = ""] const Pr = 64 ; # [doc = ""] const Disconnected = 128 ; # [doc = ""] const FuseFault = 256 ; # [doc = ""] const CombinerFuseFault = 512 ; # [doc = ""] const CombinerCabinetOpen = 1024 ; # [doc = ""] const Temp = 2048 ; # [doc = ""] const Groundfault = 4096 ; # [doc = ""] const ReversedPolarity = 8192 ; # [doc = ""] const Incompatible = 16384 ; # [doc = ""] const CommError = 32768 ; # [doc = ""] const InternalError = 65536 ; # [doc = ""] const Theft = 131072 ; # [doc = ""] const ArcDetected = 262144 ; } }
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

bitflags::bitflags! { # [doc = "Vendor Event\n\nBitmask value.  Vendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct EvtVnd : u32 { } }
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
