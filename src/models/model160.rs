//! Multiple MPPT Inverter Extension Model

/// Multiple MPPT Inverter Extension Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model160 {
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
}

#[allow(missing_docs)]

impl Model160 {
    pub const DCA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const EVT: crate::PointDef<Self, Option<Evt>> = crate::PointDef::new(4, 2, false);
    pub const N: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const TMS_PER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model160 {
    const ID: u16 = 160;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            n: Self::N.from_data(data)?,
            tms_per: Self::TMS_PER.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "Global Events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct Evt : u32 { # [doc = ""] const GroundFault = 1 ; # [doc = ""] const InputOverVoltage = 2 ; # [doc = ""] const Reserved2 = 4 ; # [doc = ""] const DcDisconnect = 8 ; # [doc = ""] const Reserved4 = 16 ; # [doc = ""] const CabinetOpen = 32 ; # [doc = ""] const ManualShutdown = 64 ; # [doc = ""] const OverTemp = 128 ; # [doc = ""] const Reserved8 = 256 ; # [doc = ""] const Reserved9 = 512 ; # [doc = ""] const Reserved10 = 1024 ; # [doc = ""] const Reserved11 = 2048 ; # [doc = ""] const BlownFuse = 4096 ; # [doc = ""] const UnderTemp = 8192 ; # [doc = ""] const MemoryLoss = 16384 ; # [doc = ""] const ArcDetection = 32768 ; # [doc = ""] const Reserved16 = 65536 ; # [doc = ""] const Reserved17 = 131072 ; # [doc = ""] const Reserved18 = 262144 ; # [doc = ""] const Reserved19 = 524288 ; # [doc = ""] const TestFailed = 1048576 ; # [doc = ""] const InputUnderVoltage = 2097152 ; # [doc = ""] const InputOverCurrent = 4194304 ; } }
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
