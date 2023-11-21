/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model501 {
    /// Status
    ///
    /// Enumerated value.  Module Status Code
    pub stat: u16,
    /// Vendor Status
    ///
    /// Module Vendor Status Code
    pub stat_vend: Option<u16>,
    /// Events
    ///
    /// Bitmask value.  Module Event Flags
    pub evt: u32,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    pub evt_vend: Option<u32>,
    /// Control
    ///
    /// Module Control
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    pub ctl_vend: Option<u32>,
    /// Control Value
    ///
    /// Module Control Value
    pub ctl_val: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    pub tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    pub out_a: Option<f32>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub out_v: Option<f32>,
    /// Output Energy
    ///
    /// Output Energy
    pub out_wh: Option<f32>,
    /// Output Power
    ///
    /// Output Power
    pub out_w: Option<f32>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<f32>,
    /// Input Current
    ///
    /// Input Current
    pub in_a: Option<f32>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub in_v: Option<f32>,
    /// Input Energy
    ///
    /// Input Energy
    pub in_wh: Option<f32>,
    /// Input Power
    ///
    /// Input Power
    pub in_w: Option<f32>,
}

#[allow(missing_docs)]

impl Model501 {
    pub const STAT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STAT_VEND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const EVT_VEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const CTL_VEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(7, 2, true);
    pub const CTL_VAL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(9, 2, true);
    pub const TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, false);
    pub const OUT_A: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(13, 2, false);
    pub const OUT_V: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(15, 2, false);
    pub const OUT_WH: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(17, 2, false);
    pub const OUT_W: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(19, 2, false);
    pub const TMP: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(21, 2, false);
    pub const IN_A: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(23, 2, false);
    pub const IN_V: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(25, 2, false);
    pub const IN_WH: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(27, 2, false);
    pub const IN_W: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(29, 2, false);
}

impl crate::Model for Model501 {
    const ID: u16 = 501;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stat: Self::STAT.from_data(data)?,
            stat_vend: Self::STAT_VEND.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evt_vend: Self::EVT_VEND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctl_vend: Self::CTL_VEND.from_data(data)?,
            ctl_val: Self::CTL_VAL.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            out_a: Self::OUT_A.from_data(data)?,
            out_v: Self::OUT_V.from_data(data)?,
            out_wh: Self::OUT_WH.from_data(data)?,
            out_w: Self::OUT_W.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            in_a: Self::IN_A.from_data(data)?,
            in_v: Self::IN_V.from_data(data)?,
            in_wh: Self::IN_WH.from_data(data)?,
            in_w: Self::IN_W.from_data(data)?,
        })
    }
}
