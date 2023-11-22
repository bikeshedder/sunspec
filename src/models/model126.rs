//! Static Volt-VAR

/// Static Volt-VAR
///
/// Static Volt-VAR Arrays
///
/// Notes: Ref 3: 8.8.1.2
#[derive(Debug)]
pub struct Model126 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is Volt-VAR control active.
    pub mod_ena: u16,
    /// WinTms
    ///
    /// Time window for volt-VAR change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub n_pt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// scale factor for dependent variable.
    pub dept_ref_sf: i16,
    #[allow(missing_docs)]
    pub rmp_inc_dec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model126 {
    pub const ACT_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const DEPT_REF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model126 {
    const ID: u16 = 126;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
            rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
        })
    }
}
