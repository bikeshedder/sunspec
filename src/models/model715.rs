/// DERCtl
///
/// DER Control
#[derive(Debug)]
pub struct Model715 {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    ///
    /// Comments: DER Controls
    pub locremctl: Option<u16>,
    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    pub derhb: Option<u32>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    pub controllerhb: Option<u32>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    pub alarmreset: Option<u16>,
    /// Set Operation
    ///
    /// Commands to PCS. Enumerated value.
    pub opctl: Option<u16>,
}

#[allow(missing_docs)]

impl Model715 {
    pub const LOCREMCTL: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const DERHB: crate::PointDef<Self, u32> = crate::PointDef::new(1, 2, false);
    pub const CONTROLLERHB: crate::PointDef<Self, u32> = crate::PointDef::new(3, 2, true);
    pub const ALARMRESET: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const OPCTL: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
}

impl crate::Model for Model715 {
    const ID: u16 = 715;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            locremctl: Self::LOCREMCTL.from_data(data)?,
            derhb: Self::DERHB.from_data(data)?,
            controllerhb: Self::CONTROLLERHB.from_data(data)?,
            alarmreset: Self::ALARMRESET.from_data(data)?,
            opctl: Self::OPCTL.from_data(data)?,
        })
    }
}
