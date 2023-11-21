/// Basic Aggregator
///
/// Aggregates a collection of models for a given model id
#[derive(Debug)]
pub struct Model2 {
    /// AID
    ///
    /// Aggregated model id
    pub aid: u16,
    /// N
    ///
    /// Number of aggregated models
    pub n: u16,
    /// UN
    ///
    /// Update Number.  Incrementing number each time the mapping is changed.  If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before.  Otherwise the entire model must be read to refresh the changes
    pub un: u16,
    /// Status
    ///
    /// Enumerated status code
    pub st: u16,
    /// Vendor Status
    ///
    /// Vendor specific status code
    pub st_vnd: Option<u16>,
    /// Event Code
    ///
    /// Bitmask event code
    pub evt: u32,
    /// Vendor Event Code
    ///
    /// Vendor specific event code
    pub evt_vnd: Option<u32>,
    /// Control
    ///
    /// Control register for all aggregated devices
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    pub ctl_vnd: Option<u32>,
    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    pub ctl_vl: Option<u32>,
}

#[allow(missing_docs)]

impl Model2 {
    pub const AID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const UN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const ST_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVT_VND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, false);
    pub const CTL_VND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(10, 2, false);
    pub const CTL_VL: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
}

impl crate::Model for Model2 {
    const ID: u16 = 2;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aid: Self::AID.from_data(data)?,
            n: Self::N.from_data(data)?,
            un: Self::UN.from_data(data)?,
            st: Self::ST.from_data(data)?,
            st_vnd: Self::ST_VND.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evt_vnd: Self::EVT_VND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctl_vnd: Self::CTL_VND.from_data(data)?,
            ctl_vl: Self::CTL_VL.from_data(data)?,
        })
    }
}
