/// LFRTX
///
/// LFRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
pub struct Model143 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub mod_ena: u16,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub n_pt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    #[allow(missing_docs)]
    pub crv_type: u16,
}

#[allow(missing_docs)]

impl Model143 {
    pub const ACT_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRV_TYPE: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model143 {
    const ID: u16 = 143;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            crv_type: Self::CRV_TYPE.from_data(data)?,
        })
    }
}
