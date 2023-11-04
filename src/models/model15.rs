/// Interface Counters Model
///
/// Interface counters
#[derive(Debug)]
pub struct Model15 {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    pub clr: Option<u16>,
    /// Input Count
    ///
    /// Number of bytes received
    pub incnt: Option<u32>,
    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    pub inuccnt: Option<u32>,
    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    pub innuccnt: Option<u32>,
    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    pub indsccnt: Option<u32>,
    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    pub inerrcnt: Option<u32>,
    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    pub inunkcnt: Option<u32>,
    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    pub outcnt: Option<u32>,
    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    pub outuccnt: Option<u32>,
    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    pub outnuccnt: Option<u32>,
    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    pub outdsccnt: Option<u32>,
    /// Output Error Count
    ///
    /// Number of outbound error packets
    pub outerrcnt: Option<u32>,
}

#[allow(missing_docs)]

impl Model15 {
    pub const CLR: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const INCNT: crate::PointDef<Self, u32> = crate::PointDef::new(1, 2, false);
    pub const INUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(3, 2, false);
    pub const INNUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const INDSCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const INERRCNT: crate::PointDef<Self, u32> = crate::PointDef::new(9, 2, false);
    pub const INUNKCNT: crate::PointDef<Self, u32> = crate::PointDef::new(11, 2, false);
    pub const OUTCNT: crate::PointDef<Self, u32> = crate::PointDef::new(13, 2, false);
    pub const OUTUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(15, 2, false);
    pub const OUTNUCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(17, 2, false);
    pub const OUTDSCCNT: crate::PointDef<Self, u32> = crate::PointDef::new(19, 2, false);
    pub const OUTERRCNT: crate::PointDef<Self, u32> = crate::PointDef::new(21, 2, false);
}

impl crate::Model for Model15 {
    const ID: u16 = 15;
    const LENGTH: u16 = 24;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            clr: Self::CLR.from_data(data)?,
            incnt: Self::INCNT.from_data(data)?,
            inuccnt: Self::INUCCNT.from_data(data)?,
            innuccnt: Self::INNUCCNT.from_data(data)?,
            indsccnt: Self::INDSCCNT.from_data(data)?,
            inerrcnt: Self::INERRCNT.from_data(data)?,
            inunkcnt: Self::INUNKCNT.from_data(data)?,
            outcnt: Self::OUTCNT.from_data(data)?,
            outuccnt: Self::OUTUCCNT.from_data(data)?,
            outnuccnt: Self::OUTNUCCNT.from_data(data)?,
            outdsccnt: Self::OUTDSCCNT.from_data(data)?,
            outerrcnt: Self::OUTERRCNT.from_data(data)?,
        })
    }
}
