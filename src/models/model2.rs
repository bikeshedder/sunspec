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
    pub stvnd: Option<u16>,
    /// Event Code
    ///
    /// Bitmask event code
    pub evt: u32,
    /// Vendor Event Code
    ///
    /// Vendor specific event code
    pub evtvnd: Option<u32>,
    /// Control
    ///
    /// Control register for all aggregated devices
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    pub ctlvnd: Option<u32>,
    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    pub ctlvl: Option<u32>,
}

#[allow(missing_docs)]

impl Model2 {
    pub const AID: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const UN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CTLVND: crate::PointDef<Self, u32> = crate::PointDef::new(10, 2, false);
    pub const CTLVL: crate::PointDef<Self, u32> = crate::PointDef::new(12, 2, false);
}

impl crate::Model for Model2 {
    const ID: u16 = 2;
    const LENGTH: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aid: Self::AID
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            un: Self::UN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvnd: Self::CTLVND.from_data(data)?,
            ctlvl: Self::CTLVL.from_data(data)?,
        })
    }
}
