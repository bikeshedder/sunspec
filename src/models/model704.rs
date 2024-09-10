//! DER AC Controls
/// DER AC Controls
///
/// DER AC controls model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model704 {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    ///
    /// Comments: Set Power Factor (when injecting active power)
    pub pfw_inj_ena: Option<PfwInjEna>,
    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    pub pfw_inj_ena_rvrt: Option<PfwInjEnaRvrt>,
    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    pub pfw_inj_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    pub pfw_inj_rvrt_rem: Option<u32>,
    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    ///
    /// Comments: Set Power Factor (when absorbing active power)
    pub pfw_abs_ena: Option<PfwAbsEna>,
    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    pub pfw_abs_ena_rvrt: Option<PfwAbsEnaRvrt>,
    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    pub pfw_abs_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    pub pfw_abs_rvrt_rem: Option<u32>,
    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    ///
    /// Comments: Limit Maximum Active Power Generation
    pub w_max_lim_pct_ena: Option<WMaxLimPctEna>,
    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    pub w_max_lim_pct: Option<u16>,
    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    pub w_max_lim_pct_rvrt: Option<u16>,
    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    pub w_max_lim_pct_ena_rvrt: Option<WMaxLimPctEnaRvrt>,
    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    pub w_max_lim_pct_rvrt_tms: Option<u32>,
    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    pub w_max_lim_pct_rvrt_rem: Option<u32>,
    /// Set Active Power Enable
    ///
    /// Set active power enable.
    ///
    /// Comments: Set Active Power Level (may be negative for charging)
    pub w_set_ena: Option<WSetEna>,
    /// Set Active Power Mode
    ///
    /// Set active power mode.
    pub w_set_mod: Option<WSetMod>,
    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    pub w_set: Option<i32>,
    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    pub w_set_rvrt: Option<i32>,
    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    pub w_set_pct: Option<i16>,
    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    pub w_set_pct_rvrt: Option<i16>,
    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    pub w_set_ena_rvrt: Option<WSetEnaRvrt>,
    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    pub w_set_rvrt_tms: Option<u32>,
    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    pub w_set_rvrt_rem: Option<u32>,
    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    ///
    /// Comments: Set Reactive Power Level
    pub var_set_ena: Option<VarSetEna>,
    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    pub var_set_mod: Option<VarSetMod>,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    pub var_set_pri: Option<VarSetPri>,
    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    pub var_set: Option<i32>,
    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    pub var_set_rvrt: Option<i32>,
    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    pub var_set_pct: Option<i16>,
    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    pub var_set_pct_rvrt: Option<i16>,
    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    pub var_set_ena_rvrt: Option<VarSetEnaRvrt>,
    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    pub var_set_rvrt_tms: Option<u32>,
    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    pub var_set_rvrt_rem: Option<u32>,
    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    ///
    /// Comments: Ramp Rate
    pub w_rmp: Option<u16>,
    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    pub w_rmp_ref: Option<WRmpRef>,
    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    pub var_rmp: Option<u16>,
    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    pub anti_isl_ena: Option<AntiIslEna>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    ///
    /// Comments: Scale Factors
    pub pf_sf: Option<i16>,
    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    pub w_max_lim_pct_sf: Option<i16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    pub w_set_sf: Option<i16>,
    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    pub w_set_pct_sf: Option<i16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    pub var_set_sf: Option<i16>,
    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    pub var_set_pct_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model704 {
    pub const PFW_INJ_ENA: crate::PointDef<Self, Option<PfwInjEna>> =
        crate::PointDef::new(0, 1, true);
    pub const PFW_INJ_ENA_RVRT: crate::PointDef<Self, Option<PfwInjEnaRvrt>> =
        crate::PointDef::new(1, 1, true);
    pub const PFW_INJ_RVRT_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(2, 2, true);
    pub const PFW_INJ_RVRT_REM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(4, 2, false);
    pub const PFW_ABS_ENA: crate::PointDef<Self, Option<PfwAbsEna>> =
        crate::PointDef::new(6, 1, true);
    pub const PFW_ABS_ENA_RVRT: crate::PointDef<Self, Option<PfwAbsEnaRvrt>> =
        crate::PointDef::new(7, 1, true);
    pub const PFW_ABS_RVRT_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(8, 2, true);
    pub const PFW_ABS_RVRT_REM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(10, 2, false);
    pub const W_MAX_LIM_PCT_ENA: crate::PointDef<Self, Option<WMaxLimPctEna>> =
        crate::PointDef::new(12, 1, true);
    pub const W_MAX_LIM_PCT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, true);
    pub const W_MAX_LIM_PCT_RVRT: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(14, 1, true);
    pub const W_MAX_LIM_PCT_ENA_RVRT: crate::PointDef<Self, Option<WMaxLimPctEnaRvrt>> =
        crate::PointDef::new(15, 1, true);
    pub const W_MAX_LIM_PCT_RVRT_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(16, 2, true);
    pub const W_MAX_LIM_PCT_RVRT_REM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(18, 2, false);
    pub const W_SET_ENA: crate::PointDef<Self, Option<WSetEna>> = crate::PointDef::new(20, 1, true);
    pub const W_SET_MOD: crate::PointDef<Self, Option<WSetMod>> = crate::PointDef::new(21, 1, true);
    pub const W_SET: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(22, 2, true);
    pub const W_SET_RVRT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(24, 2, true);
    pub const W_SET_PCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(26, 1, true);
    pub const W_SET_PCT_RVRT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(27, 1, true);
    pub const W_SET_ENA_RVRT: crate::PointDef<Self, Option<WSetEnaRvrt>> =
        crate::PointDef::new(28, 1, true);
    pub const W_SET_RVRT_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(29, 2, true);
    pub const W_SET_RVRT_REM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(31, 2, false);
    pub const VAR_SET_ENA: crate::PointDef<Self, Option<VarSetEna>> =
        crate::PointDef::new(33, 1, true);
    pub const VAR_SET_MOD: crate::PointDef<Self, Option<VarSetMod>> =
        crate::PointDef::new(34, 1, true);
    pub const VAR_SET_PRI: crate::PointDef<Self, Option<VarSetPri>> =
        crate::PointDef::new(35, 1, true);
    pub const VAR_SET: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(36, 2, true);
    pub const VAR_SET_RVRT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(38, 2, true);
    pub const VAR_SET_PCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(40, 1, true);
    pub const VAR_SET_PCT_RVRT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(41, 1, true);
    pub const VAR_SET_ENA_RVRT: crate::PointDef<Self, Option<VarSetEnaRvrt>> =
        crate::PointDef::new(42, 1, true);
    pub const VAR_SET_RVRT_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(43, 2, true);
    pub const VAR_SET_RVRT_REM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(45, 2, false);
    pub const W_RMP: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(47, 1, true);
    pub const W_RMP_REF: crate::PointDef<Self, Option<WRmpRef>> = crate::PointDef::new(48, 1, true);
    pub const VAR_RMP: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(49, 1, true);
    pub const ANTI_ISL_ENA: crate::PointDef<Self, Option<AntiIslEna>> =
        crate::PointDef::new(50, 1, true);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(51, 1, false);
    pub const W_MAX_LIM_PCT_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(52, 1, false);
    pub const W_SET_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(53, 1, false);
    pub const W_SET_PCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(54, 1, false);
    pub const VAR_SET_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(55, 1, false);
    pub const VAR_SET_PCT_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(56, 1, false);
}
impl crate::Model for Model704 {
    const ID: u16 = 704;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            pfw_inj_ena: Self::PFW_INJ_ENA.from_data(data)?,
            pfw_inj_ena_rvrt: Self::PFW_INJ_ENA_RVRT.from_data(data)?,
            pfw_inj_rvrt_tms: Self::PFW_INJ_RVRT_TMS.from_data(data)?,
            pfw_inj_rvrt_rem: Self::PFW_INJ_RVRT_REM.from_data(data)?,
            pfw_abs_ena: Self::PFW_ABS_ENA.from_data(data)?,
            pfw_abs_ena_rvrt: Self::PFW_ABS_ENA_RVRT.from_data(data)?,
            pfw_abs_rvrt_tms: Self::PFW_ABS_RVRT_TMS.from_data(data)?,
            pfw_abs_rvrt_rem: Self::PFW_ABS_RVRT_REM.from_data(data)?,
            w_max_lim_pct_ena: Self::W_MAX_LIM_PCT_ENA.from_data(data)?,
            w_max_lim_pct: Self::W_MAX_LIM_PCT.from_data(data)?,
            w_max_lim_pct_rvrt: Self::W_MAX_LIM_PCT_RVRT.from_data(data)?,
            w_max_lim_pct_ena_rvrt: Self::W_MAX_LIM_PCT_ENA_RVRT.from_data(data)?,
            w_max_lim_pct_rvrt_tms: Self::W_MAX_LIM_PCT_RVRT_TMS.from_data(data)?,
            w_max_lim_pct_rvrt_rem: Self::W_MAX_LIM_PCT_RVRT_REM.from_data(data)?,
            w_set_ena: Self::W_SET_ENA.from_data(data)?,
            w_set_mod: Self::W_SET_MOD.from_data(data)?,
            w_set: Self::W_SET.from_data(data)?,
            w_set_rvrt: Self::W_SET_RVRT.from_data(data)?,
            w_set_pct: Self::W_SET_PCT.from_data(data)?,
            w_set_pct_rvrt: Self::W_SET_PCT_RVRT.from_data(data)?,
            w_set_ena_rvrt: Self::W_SET_ENA_RVRT.from_data(data)?,
            w_set_rvrt_tms: Self::W_SET_RVRT_TMS.from_data(data)?,
            w_set_rvrt_rem: Self::W_SET_RVRT_REM.from_data(data)?,
            var_set_ena: Self::VAR_SET_ENA.from_data(data)?,
            var_set_mod: Self::VAR_SET_MOD.from_data(data)?,
            var_set_pri: Self::VAR_SET_PRI.from_data(data)?,
            var_set: Self::VAR_SET.from_data(data)?,
            var_set_rvrt: Self::VAR_SET_RVRT.from_data(data)?,
            var_set_pct: Self::VAR_SET_PCT.from_data(data)?,
            var_set_pct_rvrt: Self::VAR_SET_PCT_RVRT.from_data(data)?,
            var_set_ena_rvrt: Self::VAR_SET_ENA_RVRT.from_data(data)?,
            var_set_rvrt_tms: Self::VAR_SET_RVRT_TMS.from_data(data)?,
            var_set_rvrt_rem: Self::VAR_SET_RVRT_REM.from_data(data)?,
            w_rmp: Self::W_RMP.from_data(data)?,
            w_rmp_ref: Self::W_RMP_REF.from_data(data)?,
            var_rmp: Self::VAR_RMP.from_data(data)?,
            anti_isl_ena: Self::ANTI_ISL_ENA.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            w_max_lim_pct_sf: Self::W_MAX_LIM_PCT_SF.from_data(data)?,
            w_set_sf: Self::W_SET_SF.from_data(data)?,
            w_set_pct_sf: Self::W_SET_PCT_SF.from_data(data)?,
            var_set_sf: Self::VAR_SET_SF.from_data(data)?,
            var_set_pct_sf: Self::VAR_SET_PCT_SF.from_data(data)?,
        })
    }
}
/// Power Factor Enable (W Inj) Enable
///
/// Power factor enable when injecting active power.
///
/// Comments: Set Power Factor (when injecting active power)
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum PfwInjEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for PfwInjEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<PfwInjEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                PfwInjEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Power Factor Reversion Enable (W Inj)
///
/// Power factor reversion timer when injecting active power enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum PfwInjEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for PfwInjEnaRvrt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<PfwInjEnaRvrt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                PfwInjEnaRvrt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Power Factor Enable (W Abs) Enable
///
/// Power factor enable when absorbing active power.
///
/// Comments: Set Power Factor (when absorbing active power)
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum PfwAbsEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for PfwAbsEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<PfwAbsEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                PfwAbsEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Power Factor Reversion Enable (W Abs)
///
/// Power factor reversion timer when absorbing active power enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum PfwAbsEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for PfwAbsEnaRvrt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<PfwAbsEnaRvrt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                PfwAbsEnaRvrt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Limit Max Power Pct Enable
///
/// Limit maximum active power percent enable.
///
/// Comments: Limit Maximum Active Power Generation
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WMaxLimPctEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for WMaxLimPctEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WMaxLimPctEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WMaxLimPctEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Reversion Limit Max Power Pct Enable
///
/// Reversion limit maximum active power percent value enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WMaxLimPctEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for WMaxLimPctEnaRvrt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WMaxLimPctEnaRvrt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WMaxLimPctEnaRvrt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Set Active Power Enable
///
/// Set active power enable.
///
/// Comments: Set Active Power Level (may be negative for charging)
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for WSetEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WSetEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WSetEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Set Active Power Mode
///
/// Set active power mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WSetMod {
    /// Active Power As Max Percent
    ///
    /// Active power setting is percentage of maximum active power.
    WMaxPct = 0,
    /// Active Power As Watts
    ///
    /// Active power setting is in watts.
    Watts = 1,
}
impl crate::Value for WSetMod {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WSetMod> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WSetMod::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Reversion Active Power Enable
///
/// Reversion active power function enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for WSetEnaRvrt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WSetEnaRvrt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WSetEnaRvrt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Set Reactive Power Enable
///
/// Set reactive power enable.
///
/// Comments: Set Reactive Power Level
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VarSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for VarSetEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VarSetEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VarSetEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Set Reactive Power Mode
///
/// Set reactive power mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VarSetMod {
    /// Reactive Power As Watt Max Pct
    ///
    /// Reactive power setting is percent of maximum active power.
    WMaxPct = 0,
    /// Reactive Power As Var Max Pct
    ///
    /// Reactive power setting is percent of maximum reactive power.
    VarMaxPct = 1,
    /// Reactive Power As Var Avail Pct
    ///
    /// Reactive power setting is percent of available reactive  power.
    VarAvailPct = 2,
    /// Reactive Power As VA Max Pct
    ///
    /// Reactive power setting is percent of maximum apparent power.
    VaMaxPct = 3,
    /// Reactive Power As Vars
    ///
    /// Reactive power is in vars.
    Vars = 4,
}
impl crate::Value for VarSetMod {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VarSetMod> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VarSetMod::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Reactive Power Priority
///
/// Reactive power priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VarSetPri {
    /// Active Power Priority
    ///
    /// Active power priority.
    Active = 0,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    Reactive = 1,
    /// Vendor Power Priority
    ///
    /// Power priority is vendor specific mode.
    Vendor = 2,
}
impl crate::Value for VarSetPri {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VarSetPri> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VarSetPri::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Reversion Reactive Power Enable
///
/// Reversion reactive power function enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VarSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for VarSetEnaRvrt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VarSetEnaRvrt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VarSetEnaRvrt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Normal Ramp Rate Reference
///
/// Ramp rate reference unit for increases in active power or current during normal generation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum WRmpRef {
    /// Max Current Ramp
    ///
    /// Ramp based on percent of max current per second.
    AMax = 0,
    /// Max Active Power Ramp
    ///
    /// Ramp based on percent of max active power per second.
    WMax = 1,
}
impl crate::Value for WRmpRef {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<WRmpRef> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                WRmpRef::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Anti-Islanding Enable
///
/// Anti-islanding enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum AntiIslEna {
    /// Disabled
    ///
    /// Anti-islanding is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Anti-islanding is enabled.
    Enabled = 1,
}
impl crate::Value for AntiIslEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<AntiIslEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                AntiIslEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
