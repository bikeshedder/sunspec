/// DER Storage Capacity
///
/// DER storage capacity.
#[derive(Debug)]
pub struct Model713 {
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    pub whrtg: Option<u16>,
    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    pub whavail: Option<u16>,
    /// State of Charge
    ///
    /// State of charge of the DER storage.
    pub soc: Option<u16>,
    /// State of Health
    ///
    /// State of health of the DER storage.
    pub soh: Option<u16>,
    /// Status
    ///
    /// Storage status.
    pub sta: Option<u16>,
    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    pub wh_sf: Option<i16>,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    pub pct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model713 {
    pub const WHRTG: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WHAVAIL: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const SOH: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const STA: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model713 {
    const ID: u16 = 713;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            whrtg: Self::WHRTG.from_data(data)?,
            whavail: Self::WHAVAIL.from_data(data)?,
            soc: Self::SOC.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            sta: Self::STA.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
            pct_sf: Self::PCT_SF.from_data(data)?,
        })
    }
}
