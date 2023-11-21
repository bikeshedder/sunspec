/// DER Trip LF
///
/// DER low frequency trip model.
#[derive(Debug)]
pub struct Model709 {
    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
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
    /// Frequency Scale Factor
    ///
    /// Scale factor for curve frequency points.
    pub hz_sf: i16,
    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    pub tms_sf: i16,
}

#[allow(missing_docs)]

impl Model709 {
    pub const ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ADPTCRVREQ: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ADPTCRVRSLT: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NCRVSET: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model709 {
    const ID: u16 = 709;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ena: Self::ENA.from_data(data)?,
            adptcrvreq: Self::ADPTCRVREQ.from_data(data)?,
            adptcrvrslt: Self::ADPTCRVRSLT.from_data(data)?,
            npt: Self::NPT.from_data(data)?,
            ncrvset: Self::NCRVSET.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
        })
    }
}
