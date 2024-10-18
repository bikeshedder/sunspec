//! Immediate Controls
/// Immediate Controls
///
/// Immediate Inverter Controls
///
/// Notes: Ref 3: 8.7.1.2, 8.7.2.2, 8.7.3.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model123 {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    pub conn_win_tms: Option<u16>,
    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    pub conn_rvrt_tms: Option<u16>,
    /// Conn
    ///
    /// Enumerated valued.  Connection control.
    pub conn: Conn,
    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    pub w_max_lim_pct: u16,
    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    pub w_max_lim_pct_win_tms: Option<u16>,
    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    pub w_max_lim_pct_rvrt_tms: Option<u16>,
    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub w_max_lim_pct_rmp_tms: Option<u16>,
    /// WMaxLim_Ena
    ///
    /// Enumerated valued.  Throttle enable/disable control.
    pub w_max_lim_ena: WMaxLimEna,
    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    pub out_pf_set: i16,
    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    pub out_pf_set_win_tms: Option<u16>,
    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    pub out_pf_set_rvrt_tms: Option<u16>,
    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub out_pf_set_rmp_tms: Option<u16>,
    /// OutPFSet_Ena
    ///
    /// Enumerated valued.  Fixed power factor enable/disable control.
    pub out_pf_set_ena: OutPfSetEna,
    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    pub v_ar_w_max_pct: Option<i16>,
    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    pub v_ar_max_pct: Option<i16>,
    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    pub v_ar_aval_pct: Option<i16>,
    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    pub v_ar_pct_win_tms: Option<u16>,
    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    pub v_ar_pct_rvrt_tms: Option<u16>,
    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub v_ar_pct_rmp_tms: Option<u16>,
    /// VArPct_Mod
    ///
    /// Enumerated value. VAR percent limit mode.
    pub v_ar_pct_mod: Option<VArPctMod>,
    /// VArPct_Ena
    ///
    /// Enumerated valued.  Percent limit VAr enable/disable control.
    pub v_ar_pct_ena: VArPctEna,
    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    pub w_max_lim_pct_sf: i16,
    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    pub out_pf_set_sf: i16,
    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    pub v_ar_pct_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model123 {
    pub const CONN_WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const CONN_RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const CONN: crate::PointDef<Self, Conn> = crate::PointDef::new(2, 1, true);
    pub const W_MAX_LIM_PCT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const W_MAX_LIM_PCT_WIN_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(4, 1, true);
    pub const W_MAX_LIM_PCT_RVRT_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(5, 1, true);
    pub const W_MAX_LIM_PCT_RMP_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(6, 1, true);
    pub const W_MAX_LIM_ENA: crate::PointDef<Self, WMaxLimEna> = crate::PointDef::new(7, 1, true);
    pub const OUT_PF_SET: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, true);
    pub const OUT_PF_SET_WIN_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(9, 1, true);
    pub const OUT_PF_SET_RVRT_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(10, 1, true);
    pub const OUT_PF_SET_RMP_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(11, 1, true);
    pub const OUT_PF_SET_ENA: crate::PointDef<Self, OutPfSetEna> =
        crate::PointDef::new(12, 1, true);
    pub const V_AR_W_MAX_PCT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(13, 1, true);
    pub const V_AR_MAX_PCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(14, 1, true);
    pub const V_AR_AVAL_PCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, true);
    pub const V_AR_PCT_WIN_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(16, 1, true);
    pub const V_AR_PCT_RVRT_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(17, 1, true);
    pub const V_AR_PCT_RMP_TMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(18, 1, true);
    pub const V_AR_PCT_MOD: crate::PointDef<Self, Option<VArPctMod>> =
        crate::PointDef::new(19, 1, true);
    pub const V_AR_PCT_ENA: crate::PointDef<Self, VArPctEna> = crate::PointDef::new(20, 1, true);
    pub const W_MAX_LIM_PCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const OUT_PF_SET_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const V_AR_PCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
}
impl crate::Model for Model123 {
    const ID: u16 = 123;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            conn_win_tms: Self::CONN_WIN_TMS.from_data(data)?,
            conn_rvrt_tms: Self::CONN_RVRT_TMS.from_data(data)?,
            conn: Self::CONN.from_data(data)?,
            w_max_lim_pct: Self::W_MAX_LIM_PCT.from_data(data)?,
            w_max_lim_pct_win_tms: Self::W_MAX_LIM_PCT_WIN_TMS.from_data(data)?,
            w_max_lim_pct_rvrt_tms: Self::W_MAX_LIM_PCT_RVRT_TMS.from_data(data)?,
            w_max_lim_pct_rmp_tms: Self::W_MAX_LIM_PCT_RMP_TMS.from_data(data)?,
            w_max_lim_ena: Self::W_MAX_LIM_ENA.from_data(data)?,
            out_pf_set: Self::OUT_PF_SET.from_data(data)?,
            out_pf_set_win_tms: Self::OUT_PF_SET_WIN_TMS.from_data(data)?,
            out_pf_set_rvrt_tms: Self::OUT_PF_SET_RVRT_TMS.from_data(data)?,
            out_pf_set_rmp_tms: Self::OUT_PF_SET_RMP_TMS.from_data(data)?,
            out_pf_set_ena: Self::OUT_PF_SET_ENA.from_data(data)?,
            v_ar_w_max_pct: Self::V_AR_W_MAX_PCT.from_data(data)?,
            v_ar_max_pct: Self::V_AR_MAX_PCT.from_data(data)?,
            v_ar_aval_pct: Self::V_AR_AVAL_PCT.from_data(data)?,
            v_ar_pct_win_tms: Self::V_AR_PCT_WIN_TMS.from_data(data)?,
            v_ar_pct_rvrt_tms: Self::V_AR_PCT_RVRT_TMS.from_data(data)?,
            v_ar_pct_rmp_tms: Self::V_AR_PCT_RMP_TMS.from_data(data)?,
            v_ar_pct_mod: Self::V_AR_PCT_MOD.from_data(data)?,
            v_ar_pct_ena: Self::V_AR_PCT_ENA.from_data(data)?,
            w_max_lim_pct_sf: Self::W_MAX_LIM_PCT_SF.from_data(data)?,
            out_pf_set_sf: Self::OUT_PF_SET_SF.from_data(data)?,
            v_ar_pct_sf: Self::V_AR_PCT_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m123
    }
}
/// Conn
///
/// Enumerated valued.  Connection control.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Conn {
    #[allow(missing_docs)]
    Disconnect = 0,
    #[allow(missing_docs)]
    Connect = 1,
}
impl crate::Value for Conn {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Conn> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Conn::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// WMaxLim_Ena
///
/// Enumerated valued.  Throttle enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WMaxLimEna {
    #[allow(missing_docs)]
    Disabled = 0,
    #[allow(missing_docs)]
    Enabled = 1,
}
impl crate::Value for WMaxLimEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WMaxLimEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WMaxLimEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// OutPFSet_Ena
///
/// Enumerated valued.  Fixed power factor enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum OutPfSetEna {
    #[allow(missing_docs)]
    Disabled = 0,
    #[allow(missing_docs)]
    Enabled = 1,
}
impl crate::Value for OutPfSetEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<OutPfSetEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                OutPfSetEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// VArPct_Mod
///
/// Enumerated value. VAR percent limit mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VArPctMod {
    #[allow(missing_docs)]
    None = 0,
    #[allow(missing_docs)]
    WMax = 1,
    #[allow(missing_docs)]
    VArMax = 2,
    #[allow(missing_docs)]
    VArAval = 3,
}
impl crate::Value for VArPctMod {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VArPctMod> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VArPctMod::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// VArPct_Ena
///
/// Enumerated valued.  Percent limit VAr enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VArPctEna {
    #[allow(missing_docs)]
    Disabled = 0,
    #[allow(missing_docs)]
    Enabled = 1,
}
impl crate::Value for VArPctEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VArPctEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VArPctEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
