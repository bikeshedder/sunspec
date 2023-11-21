/// DER Trip LV
///
/// DER low voltage trip model.
#[derive(Debug)]
pub struct Model707 {
    /// DER Trip LV Module Enable
    ///
    /// DER low voltage trip control enable.
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
    pub ncrvset: u16,
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

impl Model707 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPTCRVREQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPTCRVRSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NCRVSET: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model707 {
    const ID: u16 = 707;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adptcrvreq: Self::ADPTCRVREQ.from_data(data)?,
            adptcrvrslt: Self::ADPTCRVRSLT.from_data(data)?,
            npt: Self::NPT.from_data(data)?,
            ncrvset: Self::NCRVSET.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
        })
    }
}
