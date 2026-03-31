//! DER Volt-Watt
/// Type alias for [`DerVoltWatt`].
pub type Model706 = DerVoltWatt;
struct Counts {
    n_pt: u16,
    n_crv: u16,
}
/// DER Volt-Watt
///
/// DER Volt-Watt model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerVoltWatt {
    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    pub ena: Ena,
    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    pub adpt_crv_req: u16,
    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
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
    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    pub rvrt_rem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    pub rvrt_crv: Option<u16>,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    pub v_sf: i16,
    /// Watt Scale Factor
    ///
    /// Scale factor for curve watt points.
    pub dept_ref_sf: i16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    pub rsp_tms_sf: i16,
    /// Stored Curves
    ///
    /// Stored curve sets.
    ///
    /// Comments: Stored curve sets - Number of curve sets contained in NCrv - The first set is read-only and indicates the current settings.
    pub crv: Vec<Crv>,
}
#[allow(missing_docs)]
impl DerVoltWatt {
    pub const ENA: crate::Point<Self, Ena> = crate::Point::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::Point<Self, AdptCrvRslt> = crate::Point::new(2, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(5, 2, true);
    pub const RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, false);
    pub const RVRT_CRV: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(10, 1, false);
    pub const DEPT_REF_SF: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const RSP_TMS_SF: crate::Point<Self, i16> = crate::Point::new(12, 1, false);
}
static DER_VOLT_WATT_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "ena",
        label: "DER Volt-Watt Module Enable",
        description: "Volt-Watt control enable.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "adpt_crv_req",
        label: "Adopt Curve Request",
        description: "Index of curve points to adopt. First curve index is 1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "adpt_crv_rslt",
        label: "Adopt Curve Result",
        description: "Result of last adopt curve operation.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_pt",
        label: "Number Of Points",
        description: "Number of curve points supported.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_crv",
        label: "Stored Curve Count",
        description: "Number of stored curves supported.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rvrt_tms",
        label: "Reversion Timeout",
        description: "Reversion time in seconds.  0 = No reversion time.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rvrt_rem",
        label: "Reversion Time Remaining",
        description: "Reversion time remaining in seconds.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rvrt_crv",
        label: "Reversion Curve",
        description: "Default curve after reversion timeout.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "Voltage Scale Factor",
        description: "Scale factor for curve voltage points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dept_ref_sf",
        label: "Watt Scale Factor",
        description: "Scale factor for curve watt points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rsp_tms_sf",
        label: "Open-Loop Scale Factor",
        description: "Open loop response time scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "crv",
        label: "Stored Curves",
        description: "Stored curve sets.",
        kind: crate::FieldKind::RepeatingGroup(<Crv as crate::GroupMeta>::group_info),
    },
];
static DER_VOLT_WATT_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DERVoltWatt",
    label: "DER Volt-Watt",
    description: "DER Volt-Watt model.",
    fields: DER_VOLT_WATT_FIELDS,
};
impl crate::GroupMeta for DerVoltWatt {
    fn group_info() -> &'static crate::GroupInfo {
        &DER_VOLT_WATT_GROUP_INFO
    }
}
impl crate::Group for DerVoltWatt {
    const LEN: u16 = 13;
}
impl DerVoltWatt {
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
                v_sf: Self::V_SF.from_data(data)?,
                dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
                rsp_tms_sf: Self::RSP_TMS_SF.from_data(data)?,
                crv,
            },
        ))
    }
}
/// DER Volt-Watt Module Enable
///
/// Volt-Watt control enable.
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
/// Adopt Curve Result
///
/// Result of last adopt curve operation.
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
    /// Open Loop Response Time
    ///
    /// Open loop response time.
    pub rsp_tms: Option<u32>,
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
    pub const RSP_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(2, 2, true);
    pub const READ_ONLY: crate::Point<Self, CrvReadOnly> = crate::Point::new(4, 1, false);
}
static CRV_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "act_pt",
        label: "Active Points",
        description: "Number of active points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dept_ref",
        label: "Dependent Reference",
        description: "Curve dependent reference.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rsp_tms",
        label: "Open Loop Response Time",
        description: "Open loop response time.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "read_only",
        label: "Curve Access",
        description: "Curve read-write access.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pt",
        label: "Stored Curve Points",
        description: "Stored curve points.",
        kind: crate::FieldKind::RepeatingGroup(<Pt as crate::GroupMeta>::group_info),
    },
];
static CRV_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "Crv",
    label: "Stored Curves",
    description: "Stored curve sets.",
    fields: CRV_FIELDS,
};
impl crate::GroupMeta for Crv {
    fn group_info() -> &'static crate::GroupInfo {
        &CRV_GROUP_INFO
    }
}
impl crate::Group for Crv {
    const LEN: u16 = 5;
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
                rsp_tms: Self::RSP_TMS.from_data(data)?,
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
    #[allow(missing_docs)]
    WMaxPct,
    #[allow(missing_docs)]
    WAvalPct,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CrvDeptRef {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::WMaxPct,
            1 => Self::WAvalPct,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMaxPct => 0,
            Self::WAvalPct => 1,
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
    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub v: Option<u16>,
    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub w: Option<i16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const W: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, true);
}
static PT_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "v",
        label: "Voltage Point",
        description: "Curve voltage point as percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w",
        label: "Dependent Reference",
        description: "Active power in percent of rated active power.",
        kind: crate::FieldKind::Point,
    },
];
static PT_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "Pt",
    label: "Stored Curve Points",
    description: "Stored curve points.",
    fields: PT_FIELDS,
};
impl crate::GroupMeta for Pt {
    fn group_info() -> &'static crate::GroupInfo {
        &PT_GROUP_INFO
    }
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
                v: Self::V.from_data(data)?,
                w: Self::W.from_data(data)?,
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
impl crate::Model for DerVoltWatt {
    const ID: u16 = 706;
    const NAME: &'static str = "DERVoltWatt";
    const LABEL: &'static str = "DER Volt-Watt";
    const DESCRIPTION: &'static str = "DER Volt-Watt model.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m706
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
