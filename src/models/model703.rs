/// Enter Service
///
/// Enter service model.
#[derive(Debug)]
pub struct Model703 {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    pub es: Option<u16>,
    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    pub esv_hi: Option<u16>,
    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    pub esv_lo: Option<u16>,
    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    pub es_hz_hi: Option<u32>,
    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    pub es_hz_lo: Option<u32>,
    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    pub es_dly_tms: Option<u32>,
    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    pub es_rnd_tms: Option<u32>,
    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    pub es_rmp_tms: Option<u32>,
    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    pub es_dly_rem_tms: Option<u32>,
    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    pub v_sf: Option<i16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    pub hz_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model703 {
    pub const ES: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const ESV_HI: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const ESV_LO: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const ES_HZ_HI: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(3, 2, true);
    pub const ES_HZ_LO: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, true);
    pub const ES_DLY_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, true);
    pub const ES_RND_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(9, 2, true);
    pub const ES_RMP_TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, true);
    pub const ES_DLY_REM_TMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(13, 2, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const HZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
}

impl crate::Model for Model703 {
    const ID: u16 = 703;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            es: Self::ES.from_data(data)?,
            esv_hi: Self::ESV_HI.from_data(data)?,
            esv_lo: Self::ESV_LO.from_data(data)?,
            es_hz_hi: Self::ES_HZ_HI.from_data(data)?,
            es_hz_lo: Self::ES_HZ_LO.from_data(data)?,
            es_dly_tms: Self::ES_DLY_TMS.from_data(data)?,
            es_rnd_tms: Self::ES_RND_TMS.from_data(data)?,
            es_rmp_tms: Self::ES_RMP_TMS.from_data(data)?,
            es_dly_rem_tms: Self::ES_DLY_REM_TMS.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
        })
    }
}
