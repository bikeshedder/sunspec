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
    pub adptcrvreq: u16,
    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    pub adptcrvrslt: u16,
    /// Number Of Points
    ///
    /// Number of curve points supported.
    pub npt: u16,
    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    pub ncrv: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrttms: Option<u32>,
    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    pub rvrtrem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    pub rvrtcrv: Option<u16>,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    pub v_sf: i16,
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    pub deptref_sf: i16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    pub rsptms_sf: i16,
}

#[allow(missing_docs)]

impl Model705 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPTCRVREQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPTCRVRSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const RVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, true);
    pub const RVRTREM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, false);
    pub const RVRTCRV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, true);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const DEPTREF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const RSPTMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
}

impl crate::Model for Model705 {
    const ID: u16 = 705;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adptcrvreq: Self::ADPTCRVREQ.from_data(data)?,
            adptcrvrslt: Self::ADPTCRVRSLT.from_data(data)?,
            npt: Self::NPT.from_data(data)?,
            ncrv: Self::NCRV.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rvrtrem: Self::RVRTREM.from_data(data)?,
            rvrtcrv: Self::RVRTCRV.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            deptref_sf: Self::DEPTREF_SF.from_data(data)?,
            rsptms_sf: Self::RSPTMS_SF.from_data(data)?,
        })
    }
}
