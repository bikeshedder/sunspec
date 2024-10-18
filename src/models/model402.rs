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
    pub const DCA_SF: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const DCV_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const DCW_SF: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const DC_WH_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const DCA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const N: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(7, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(9, 2, false);
    pub const DCA: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
    pub const DCV: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const DCW: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const DCPR: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const DC_WH: crate::Point<Self, u32> = crate::Point::new(18, 2, false);
}
impl crate::Model for Model402 {
    const ID: u16 = 402;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
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
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m402
    }
}
bitflags::bitflags! {
    #[doc = " Event"] #[doc = " "] #[doc = " Bitmask value.  Events"] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt : u32 {
    #[allow(missing_docs)] const LowVoltage = 1; #[allow(missing_docs)] const LowPower =
    2; #[allow(missing_docs)] const LowEfficiency = 4; #[allow(missing_docs)] const
    Current = 8; #[allow(missing_docs)] const Voltage = 16; #[allow(missing_docs)] const
    Power = 32; #[allow(missing_docs)] const Pr = 64; #[allow(missing_docs)] const
    Disconnected = 128; #[allow(missing_docs)] const FuseFault = 256;
    #[allow(missing_docs)] const CombinerFuseFault = 512; #[allow(missing_docs)] const
    CombinerCabinetOpen = 1024; #[allow(missing_docs)] const Temp = 2048;
    #[allow(missing_docs)] const Groundfault = 4096; #[allow(missing_docs)] const
    ReversedPolarity = 8192; #[allow(missing_docs)] const Incompatible = 16384;
    #[allow(missing_docs)] const CommError = 32768; #[allow(missing_docs)] const
    InternalError = 65536; #[allow(missing_docs)] const Theft = 131072;
    #[allow(missing_docs)] const ArcDetected = 262144; }
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
    #[doc = " Vendor Event"] #[doc = " "] #[doc =
    " Bitmask value.  Vendor defined events"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct EvtVnd : u32 {}
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
