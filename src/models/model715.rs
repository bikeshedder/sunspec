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
    pub loc_rem_ctl: Option<u16>,
    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    pub der_hb: Option<u32>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    pub controller_hb: Option<u32>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    pub alarm_reset: Option<u16>,
    /// Set Operation
    ///
    /// Commands to PCS. Enumerated value.
    pub op_ctl: Option<u16>,
}

#[allow(missing_docs)]

impl Model715 {
    pub const LOC_REM_CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, false);
    pub const DER_HB: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(1, 2, false);
    pub const CONTROLLER_HB: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(3, 2, true);
    pub const ALARM_RESET: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const OP_CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
}

impl crate::Model for Model715 {
    const ID: u16 = 715;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            loc_rem_ctl: Self::LOC_REM_CTL.from_data(data)?,
            der_hb: Self::DER_HB.from_data(data)?,
            controller_hb: Self::CONTROLLER_HB.from_data(data)?,
            alarm_reset: Self::ALARM_RESET.from_data(data)?,
            op_ctl: Self::OP_CTL.from_data(data)?,
        })
    }
}
