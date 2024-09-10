//! Inverter (Single Phase)
/// Inverter (Single Phase)
///
/// Include this model for single phase inverter monitoring
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model101 {
    /// Amps
    ///
    /// AC Current
    pub a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub aph_a: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aph_b: Option<u16>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aph_c: Option<u16>,
    #[allow(missing_docs)]
    pub a_sf: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_vph_ab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_vph_bc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_vph_ca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_vph_a: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_vph_b: Option<u16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_vph_c: Option<u16>,
    #[allow(missing_docs)]
    pub v_sf: i16,
    /// Watts
    ///
    /// AC Power
    pub w: i16,
    #[allow(missing_docs)]
    pub w_sf: i16,
    /// Hz
    ///
    /// Line Frequency
    pub hz: u16,
    #[allow(missing_docs)]
    pub hz_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    #[allow(missing_docs)]
    pub va_sf: Option<i16>,
    /// VAr
    ///
    /// AC Reactive Power
    pub v_ar: Option<i16>,
    #[allow(missing_docs)]
    pub v_ar_sf: Option<i16>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<i16>,
    #[allow(missing_docs)]
    pub pf_sf: Option<i16>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: u32,
    #[allow(missing_docs)]
    pub wh_sf: i16,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<u16>,
    #[allow(missing_docs)]
    pub dca_sf: Option<i16>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<u16>,
    #[allow(missing_docs)]
    pub dcv_sf: Option<i16>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<i16>,
    #[allow(missing_docs)]
    pub dcw_sf: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmp_cab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmp_snk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmp_trns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmp_ot: Option<i16>,
    #[allow(missing_docs)]
    pub tmp_sf: i16,
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
impl Model101 {
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const APH_A: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const APH_B: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const APH_C: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PP_VPH_AB: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const PP_VPH_BC: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const PP_VPH_CA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
    pub const PH_VPH_A: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const PH_VPH_B: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, false);
    pub const PH_VPH_C: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const VA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
    pub const VA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
    pub const V_AR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const V_AR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(19, 1, false);
    pub const PF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(20, 1, false);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const WH: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const DCA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(25, 1, false);
    pub const DCA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(26, 1, false);
    pub const DCV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(27, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(28, 1, false);
    pub const DCW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(29, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(30, 1, false);
    pub const TMP_CAB: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const TMP_SNK: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(32, 1, false);
    pub const TMP_TRNS: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(33, 1, false);
    pub const TMP_OT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(34, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(36, 1, false);
    pub const ST_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(37, 1, false);
    pub const EVT1: crate::PointDef<Self, Evt1> = crate::PointDef::new(38, 2, false);
    pub const EVT2: crate::PointDef<Self, Evt2> = crate::PointDef::new(40, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, Option<EvtVnd1>> = crate::PointDef::new(42, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, Option<EvtVnd2>> = crate::PointDef::new(44, 2, false);
    pub const EVT_VND3: crate::PointDef<Self, Option<EvtVnd3>> = crate::PointDef::new(46, 2, false);
    pub const EVT_VND4: crate::PointDef<Self, Option<EvtVnd4>> = crate::PointDef::new(48, 2, false);
}
impl crate::Model for Model101 {
    const ID: u16 = 101;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            aph_a: Self::APH_A.from_data(data)?,
            aph_b: Self::APH_B.from_data(data)?,
            aph_c: Self::APH_C.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            pp_vph_ab: Self::PP_VPH_AB.from_data(data)?,
            pp_vph_bc: Self::PP_VPH_BC.from_data(data)?,
            pp_vph_ca: Self::PP_VPH_CA.from_data(data)?,
            ph_vph_a: Self::PH_VPH_A.from_data(data)?,
            ph_vph_b: Self::PH_VPH_B.from_data(data)?,
            ph_vph_c: Self::PH_VPH_C.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            w: Self::W.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            v_ar: Self::V_AR.from_data(data)?,
            v_ar_sf: Self::V_AR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wh: Self::WH.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            tmp_cab: Self::TMP_CAB.from_data(data)?,
            tmp_snk: Self::TMP_SNK.from_data(data)?,
            tmp_trns: Self::TMP_TRNS.from_data(data)?,
            tmp_ot: Self::TMP_OT.from_data(data)?,
            tmp_sf: Self::TMP_SF.from_data(data)?,
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
/// Operating State
///
/// Enumerated value.  Operating state
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum St {
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
    #[doc = " Event1"] #[doc = " "] #[doc = " Bitmask value. Event fields"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt1 : u32 {
    #[allow(missing_docs)] const GroundFault = 1; #[allow(missing_docs)] const DcOverVolt
    = 2; #[allow(missing_docs)] const AcDisconnect = 4; #[allow(missing_docs)] const
    DcDisconnect = 8; #[allow(missing_docs)] const GridDisconnect = 16;
    #[allow(missing_docs)] const CabinetOpen = 32; #[allow(missing_docs)] const
    ManualShutdown = 64; #[allow(missing_docs)] const OverTemp = 128;
    #[allow(missing_docs)] const OverFrequency = 256; #[allow(missing_docs)] const
    UnderFrequency = 512; #[allow(missing_docs)] const AcOverVolt = 1024;
    #[allow(missing_docs)] const AcUnderVolt = 2048; #[allow(missing_docs)] const
    BlownStringFuse = 4096; #[allow(missing_docs)] const UnderTemp = 8192;
    #[allow(missing_docs)] const MemoryLoss = 16384; #[allow(missing_docs)] const
    HwTestFailure = 32768; }
}
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
bitflags::bitflags! {
    #[doc = " Event Bitfield 2"] #[doc = " "] #[doc = " Reserved for future use"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt2 : u32 {}
}
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
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 1"] #[doc = " "] #[doc = " Vendor defined events"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd1 : u32 {}
}
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
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 2"] #[doc = " "] #[doc = " Vendor defined events"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd2 : u32 {}
}
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
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 3"] #[doc = " "] #[doc = " Vendor defined events"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd3 : u32 {}
}
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
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 4"] #[doc = " "] #[doc = " Vendor defined events"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd4 : u32 {}
}
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
