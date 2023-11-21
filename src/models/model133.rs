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
    pub actschd: u32,
    /// ModEna
    ///
    /// Is basic scheduling active.
    pub modena: u16,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    pub nschd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    pub npts: u16,
}

#[allow(missing_docs)]

impl Model133 {
    pub const ACTSCHD: crate::PointDef<Self, u32> = crate::PointDef::new(0, 2, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const NSCHD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const NPTS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
}

impl crate::Model for Model133 {
    const ID: u16 = 133;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actschd: Self::ACTSCHD.from_data(data)?,
            modena: Self::MODENA.from_data(data)?,
            nschd: Self::NSCHD.from_data(data)?,
            npts: Self::NPTS.from_data(data)?,
        })
    }
}
