//! DER Watt-Var
/// Type alias for [`DerWattVar`].
pub type Model712 = DerWattVar;
struct Counts {
    n_pt: u16,
    n_crv: u16,
}
/// DER Watt-Var
///
/// DER Watt-Var model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerWattVar {
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    pub ena: Ena,
    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    pub adpt_crv_req: u16,
    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    pub adpt_crv_rslt: AdptCrvRslt,
    /// Number Of Points
    ///
    /// Number of curve points supported.
    pub n_pt: u16,
    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    pub n_crv: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrt_tms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    pub rvrt_rem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    pub rvrt_crv: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    pub w_sf: i16,
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    pub dept_ref_sf: i16,
    /// Stored Curves
    ///
    /// Stored curve sets.
    ///
    /// Comments: Stored curve sets - Number of curve sets contained in NCrv - The first set is read-only and indicates the current settings.
    pub crv: Vec<Crv>,
}
#[allow(missing_docs)]
impl DerWattVar {
    pub const ENA: crate::Point<Self, Ena> = crate::Point::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::Point<Self, AdptCrvRslt> = crate::Point::new(2, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(5, 2, true);
    pub const RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, false);
    pub const RVRT_CRV: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(10, 1, false);
    pub const DEPT_REF_SF: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
}
impl crate::Group for DerWattVar {
    const LEN: u16 = 12;
}
impl DerWattVar {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            n_pt: Self::N_PT.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
        };
        let (nested_data, crv) = Crv::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                ena: Self::ENA.from_data(data)?,
                adpt_crv_req: Self::ADPT_CRV_REQ.from_data(data)?,
                adpt_crv_rslt: Self::ADPT_CRV_RSLT.from_data(data)?,
                n_pt: Self::N_PT.from_data(data)?,
                n_crv: Self::N_CRV.from_data(data)?,
                rvrt_tms: Self::RVRT_TMS.from_data(data)?,
                rvrt_rem: Self::RVRT_REM.from_data(data)?,
                rvrt_crv: Self::RVRT_CRV.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
                crv,
            },
        ))
    }
}
/// DER Watt-Var Module Enable
///
/// DER Watt-Var control enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Ena {
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
impl crate::EnumValue for Ena {
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
impl crate::FixedSize for Ena {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Active Curve Result
///
/// Result of last set active curve operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AdptCrvRslt {
    /// Update In Progress
    ///
    /// Curve update in progress.
    InProgress,
    /// Update Complete
    ///
    /// Curve update completed successfully.
    Completed,
    /// Update Failed
    ///
    /// Curve update failed.
    Failed,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for AdptCrvRslt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::InProgress,
            1 => Self::Completed,
            2 => Self::Failed,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::InProgress => 0,
            Self::Completed => 1,
            Self::Failed => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for AdptCrvRslt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Stored Curves
///
/// Stored curve sets.
///
/// Comments: Stored curve sets - Number of curve sets contained in NCrv - The first set is read-only and indicates the current settings.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Crv {
    /// Active Points
    ///
    /// Number of active points.
    pub act_pt: u16,
    /// Dependent Reference
    ///
    /// Curve dependent reference.
    pub dept_ref: CrvDeptRef,
    /// Power Priority
    ///
    /// Power priority.
    pub pri: Option<CrvPri>,
    /// Curve Access
    ///
    /// Curve read-write access.
    pub read_only: CrvReadOnly,
    /// Stored Curve Points
    ///
    /// Stored curve points.
    ///
    /// Comments: Stored curve sets - curve points for each stored curve - Number of curve points contained in NPt
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl Crv {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, CrvDeptRef> = crate::Point::new(1, 1, true);
    pub const PRI: crate::Point<Self, Option<CrvPri>> = crate::Point::new(2, 1, true);
    pub const READ_ONLY: crate::Point<Self, CrvReadOnly> = crate::Point::new(3, 1, false);
}
impl crate::Group for Crv {
    const LEN: u16 = 4;
}
impl Crv {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                dept_ref: Self::DEPT_REF.from_data(data)?,
                pri: Self::PRI.from_data(data)?,
                read_only: Self::READ_ONLY.from_data(data)?,
                pt,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_crv).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Crv::parse_group(data, counts)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Dependent Reference
///
/// Curve dependent reference.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CrvDeptRef {
    /// Percent Max Watts
    WMaxPct,
    /// Percent Max Vars
    VarMaxPct,
    /// Percent Available Vars
    VarAvalPct,
    /// Percent Max Apparent Power
    VaMaxPct,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CrvDeptRef {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::WMaxPct,
            1 => Self::VarMaxPct,
            2 => Self::VarAvalPct,
            3 => Self::VaMaxPct,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMaxPct => 0,
            Self::VarMaxPct => 1,
            Self::VarAvalPct => 2,
            Self::VaMaxPct => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CrvDeptRef {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power Priority
///
/// Power priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CrvPri {
    /// Active Power Priority
    ///
    /// Active power priority.
    Active,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    Reactive,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CrvPri {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::Reactive,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Active => 0,
            Self::Reactive => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CrvPri {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Curve Access
///
/// Curve read-write access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CrvReadOnly {
    /// Read-Write Access
    ///
    /// Curve has read-write access.
    Rw,
    /// Read-Only Access
    ///
    /// Curve has read-only access.
    R,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CrvReadOnly {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Rw,
            1 => Self::R,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Rw => 0,
            Self::R => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CrvReadOnly {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Stored Curve Points
///
/// Stored curve points.
///
/// Comments: Stored curve sets - curve points for each stored curve - Number of curve points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    pub w: Option<i16>,
    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    pub var: Option<i16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const W: crate::Point<Self, Option<i16>> = crate::Point::new(0, 1, true);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, true);
}
impl crate::Group for Pt {
    const LEN: u16 = 2;
}
impl Pt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                w: Self::W.from_data(data)?,
                var: Self::VAR.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_pt).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Pt::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for DerWattVar {
    const ID: u16 = 712;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m712
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
