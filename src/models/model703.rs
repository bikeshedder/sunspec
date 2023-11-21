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
    pub esvhi: Option<u16>,
    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    pub esvlo: Option<u16>,
    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    pub eshzhi: Option<u32>,
    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    pub eshzlo: Option<u32>,
    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    pub esdlytms: Option<u32>,
    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    pub esrndtms: Option<u32>,
    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    pub esrmptms: Option<u32>,
    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    pub esdlyremtms: Option<u32>,
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
    pub const ESVHI: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const ESVLO: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const ESHZHI: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(3, 2, true);
    pub const ESHZLO: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(5, 2, true);
    pub const ESDLYTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, true);
    pub const ESRNDTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(9, 2, true);
    pub const ESRMPTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, true);
    pub const ESDLYREMTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(13, 2, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const HZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
}

impl crate::Model for Model703 {
    const ID: u16 = 703;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            es: Self::ES.from_data(data)?,
            esvhi: Self::ESVHI.from_data(data)?,
            esvlo: Self::ESVLO.from_data(data)?,
            eshzhi: Self::ESHZHI.from_data(data)?,
            eshzlo: Self::ESHZLO.from_data(data)?,
            esdlytms: Self::ESDLYTMS.from_data(data)?,
            esrndtms: Self::ESRNDTMS.from_data(data)?,
            esrmptms: Self::ESRMPTMS.from_data(data)?,
            esdlyremtms: Self::ESDLYREMTMS.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
        })
    }
}
