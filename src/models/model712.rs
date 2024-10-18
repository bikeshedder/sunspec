//! DER Watt-Var
/// DER Watt-Var
///
/// DER Watt-Var model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model712 {
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
}
#[allow(missing_docs)]
impl Model712 {
    pub const ENA: crate::PointDef<Self, Ena> = crate::PointDef::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::PointDef<Self, AdptCrvRslt> = crate::PointDef::new(2, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, true);
    pub const RVRT_REM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, false);
    pub const RVRT_CRV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, true);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const DEPT_REF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
}
impl crate::Model for Model712 {
    const ID: u16 = 712;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
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
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m712
    }
}
/// DER Watt-Var Module Enable
///
/// DER Watt-Var control enable.
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
/// Set Active Curve Result
///
/// Result of last set active curve operation.
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
