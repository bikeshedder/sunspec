//! Basic Scheduling

/// Basic Scheduling
///
/// Basic Scheduling
///
/// Notes: Ref 2: 2.2.8
#[derive(Debug)]
pub struct Model133 {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    pub act_schd: u32,
    /// ModEna
    ///
    /// Is basic scheduling active.
    pub mod_ena: u16,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    pub n_schd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    pub n_pts: u16,
}

#[allow(missing_docs)]

impl Model133 {
    pub const ACT_SCHD: crate::PointDef<Self, u32> = crate::PointDef::new(0, 2, true);
    pub const MOD_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const N_SCHD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N_PTS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
}

impl crate::Model for Model133 {
    const ID: u16 = 133;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_schd: Self::ACT_SCHD.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            n_schd: Self::N_SCHD.from_data(data)?,
            n_pts: Self::N_PTS.from_data(data)?,
        })
    }
}
