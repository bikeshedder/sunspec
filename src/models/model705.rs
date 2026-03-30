//! DER Volt-Var
pub type Model705 = DerVoltVar;
/// DER Volt-Var
///
/// DER Volt-Var model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerVoltVar {
    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
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
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    pub dept_ref_sf: i16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    pub rsp_tms_sf: i16,
    /// Stored Curves
    ///
    /// Stored curve sets.
    ///
    /// Comments: Stored Curve Sets - Number of curve sets contained in NCrv - The first set is read-only and indicates the current settings.
    pub crv: Vec<Crv>,
}
#[allow(missing_docs)]
impl DerVoltVar {
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
impl crate::Group for DerVoltVar {
    const LEN: u16 = 13;
}
impl DerVoltVar {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
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
                crv: Vec::new(),
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.crv) = Crv::parse_multiple(data, &group)?;
        Ok((data, group))
    }
}
/// DER Volt-Var Module Enable
///
/// Volt-Var control enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Ena {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}
impl crate::Value for Ena {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Ena> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Ena::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Adopt Curve Result
///
/// Result of last adopt curve operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum AdptCrvRslt {
    /// Update In Progress
    ///
    /// Curve update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Curve update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Curve update failed.
    Failed = 2,
}
impl crate::Value for AdptCrvRslt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<AdptCrvRslt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                AdptCrvRslt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Stored Curves
///
/// Stored curve sets.
///
/// Comments: Stored Curve Sets - Number of curve sets contained in NCrv - The first set is read-only and indicates the current settings.
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
    /// Vref Adjustment
    ///
    /// Vref adjustment as percentage of nominal voltage.
    pub v_ref: Option<u16>,
    /// Current Autonomous Vref
    ///
    /// Autonomous vref value as a percentage of nominal voltage.
    pub v_ref_auto: Option<u16>,
    /// Autonomous Vref Enable
    ///
    /// Enable autonomous vref.
    pub v_ref_auto_ena: Option<CrvVRefAutoEna>,
    /// Auto Vref Time Constant
    ///
    /// Autonomous vref time constant.
    pub v_ref_auto_tms: Option<u16>,
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
    /// Comments: Stored Curve Sets - Curve points for each stored curve - Number of curve points contained in NPt
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl Crv {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, CrvDeptRef> = crate::Point::new(1, 1, true);
    pub const PRI: crate::Point<Self, Option<CrvPri>> = crate::Point::new(2, 1, true);
    pub const V_REF: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const V_REF_AUTO: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const V_REF_AUTO_ENA: crate::Point<Self, Option<CrvVRefAutoEna>> =
        crate::Point::new(5, 1, true);
    pub const V_REF_AUTO_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const RSP_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, true);
    pub const READ_ONLY: crate::Point<Self, CrvReadOnly> = crate::Point::new(9, 1, false);
}
impl crate::Group for Crv {
    const LEN: u16 = 10;
}
impl Crv {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                dept_ref: Self::DEPT_REF.from_data(data)?,
                pri: Self::PRI.from_data(data)?,
                v_ref: Self::V_REF.from_data(data)?,
                v_ref_auto: Self::V_REF_AUTO.from_data(data)?,
                v_ref_auto_ena: Self::V_REF_AUTO_ENA.from_data(data)?,
                v_ref_auto_tms: Self::V_REF_AUTO_TMS.from_data(data)?,
                rsp_tms: Self::RSP_TMS.from_data(data)?,
                read_only: Self::READ_ONLY.from_data(data)?,
                pt: Vec::new(),
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &DerVoltVar,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.pt) = Pt::parse_multiple(data, model)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &DerVoltVar,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..model.n_crv {
            let group;
            (data, group) = Crv::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
/// Dependent Reference
///
/// Curve dependent reference.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvDeptRef {
    /// Percent Max Watts
    WMaxPct = 0,
    /// Percent Max Vars
    VarMaxPct = 1,
    /// Percent Available Vars
    VarAvalPct = 2,
    /// Percent Max Apparent Power
    VaMaxPct = 3,
}
impl crate::Value for CrvDeptRef {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvDeptRef> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvDeptRef::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Power Priority
///
/// Power priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvPri {
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
impl crate::Value for CrvPri {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvPri> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvPri::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Autonomous Vref Enable
///
/// Enable autonomous vref.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvVRefAutoEna {
    /// Disabled Flag
    ///
    /// Disabled flag (Disabled = 0, Enabled = 1).
    Disabled = 0,
    /// Enabled Flag
    ///
    /// Enabled flag (Disabled = 0, Enabled = 1).
    Enabled = 1,
}
impl crate::Value for CrvVRefAutoEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvVRefAutoEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvVRefAutoEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Curve Access
///
/// Curve read-write access.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvReadOnly {
    /// Read-Write Access
    ///
    /// Curve has read-write access.
    Rw = 0,
    /// Read-Only Access
    ///
    /// Curve has read-only access.
    R = 1,
}
impl crate::Value for CrvReadOnly {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvReadOnly> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvReadOnly::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Stored Curve Points
///
/// Stored curve points.
///
/// Comments: Stored Curve Sets - Curve points for each stored curve - Number of curve points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub v: Option<u16>,
    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Detail: Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    pub var: Option<i16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, true);
}
impl crate::Group for Pt {
    const LEN: u16 = 2;
}
impl Pt {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                v: Self::V.from_data(data)?,
                var: Self::VAR.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &DerVoltVar,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &DerVoltVar,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..model.n_pt {
            let group;
            (data, group) = Pt::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for DerVoltVar {
    const ID: u16 = 705;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m705
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
