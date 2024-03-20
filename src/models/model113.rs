//! Inverter (Three Phase) FLOAT

/// Inverter (Three Phase) FLOAT
///
/// Include this model for three phase inverter monitoring using float values
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model113 {
    /// Amps
    ///
    /// AC Current
    ///
    /// Notes: Sum of active phases
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub aph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aph_b: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Notes: Connected Phase
    pub aph_c: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_vph_ab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_vph_bc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_vph_ca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_vph_a: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_vph_b: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_vph_c: f32,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub v_ar: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmp_cab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmp_snk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmp_trns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmp_ot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: St,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub st_vnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: Evt1,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: Evt2,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evt_vnd1: Option<EvtVnd1>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evt_vnd2: Option<EvtVnd2>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evt_vnd3: Option<EvtVnd3>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evt_vnd4: Option<EvtVnd4>,
}

#[allow(missing_docs)]

impl Model113 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APH_A: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APH_B: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APH_C: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PP_VPH_AB: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(8, 2, false);
    pub const PP_VPH_BC: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(10, 2, false);
    pub const PP_VPH_CA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(12, 2, false);
    pub const PH_VPH_A: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PH_VPH_B: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PH_VPH_C: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(24, 2, false);
    pub const V_AR: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(36, 2, false);
    pub const TMP_CAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMP_SNK: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(40, 2, false);
    pub const TMP_TRNS: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(42, 2, false);
    pub const TMP_OT: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(46, 1, false);
    pub const ST_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, Evt1> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, Evt2> = crate::PointDef::new(50, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, Option<EvtVnd1>> = crate::PointDef::new(52, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, Option<EvtVnd2>> = crate::PointDef::new(54, 2, false);
    pub const EVT_VND3: crate::PointDef<Self, Option<EvtVnd3>> = crate::PointDef::new(56, 2, false);
    pub const EVT_VND4: crate::PointDef<Self, Option<EvtVnd4>> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model113 {
    const ID: u16 = 113;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            aph_a: Self::APH_A.from_data(data)?,
            aph_b: Self::APH_B.from_data(data)?,
            aph_c: Self::APH_C.from_data(data)?,
            pp_vph_ab: Self::PP_VPH_AB.from_data(data)?,
            pp_vph_bc: Self::PP_VPH_BC.from_data(data)?,
            pp_vph_ca: Self::PP_VPH_CA.from_data(data)?,
            ph_vph_a: Self::PH_VPH_A.from_data(data)?,
            ph_vph_b: Self::PH_VPH_B.from_data(data)?,
            ph_vph_c: Self::PH_VPH_C.from_data(data)?,
            w: Self::W.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            va: Self::VA.from_data(data)?,
            v_ar: Self::V_AR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmp_cab: Self::TMP_CAB.from_data(data)?,
            tmp_snk: Self::TMP_SNK.from_data(data)?,
            tmp_trns: Self::TMP_TRNS.from_data(data)?,
            tmp_ot: Self::TMP_OT.from_data(data)?,
            st: Self::ST.from_data(data)?,
            st_vnd: Self::ST_VND.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evt_vnd1: Self::EVT_VND1.from_data(data)?,
            evt_vnd2: Self::EVT_VND2.from_data(data)?,
            evt_vnd3: Self::EVT_VND3.from_data(data)?,
            evt_vnd4: Self::EVT_VND4.from_data(data)?,
        })
    }
}

#[doc = "Operating State\n\nEnumerated value.  Operating state"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum St {
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

bitflags::bitflags! { # [doc = "Event1\n\nBitmask value. Event fields"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct Evt1 : u32 { # [doc = ""] const GroundFault = 1 ; # [doc = ""] const DcOverVolt = 2 ; # [doc = ""] const AcDisconnect = 4 ; # [doc = ""] const DcDisconnect = 8 ; # [doc = ""] const GridDisconnect = 16 ; # [doc = ""] const CabinetOpen = 32 ; # [doc = ""] const ManualShutdown = 64 ; # [doc = ""] const OverTemp = 128 ; # [doc = ""] const OverFrequency = 256 ; # [doc = ""] const UnderFrequency = 512 ; # [doc = ""] const AcOverVolt = 1024 ; # [doc = ""] const AcUnderVolt = 2048 ; # [doc = ""] const BlownStringFuse = 4096 ; # [doc = ""] const UnderTemp = 8192 ; # [doc = ""] const MemoryLoss = 16384 ; # [doc = ""] const HwTestFailure = 32768 ; } }
impl crate::Value for Evt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Evt1> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt1::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "Event Bitfield 2\n\nReserved for future use"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct Evt2 : u32 { } }
impl crate::Value for Evt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Evt2> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt2::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 1\n\nVendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVnd1 : u32 { } }
impl crate::Value for EvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd1> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd1::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 2\n\nVendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVnd2 : u32 { } }
impl crate::Value for EvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd2> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd2::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 3\n\nVendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVnd3 : u32 { } }
impl crate::Value for EvtVnd3 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd3> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd3::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 4\n\nVendor defined events"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct EvtVnd4 : u32 { } }
impl crate::Value for EvtVnd4 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd4> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd4::from_bits_retain(value)))
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
