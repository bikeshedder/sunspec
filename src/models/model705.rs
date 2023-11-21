/// DER Volt-Var
///
/// DER Volt-Var model.
#[derive(Debug)]
pub struct Model705 {
    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    pub ena: u16,
    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    pub adpt_crv_req: u16,
    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    pub adpt_crv_rslt: u16,
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
}

#[allow(missing_docs)]

impl Model705 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, true);
    pub const RVRT_REM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, false);
    pub const RVRT_CRV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, true);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const DEPT_REF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const RSP_TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
}

impl crate::Model for Model705 {
    const ID: u16 = 705;
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
            v_sf: Self::V_SF.from_data(data)?,
            dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
            rsp_tms_sf: Self::RSP_TMS_SF.from_data(data)?,
        })
    }
}
