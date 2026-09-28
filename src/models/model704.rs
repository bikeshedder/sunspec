//! DER AC Controls
/// Type alias for [`DerCtlAc`].
pub type Model704 = DerCtlAc;
/// DER AC Controls
///
/// DER AC controls model.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerCtlAc {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    ///
    /// Comments: Set Power Factor (when injecting active power)
    pub pf_w_inj_ena: Option<PfWInjEna>,
    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    pub pf_w_inj_ena_rvrt: Option<PfWInjEnaRvrt>,
    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    pub pf_w_inj_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    pub pf_w_inj_rvrt_rem: Option<u32>,
    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    ///
    /// Comments: Set Power Factor (when absorbing active power)
    pub pf_w_abs_ena: Option<PfWAbsEna>,
    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    pub pf_w_abs_ena_rvrt: Option<PfWAbsEnaRvrt>,
    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    pub pf_w_abs_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    pub pf_w_abs_rvrt_rem: Option<u32>,
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
    /// Power Factor (W Inj)
    ///
    /// Power factor setpoint when injecting active power.
    ///
    /// Comments: Power Factor Settings
    pub pf_w_inj: PfWInj,
    /// Reversion Power Factor (W Inj)
    ///
    /// Reversion power factor setpoint when injecting active power.
    pub pf_w_inj_rvrt: PfWInjRvrt,
    /// Power Factor (W Abs)
    ///
    /// Power factor setpoint when absorbing active power.
    pub pf_w_abs: PfWAbs,
    /// Reversion Power Factor (W Abs)
    ///
    /// Reversion power factor setpoint when absorbing active power.
    pub pf_w_abs_rvrt: PfWAbsRvrt,
}
#[allow(missing_docs)]
impl DerCtlAc {
    pub const PF_W_INJ_ENA: crate::Point<Self, Option<PfWInjEna>> = crate::Point::new(0, 1, true);
    pub const PF_W_INJ_ENA_RVRT: crate::Point<Self, Option<PfWInjEnaRvrt>> =
        crate::Point::new(1, 1, true);
    pub const PF_W_INJ_RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(2, 2, true);
    pub const PF_W_INJ_RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(4, 2, false);
    pub const PF_W_ABS_ENA: crate::Point<Self, Option<PfWAbsEna>> = crate::Point::new(6, 1, true);
    pub const PF_W_ABS_ENA_RVRT: crate::Point<Self, Option<PfWAbsEnaRvrt>> =
        crate::Point::new(7, 1, true);
    pub const PF_W_ABS_RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(8, 2, true);
    pub const PF_W_ABS_RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2, false);
    pub const W_MAX_LIM_PCT_ENA: crate::Point<Self, Option<WMaxLimPctEna>> =
        crate::Point::new(12, 1, true);
    pub const W_MAX_LIM_PCT: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const W_MAX_LIM_PCT_RVRT: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const W_MAX_LIM_PCT_ENA_RVRT: crate::Point<Self, Option<WMaxLimPctEnaRvrt>> =
        crate::Point::new(15, 1, true);
    pub const W_MAX_LIM_PCT_RVRT_TMS: crate::Point<Self, Option<u32>> =
        crate::Point::new(16, 2, true);
    pub const W_MAX_LIM_PCT_RVRT_REM: crate::Point<Self, Option<u32>> =
        crate::Point::new(18, 2, false);
    pub const W_SET_ENA: crate::Point<Self, Option<WSetEna>> = crate::Point::new(20, 1, true);
    pub const W_SET_MOD: crate::Point<Self, Option<WSetMod>> = crate::Point::new(21, 1, true);
    pub const W_SET: crate::Point<Self, Option<i32>> = crate::Point::new(22, 2, true);
    pub const W_SET_RVRT: crate::Point<Self, Option<i32>> = crate::Point::new(24, 2, true);
    pub const W_SET_PCT: crate::Point<Self, Option<i16>> = crate::Point::new(26, 1, true);
    pub const W_SET_PCT_RVRT: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, true);
    pub const W_SET_ENA_RVRT: crate::Point<Self, Option<WSetEnaRvrt>> =
        crate::Point::new(28, 1, true);
    pub const W_SET_RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(29, 2, true);
    pub const W_SET_RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(31, 2, false);
    pub const VAR_SET_ENA: crate::Point<Self, Option<VarSetEna>> = crate::Point::new(33, 1, true);
    pub const VAR_SET_MOD: crate::Point<Self, Option<VarSetMod>> = crate::Point::new(34, 1, true);
    pub const VAR_SET_PRI: crate::Point<Self, Option<VarSetPri>> = crate::Point::new(35, 1, true);
    pub const VAR_SET: crate::Point<Self, Option<i32>> = crate::Point::new(36, 2, true);
    pub const VAR_SET_RVRT: crate::Point<Self, Option<i32>> = crate::Point::new(38, 2, true);
    pub const VAR_SET_PCT: crate::Point<Self, Option<i16>> = crate::Point::new(40, 1, true);
    pub const VAR_SET_PCT_RVRT: crate::Point<Self, Option<i16>> = crate::Point::new(41, 1, true);
    pub const VAR_SET_ENA_RVRT: crate::Point<Self, Option<VarSetEnaRvrt>> =
        crate::Point::new(42, 1, true);
    pub const VAR_SET_RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(43, 2, true);
    pub const VAR_SET_RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(45, 2, false);
    pub const W_RMP: crate::Point<Self, Option<u16>> = crate::Point::new(47, 1, true);
    pub const W_RMP_REF: crate::Point<Self, Option<WRmpRef>> = crate::Point::new(48, 1, true);
    pub const VAR_RMP: crate::Point<Self, Option<u16>> = crate::Point::new(49, 1, true);
    pub const ANTI_ISL_ENA: crate::Point<Self, Option<AntiIslEna>> = crate::Point::new(50, 1, true);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(51, 1, false);
    pub const W_MAX_LIM_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(52, 1, false);
    pub const W_SET_SF: crate::Point<Self, Option<i16>> = crate::Point::new(53, 1, false);
    pub const W_SET_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(54, 1, false);
    pub const VAR_SET_SF: crate::Point<Self, Option<i16>> = crate::Point::new(55, 1, false);
    pub const VAR_SET_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(56, 1, false);
}
impl crate::sealed::Sealed for DerCtlAc {}
impl crate::Group for DerCtlAc {
    const LEN: u16 = 57;
}
impl DerCtlAc {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, pf_w_inj) = PfWInj::parse_group(nested_data)?;
        let (nested_data, pf_w_inj_rvrt) = PfWInjRvrt::parse_group(nested_data)?;
        let (nested_data, pf_w_abs) = PfWAbs::parse_group(nested_data)?;
        let (nested_data, pf_w_abs_rvrt) = PfWAbsRvrt::parse_group(nested_data)?;
        Ok((
            nested_data,
            Self {
                pf_w_inj_ena: Self::PF_W_INJ_ENA.from_data(data)?,
                pf_w_inj_ena_rvrt: Self::PF_W_INJ_ENA_RVRT.from_data(data)?,
                pf_w_inj_rvrt_tms: Self::PF_W_INJ_RVRT_TMS.from_data(data)?,
                pf_w_inj_rvrt_rem: Self::PF_W_INJ_RVRT_REM.from_data(data)?,
                pf_w_abs_ena: Self::PF_W_ABS_ENA.from_data(data)?,
                pf_w_abs_ena_rvrt: Self::PF_W_ABS_ENA_RVRT.from_data(data)?,
                pf_w_abs_rvrt_tms: Self::PF_W_ABS_RVRT_TMS.from_data(data)?,
                pf_w_abs_rvrt_rem: Self::PF_W_ABS_RVRT_REM.from_data(data)?,
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
                pf_w_inj,
                pf_w_inj_rvrt,
                pf_w_abs,
                pf_w_abs_rvrt,
            },
        ))
    }
}
/// Power Factor Enable (W Inj) Enable
///
/// Power factor enable when injecting active power.
///
/// Comments: Set Power Factor (when injecting active power)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWInjEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWInjEna {}
impl crate::EnumValue for PfWInjEna {
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
impl crate::FixedSize for PfWInjEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Factor Reversion Enable (W Inj)
///
/// Power factor reversion timer when injecting active power enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWInjEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWInjEnaRvrt {}
impl crate::EnumValue for PfWInjEnaRvrt {
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
impl crate::FixedSize for PfWInjEnaRvrt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Factor Enable (W Abs) Enable
///
/// Power factor enable when absorbing active power.
///
/// Comments: Set Power Factor (when absorbing active power)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWAbsEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWAbsEna {}
impl crate::EnumValue for PfWAbsEna {
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
impl crate::FixedSize for PfWAbsEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Factor Reversion Enable (W Abs)
///
/// Power factor reversion timer when absorbing active power enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWAbsEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWAbsEnaRvrt {}
impl crate::EnumValue for PfWAbsEnaRvrt {
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
impl crate::FixedSize for PfWAbsEnaRvrt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Limit Max Power Pct Enable
///
/// Limit maximum active power percent enable.
///
/// Comments: Limit Maximum Active Power Generation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WMaxLimPctEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WMaxLimPctEna {}
impl crate::EnumValue for WMaxLimPctEna {
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
impl crate::FixedSize for WMaxLimPctEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reversion Limit Max Power Pct Enable
///
/// Reversion limit maximum active power percent value enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WMaxLimPctEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WMaxLimPctEnaRvrt {}
impl crate::EnumValue for WMaxLimPctEnaRvrt {
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
impl crate::FixedSize for WMaxLimPctEnaRvrt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Active Power Enable
///
/// Set active power enable.
///
/// Comments: Set Active Power Level (may be negative for charging)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WSetEna {}
impl crate::EnumValue for WSetEna {
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
impl crate::FixedSize for WSetEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Active Power Mode
///
/// Set active power mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WSetMod {
    /// Active Power As Max Percent
    ///
    /// Active power setting is percentage of maximum active power.
    WMaxPct,
    /// Active Power As Watts
    ///
    /// Active power setting is in watts.
    Watts,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WSetMod {}
impl crate::EnumValue for WSetMod {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::WMaxPct,
            1 => Self::Watts,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMaxPct => 0,
            Self::Watts => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for WSetMod {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reversion Active Power Enable
///
/// Reversion active power function enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WSetEnaRvrt {}
impl crate::EnumValue for WSetEnaRvrt {
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
impl crate::FixedSize for WSetEnaRvrt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Reactive Power Enable
///
/// Set reactive power enable.
///
/// Comments: Set Reactive Power Level
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarSetEna {}
impl crate::EnumValue for VarSetEna {
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
impl crate::FixedSize for VarSetEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Reactive Power Mode
///
/// Set reactive power mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarSetMod {
    /// Reactive Power As Watt Max Pct
    ///
    /// Reactive power setting is percent of maximum active power.
    WMaxPct,
    /// Reactive Power As Var Max Pct
    ///
    /// Reactive power setting is percent of maximum reactive power.
    VarMaxPct,
    /// Reactive Power As Var Avail Pct
    ///
    /// Reactive power setting is percent of available reactive  power.
    VarAvailPct,
    /// Reactive Power As VA Max Pct
    ///
    /// Reactive power setting is percent of maximum apparent power.
    VaMaxPct,
    /// Reactive Power As Vars
    ///
    /// Reactive power is in vars.
    Vars,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarSetMod {}
impl crate::EnumValue for VarSetMod {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::WMaxPct,
            1 => Self::VarMaxPct,
            2 => Self::VarAvailPct,
            3 => Self::VaMaxPct,
            4 => Self::Vars,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMaxPct => 0,
            Self::VarMaxPct => 1,
            Self::VarAvailPct => 2,
            Self::VaMaxPct => 3,
            Self::Vars => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for VarSetMod {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reactive Power Priority
///
/// Reactive power priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarSetPri {
    /// Active Power Priority
    ///
    /// Active power priority.
    Active,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    Reactive,
    /// Vendor Power Priority
    ///
    /// Power priority is vendor specific mode.
    Vendor,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarSetPri {}
impl crate::EnumValue for VarSetPri {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::Reactive,
            2 => Self::Vendor,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Active => 0,
            Self::Reactive => 1,
            Self::Vendor => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for VarSetPri {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reversion Reactive Power Enable
///
/// Reversion reactive power function enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum VarSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for VarSetEnaRvrt {}
impl crate::EnumValue for VarSetEnaRvrt {
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
impl crate::FixedSize for VarSetEnaRvrt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Normal Ramp Rate Reference
///
/// Ramp rate reference unit for increases in active power or current during normal generation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WRmpRef {
    /// Max Current Ramp
    ///
    /// Ramp based on percent of max current per second.
    AMax,
    /// Max Active Power Ramp
    ///
    /// Ramp based on percent of max active power per second.
    WMax,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for WRmpRef {}
impl crate::EnumValue for WRmpRef {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::AMax,
            1 => Self::WMax,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::AMax => 0,
            Self::WMax => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for WRmpRef {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Anti-Islanding Enable
///
/// Anti-islanding enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AntiIslEna {
    /// Disabled
    ///
    /// Anti-islanding is disabled.
    Disabled,
    /// Enabled
    ///
    /// Anti-islanding is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for AntiIslEna {}
impl crate::EnumValue for AntiIslEna {
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
impl crate::FixedSize for AntiIslEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Factor (W Inj)
///
/// Power factor setpoint when injecting active power.
///
/// Comments: Power Factor Settings
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PfWInj {
    /// Power Factor (W Inj)
    ///
    /// Power factor setpoint when injecting active power.
    pub pf: Option<u16>,
    /// Power Factor Excitation (W Inj)
    ///
    /// Power factor excitation setpoint when injecting active power.
    pub ext: Option<PfWInjExt>,
}
#[allow(missing_docs)]
impl PfWInj {
    pub const PF: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const EXT: crate::Point<Self, Option<PfWInjExt>> = crate::Point::new(1, 1, true);
}
impl crate::sealed::Sealed for PfWInj {}
impl crate::Group for PfWInj {
    const LEN: u16 = 2;
}
impl PfWInj {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                pf: Self::PF.from_data(data)?,
                ext: Self::EXT.from_data(data)?,
            },
        ))
    }
}
/// Power Factor Excitation (W Inj)
///
/// Power factor excitation setpoint when injecting active power.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWInjExt {
    /// Over-Excited
    ///
    /// Power factor over-excited excitation.
    OverExcited,
    /// Under-Excited
    ///
    /// Power factor under-excited excitation.
    UnderExcited,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWInjExt {}
impl crate::EnumValue for PfWInjExt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::OverExcited,
            1 => Self::UnderExcited,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::OverExcited => 0,
            Self::UnderExcited => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PfWInjExt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reversion Power Factor (W Inj)
///
/// Reversion power factor setpoint when injecting active power.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PfWInjRvrt {
    /// Reversion Power Factor (W Inj)
    ///
    /// Reversion power factor setpoint when injecting active power.
    pub pf: Option<u16>,
    /// Reversion PF Excitation (W Inj)
    ///
    /// Reversion power factor excitation setpoint when injecting active power.
    pub ext: Option<PfWInjRvrtExt>,
}
#[allow(missing_docs)]
impl PfWInjRvrt {
    pub const PF: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const EXT: crate::Point<Self, Option<PfWInjRvrtExt>> = crate::Point::new(1, 1, true);
}
impl crate::sealed::Sealed for PfWInjRvrt {}
impl crate::Group for PfWInjRvrt {
    const LEN: u16 = 2;
}
impl PfWInjRvrt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                pf: Self::PF.from_data(data)?,
                ext: Self::EXT.from_data(data)?,
            },
        ))
    }
}
/// Reversion PF Excitation (W Inj)
///
/// Reversion power factor excitation setpoint when injecting active power.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWInjRvrtExt {
    /// Over-Excited
    ///
    /// Power factor over-excited excitation.
    OverExcited,
    /// Under-Excited
    ///
    /// Power factor under-excited excitation.
    UnderExcited,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWInjRvrtExt {}
impl crate::EnumValue for PfWInjRvrtExt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::OverExcited,
            1 => Self::UnderExcited,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::OverExcited => 0,
            Self::UnderExcited => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PfWInjRvrtExt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Factor (W Abs)
///
/// Power factor setpoint when absorbing active power.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PfWAbs {
    /// Power Factor (W Abs)
    ///
    /// Power factor setpoint when absorbing active power.
    pub pf: Option<u16>,
    /// Power Factor Excitation (W Abs)
    ///
    /// Power factor excitation setpoint when absorbing active power.
    pub ext: Option<PfWAbsExt>,
}
#[allow(missing_docs)]
impl PfWAbs {
    pub const PF: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const EXT: crate::Point<Self, Option<PfWAbsExt>> = crate::Point::new(1, 1, true);
}
impl crate::sealed::Sealed for PfWAbs {}
impl crate::Group for PfWAbs {
    const LEN: u16 = 2;
}
impl PfWAbs {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                pf: Self::PF.from_data(data)?,
                ext: Self::EXT.from_data(data)?,
            },
        ))
    }
}
/// Power Factor Excitation (W Abs)
///
/// Power factor excitation setpoint when absorbing active power.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWAbsExt {
    /// Over-Excited
    ///
    /// Power factor over-excited excitation.
    OverExcited,
    /// Under-Excited
    ///
    /// Power factor under-excited excitation.
    UnderExcited,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWAbsExt {}
impl crate::EnumValue for PfWAbsExt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::OverExcited,
            1 => Self::UnderExcited,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::OverExcited => 0,
            Self::UnderExcited => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PfWAbsExt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reversion Power Factor (W Abs)
///
/// Reversion power factor setpoint when absorbing active power.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PfWAbsRvrt {
    /// Reversion Power Factor (W Abs)
    ///
    /// Reversion power factor setpoint when absorbing active power.
    pub pf: Option<u16>,
    /// Reversion PF Excitation (W Abs)
    ///
    /// Reversion power factor excitation setpoint when absorbing active power.
    pub ext: Option<PfWAbsRvrtExt>,
}
#[allow(missing_docs)]
impl PfWAbsRvrt {
    pub const PF: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const EXT: crate::Point<Self, Option<PfWAbsRvrtExt>> = crate::Point::new(1, 1, true);
}
impl crate::sealed::Sealed for PfWAbsRvrt {}
impl crate::Group for PfWAbsRvrt {
    const LEN: u16 = 2;
}
impl PfWAbsRvrt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                pf: Self::PF.from_data(data)?,
                ext: Self::EXT.from_data(data)?,
            },
        ))
    }
}
/// Reversion PF Excitation (W Abs)
///
/// Reversion power factor excitation setpoint when absorbing active power.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PfWAbsRvrtExt {
    /// Over-Excited
    ///
    /// Power factor over-excited excitation.
    OverExcited,
    /// Under-Excited
    ///
    /// Power factor under-excited excitation.
    UnderExcited,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for PfWAbsRvrtExt {}
impl crate::EnumValue for PfWAbsRvrtExt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::OverExcited,
            1 => Self::UnderExcited,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::OverExcited => 0,
            Self::UnderExcited => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PfWAbsRvrtExt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl From<DerCtlAc> for crate::AnyModel {
    fn from(model: DerCtlAc) -> Self {
        Self::M704(model)
    }
}
impl crate::Model for DerCtlAc {
    const ID: u16 = 704;
    const NAME: &'static str = "DERCtlAC";
    const LABEL: &'static str = "DER AC Controls";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m704
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
