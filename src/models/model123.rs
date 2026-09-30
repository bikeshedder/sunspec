//! Immediate Controls
/// Type alias for [`Controls`].
pub type Model123 = Controls;
/// Immediate Controls
///
/// Immediate Inverter Controls
///
/// Detail: Ref 3: 8.7.1.2, 8.7.2.2, 8.7.3.2
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Controls {
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
    /// Connection control.
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
    /// Throttle enable/disable control.
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
    /// Fixed power factor enable/disable control.
    pub out_pf_set_ena: OutPfSetEna,
    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    pub var_w_max_pct: Option<i16>,
    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    pub var_max_pct: Option<i16>,
    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    pub var_aval_pct: Option<i16>,
    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    pub var_pct_win_tms: Option<u16>,
    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    pub var_pct_rvrt_tms: Option<u16>,
    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub var_pct_rmp_tms: Option<u16>,
    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    pub var_pct_mod: Option<VarPctMod>,
    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    pub var_pct_ena: VarPctEna,
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
    pub var_pct_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Controls {
    pub const CONN_WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const CONN_RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const CONN: crate::Point<Self, Conn> = crate::Point::new(2, 1, true);
    pub const W_MAX_LIM_PCT: crate::Point<Self, u16> = crate::Point::new(3, 1, true);
    pub const W_MAX_LIM_PCT_WIN_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(4, 1, true);
    pub const W_MAX_LIM_PCT_RVRT_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(5, 1, true);
    pub const W_MAX_LIM_PCT_RMP_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(6, 1, true);
    pub const W_MAX_LIM_ENA: crate::Point<Self, WMaxLimEna> = crate::Point::new(7, 1, true);
    pub const OUT_PF_SET: crate::Point<Self, i16> = crate::Point::new(8, 1, true);
    pub const OUT_PF_SET_WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const OUT_PF_SET_RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const OUT_PF_SET_RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const OUT_PF_SET_ENA: crate::Point<Self, OutPfSetEna> = crate::Point::new(12, 1, true);
    pub const VAR_W_MAX_PCT: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, true);
    pub const VAR_MAX_PCT: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, true);
    pub const VAR_AVAL_PCT: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, true);
    pub const VAR_PCT_WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, true);
    pub const VAR_PCT_RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const VAR_PCT_RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const VAR_PCT_MOD: crate::Point<Self, Option<VarPctMod>> = crate::Point::new(19, 1, true);
    pub const VAR_PCT_ENA: crate::Point<Self, VarPctEna> = crate::Point::new(20, 1, true);
    pub const W_MAX_LIM_PCT_SF: crate::Point<Self, i16> = crate::Point::new(21, 1, false);
    pub const OUT_PF_SET_SF: crate::Point<Self, i16> = crate::Point::new(22, 1, false);
    pub const VAR_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
}
impl crate::sealed::Sealed for Controls {}
impl crate::Group for Controls {
    const LEN: u16 = 24;
}
impl Controls {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(24..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
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
                var_w_max_pct: Self::VAR_W_MAX_PCT.from_data(data)?,
                var_max_pct: Self::VAR_MAX_PCT.from_data(data)?,
                var_aval_pct: Self::VAR_AVAL_PCT.from_data(data)?,
                var_pct_win_tms: Self::VAR_PCT_WIN_TMS.from_data(data)?,
                var_pct_rvrt_tms: Self::VAR_PCT_RVRT_TMS.from_data(data)?,
                var_pct_rmp_tms: Self::VAR_PCT_RMP_TMS.from_data(data)?,
                var_pct_mod: Self::VAR_PCT_MOD.from_data(data)?,
                var_pct_ena: Self::VAR_PCT_ENA.from_data(data)?,
                w_max_lim_pct_sf: Self::W_MAX_LIM_PCT_SF.from_data(data)?,
                out_pf_set_sf: Self::OUT_PF_SET_SF.from_data(data)?,
                var_pct_sf: Self::VAR_PCT_SF.from_data(data)?,
            },
        ))
    }
}
/// Conn
///
/// Connection control.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Conn {
    #[allow(missing_docs)]
    Disconnect,
    #[allow(missing_docs)]
    Connect,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for Conn {}
impl crate::EnumValue for Conn {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disconnect,
            1 => Self::Connect,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disconnect => 0,
            Self::Connect => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Conn {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// WMaxLim_Ena
///
/// Throttle enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WMaxLimEna {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WMaxLimEna {}
impl crate::EnumValue for WMaxLimEna {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for WMaxLimEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// OutPFSet_Ena
///
/// Fixed power factor enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum OutPfSetEna {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for OutPfSetEna {}
impl crate::EnumValue for OutPfSetEna {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for OutPfSetEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// VArPct_Mod
///
/// VAR percent limit mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarPctMod {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    WMax,
    #[allow(missing_docs)]
    VarMax,
    #[allow(missing_docs)]
    VarAval,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarPctMod {}
impl crate::EnumValue for VarPctMod {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::WMax,
            2 => Self::VarMax,
            3 => Self::VarAval,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::WMax => 1,
            Self::VarMax => 2,
            Self::VarAval => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for VarPctMod {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// VArPct_Ena
///
/// Percent limit VAr enable/disable control.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarPctEna {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarPctEna {}
impl crate::EnumValue for VarPctEna {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for VarPctEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl From<Controls> for crate::AnyModel {
    fn from(model: Controls) -> Self {
        Self::M123(model)
    }
}
impl crate::Model for Controls {
    const ID: u16 = 123;
    const NAME: &'static str = "controls";
    const LABEL: &'static str = "Immediate Controls";
    fn addr(models: &crate::Models) -> Option<crate::ModelAddr<Self>> {
        models.m123
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
