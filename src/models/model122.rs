//! Measurements_Status

/// Measurements_Status
///
/// Inverter Controls Extended Measurements and Status
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model122 {
    /// PVConn
    ///
    /// PV inverter present/available status. Enumerated value.
    pub pv_conn: PvConn,
    /// StorConn
    ///
    /// Storage inverter present/available status. Enumerated value.
    pub stor_conn: StorConn,
    /// ECPConn
    ///
    /// ECP connection status: disconnected=0  connected=1.
    pub ecp_conn: EcpConn,
    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    pub act_wh: Option<u64>,
    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    pub act_v_ah: Option<u64>,
    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    pub act_v_arh_q1: Option<u64>,
    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    pub act_v_arh_q2: Option<u64>,
    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output  in quadrant 3.
    pub act_v_arh_q3: Option<u64>,
    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output  in quadrant 4.
    pub act_v_arh_q4: Option<u64>,
    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    pub v_ar_aval: Option<i16>,
    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    pub v_ar_aval_sf: Option<i16>,
    /// WAval
    ///
    /// Amount of Watts available.
    pub w_aval: Option<u16>,
    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    pub w_aval_sf: Option<i16>,
    /// StSetLimMsk
    ///
    /// Bit Mask indicating setpoint limit(s) reached.
    ///
    /// Notes: Bits shall be automatically cleared on read.
    pub st_set_lim_msk: Option<StSetLimMsk>,
    /// StActCtl
    ///
    /// Bit Mask indicating which inverter controls are currently active.
    pub st_act_ctl: Option<StActCtl>,
    /// TmSrc
    ///
    /// Source of time synchronization.
    pub tm_src: Option<String>,
    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    pub tms: Option<u32>,
    /// RtSt
    ///
    /// Bit Mask indicating active ride-through status.
    pub rt_st: Option<RtSt>,
    /// Ris
    ///
    /// Isolation resistance.
    pub ris: Option<u16>,
    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    pub ris_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model122 {
    pub const PV_CONN: crate::PointDef<Self, PvConn> = crate::PointDef::new(0, 1, false);
    pub const STOR_CONN: crate::PointDef<Self, StorConn> = crate::PointDef::new(1, 1, false);
    pub const ECP_CONN: crate::PointDef<Self, EcpConn> = crate::PointDef::new(2, 1, false);
    pub const ACT_WH: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(3, 4, false);
    pub const ACT_V_AH: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(7, 4, false);
    pub const ACT_V_ARH_Q1: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(11, 4, false);
    pub const ACT_V_ARH_Q2: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(15, 4, false);
    pub const ACT_V_ARH_Q3: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(19, 4, false);
    pub const ACT_V_ARH_Q4: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(23, 4, false);
    pub const V_AR_AVAL: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, false);
    pub const V_AR_AVAL_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(28, 1, false);
    pub const W_AVAL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(29, 1, false);
    pub const W_AVAL_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(30, 1, false);
    pub const ST_SET_LIM_MSK: crate::PointDef<Self, Option<StSetLimMsk>> =
        crate::PointDef::new(31, 2, false);
    pub const ST_ACT_CTL: crate::PointDef<Self, Option<StActCtl>> =
        crate::PointDef::new(33, 2, false);
    pub const TM_SRC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(35, 4, false);
    pub const TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(39, 2, false);
    pub const RT_ST: crate::PointDef<Self, Option<RtSt>> = crate::PointDef::new(41, 1, false);
    pub const RIS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(42, 1, false);
    pub const RIS_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(43, 1, false);
}

impl crate::Model for Model122 {
    const ID: u16 = 122;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            pv_conn: Self::PV_CONN.from_data(data)?,
            stor_conn: Self::STOR_CONN.from_data(data)?,
            ecp_conn: Self::ECP_CONN.from_data(data)?,
            act_wh: Self::ACT_WH.from_data(data)?,
            act_v_ah: Self::ACT_V_AH.from_data(data)?,
            act_v_arh_q1: Self::ACT_V_ARH_Q1.from_data(data)?,
            act_v_arh_q2: Self::ACT_V_ARH_Q2.from_data(data)?,
            act_v_arh_q3: Self::ACT_V_ARH_Q3.from_data(data)?,
            act_v_arh_q4: Self::ACT_V_ARH_Q4.from_data(data)?,
            v_ar_aval: Self::V_AR_AVAL.from_data(data)?,
            v_ar_aval_sf: Self::V_AR_AVAL_SF.from_data(data)?,
            w_aval: Self::W_AVAL.from_data(data)?,
            w_aval_sf: Self::W_AVAL_SF.from_data(data)?,
            st_set_lim_msk: Self::ST_SET_LIM_MSK.from_data(data)?,
            st_act_ctl: Self::ST_ACT_CTL.from_data(data)?,
            tm_src: Self::TM_SRC.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            rt_st: Self::RT_ST.from_data(data)?,
            ris: Self::RIS.from_data(data)?,
            ris_sf: Self::RIS_SF.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "PVConn\n\nPV inverter present/available status. Enumerated value."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct PvConn : u16 { # [doc = ""] const Connected = 1 ; # [doc = ""] const Available = 2 ; # [doc = ""] const Operating = 4 ; # [doc = ""] const Test = 8 ; } }
impl crate::Value for PvConn {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<PvConn> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(PvConn::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "StorConn\n\nStorage inverter present/available status. Enumerated value."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct StorConn : u16 { # [doc = ""] const Connected = 1 ; # [doc = ""] const Available = 2 ; # [doc = ""] const Operating = 4 ; # [doc = ""] const Test = 8 ; } }
impl crate::Value for StorConn {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<StorConn> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(StorConn::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "ECPConn\n\nECP connection status: disconnected=0  connected=1."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct EcpConn : u16 { # [doc = ""] const Connected = 1 ; } }
impl crate::Value for EcpConn {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EcpConn> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(EcpConn::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "StSetLimMsk\n\nBit Mask indicating setpoint limit(s) reached.\n\nNotes: Bits shall be automatically cleared on read."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct StSetLimMsk : u32 { # [doc = ""] const WMax = 1 ; # [doc = ""] const VaMax = 2 ; # [doc = ""] const VArAval = 4 ; # [doc = ""] const VArMaxQ1 = 8 ; # [doc = ""] const VArMaxQ2 = 16 ; # [doc = ""] const VArMaxQ3 = 32 ; # [doc = ""] const VArMaxQ4 = 64 ; # [doc = ""] const PfMinQ1 = 128 ; # [doc = ""] const PfMinQ2 = 256 ; # [doc = ""] const PfMinQ3 = 512 ; # [doc = ""] const PfMinQ4 = 1024 ; } }
impl crate::Value for StSetLimMsk {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<StSetLimMsk> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(StSetLimMsk::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "StActCtl\n\nBit Mask indicating which inverter controls are currently active."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct StActCtl : u32 { # [doc = ""] const FixedW = 1 ; # [doc = ""] const FixedVar = 2 ; # [doc = ""] const FixedPf = 4 ; # [doc = ""] const VoltVAr = 8 ; # [doc = ""] const FreqWattParam = 16 ; # [doc = ""] const FreqWattCurve = 32 ; # [doc = ""] const DynReactiveCurrent = 64 ; # [doc = ""] const Lvrt = 128 ; # [doc = ""] const Hvrt = 256 ; # [doc = ""] const WattPf = 512 ; # [doc = ""] const VoltWatt = 1024 ; # [doc = ""] const Scheduled = 4096 ; # [doc = ""] const Lfrt = 8192 ; # [doc = ""] const Hfrt = 16384 ; } }
impl crate::Value for StActCtl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<StActCtl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(StActCtl::from_bits_retain(value)))
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

bitflags::bitflags! { # [doc = "RtSt\n\nBit Mask indicating active ride-through status."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct RtSt : u16 { # [doc = ""] const LvrtActive = 1 ; # [doc = ""] const HvrtActive = 2 ; # [doc = ""] const LfrtActive = 4 ; # [doc = ""] const HfrtActive = 8 ; } }
impl crate::Value for RtSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<RtSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(RtSt::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
