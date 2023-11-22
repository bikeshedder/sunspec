//! Multiple MPPT Inverter Extension Model

/// Multiple MPPT Inverter Extension Model
#[derive(Debug)]
pub struct Model160 {
    /// Current Scale Factor
    pub dca_sf: Option<i16>,
    /// Voltage Scale Factor
    pub dcv_sf: Option<i16>,
    /// Power Scale Factor
    pub dcw_sf: Option<i16>,
    /// Energy Scale Factor
    pub dcwh_sf: Option<i16>,
    /// Global Events
    pub evt: Option<u32>,
    /// Number of Modules
    pub n: Option<u16>,
    /// Timestamp Period
    pub tms_per: Option<u16>,
}

#[allow(missing_docs)]

impl Model160 {
    pub const DCA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const EVT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, false);
    pub const N: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const TMS_PER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model160 {
    const ID: u16 = 160;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            n: Self::N.from_data(data)?,
            tms_per: Self::TMS_PER.from_data(data)?,
        })
    }
}
