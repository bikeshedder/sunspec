/// Pricing
///
/// Pricing Signal
///
/// Notes: Ref 3: 8.7.5.1; Ref 4: 6
#[derive(Debug)]
pub struct Model125 {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    pub mod_ena: u16,
    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    pub sig_type: Option<u16>,
    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    pub sig: i16,
    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    pub win_tms: Option<u16>,
    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    pub rvt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    pub rmp_tms: Option<u16>,
    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    pub sig_sf: i16,
}

#[allow(missing_docs)]

impl Model125 {
    pub const MOD_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const SIG_TYPE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const SIG: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RVT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const SIG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
}

impl crate::Model for Model125 {
    const ID: u16 = 125;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            mod_ena: Self::MOD_ENA.from_data(data)?,
            sig_type: Self::SIG_TYPE.from_data(data)?,
            sig: Self::SIG.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvt_tms: Self::RVT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            sig_sf: Self::SIG_SF.from_data(data)?,
        })
    }
}
