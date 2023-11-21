/// Watt-PF
///
/// Watt-Power Factor
///
/// Notes: Ref 3: 8.11.1.2
#[derive(Debug)]
pub struct Model131 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is watt-PF mode active.
    pub mod_ena: u16,
    /// WinTms
    ///
    /// Time window for watt-PF change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Max number of points in array.
    pub n_pt: u16,
    /// W_SF
    ///
    /// Scale factor for percent WMax.
    pub w_sf: i16,
    /// PF_SF
    ///
    /// Scale factor for PF.
    pub pf_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmp_inc_dec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model131 {
    pub const ACT_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model131 {
    const ID: u16 = 131;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
        })
    }
}
