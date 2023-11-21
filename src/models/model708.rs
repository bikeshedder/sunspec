/// DER Trip HV
///
/// DER high voltage trip model.
#[derive(Debug)]
pub struct Model708 {
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
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
    pub n_crv_set: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    pub v_sf: i16,
    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    pub tms_sf: i16,
}

#[allow(missing_docs)]

impl Model708 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPT_CRV_REQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPT_CRV_RSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N_CRV_SET: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model708 {
    const ID: u16 = 708;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adpt_crv_req: Self::ADPT_CRV_REQ.from_data(data)?,
            adpt_crv_rslt: Self::ADPT_CRV_RSLT.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            n_crv_set: Self::N_CRV_SET.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
        })
    }
}
