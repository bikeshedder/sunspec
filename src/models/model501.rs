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
    pub statvend: Option<u16>,
    /// Events
    ///
    /// Bitmask value.  Module Event Flags
    pub evt: u32,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    pub evtvend: Option<u32>,
    /// Control
    ///
    /// Module Control
    pub ctl: Option<u16>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    pub ctlvend: Option<u32>,
    /// Control Value
    ///
    /// Module Control Value
    pub ctlval: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    pub tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    pub outa: Option<f32>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub outv: Option<f32>,
    /// Output Energy
    ///
    /// Output Energy
    pub outwh: Option<f32>,
    /// Output Power
    ///
    /// Output Power
    pub outw: Option<f32>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<f32>,
    /// Input Current
    ///
    /// Input Current
    pub ina: Option<f32>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub inv: Option<f32>,
    /// Input Energy
    ///
    /// Input Energy
    pub inwh: Option<f32>,
    /// Input Power
    ///
    /// Input Power
    pub inw: Option<f32>,
}

#[allow(missing_docs)]

impl Model501 {
    pub const STAT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STATVEND: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const EVTVEND: crate::PointDef<Self, u32> = crate::PointDef::new(4, 2, false);
    pub const CTL: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const CTLVEND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, true);
    pub const CTLVAL: crate::PointDef<Self, i32> = crate::PointDef::new(9, 2, true);
    pub const TMS: crate::PointDef<Self, u32> = crate::PointDef::new(11, 2, false);
    pub const OUTA: crate::PointDef<Self, f32> = crate::PointDef::new(13, 2, false);
    pub const OUTV: crate::PointDef<Self, f32> = crate::PointDef::new(15, 2, false);
    pub const OUTWH: crate::PointDef<Self, f32> = crate::PointDef::new(17, 2, false);
    pub const OUTW: crate::PointDef<Self, f32> = crate::PointDef::new(19, 2, false);
    pub const TMP: crate::PointDef<Self, f32> = crate::PointDef::new(21, 2, false);
    pub const INA: crate::PointDef<Self, f32> = crate::PointDef::new(23, 2, false);
    pub const INV: crate::PointDef<Self, f32> = crate::PointDef::new(25, 2, false);
    pub const INWH: crate::PointDef<Self, f32> = crate::PointDef::new(27, 2, false);
    pub const INW: crate::PointDef<Self, f32> = crate::PointDef::new(29, 2, false);
}

impl crate::Model for Model501 {
    const ID: u16 = 501;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stat: Self::STAT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            statvend: Self::STATVEND.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvend: Self::EVTVEND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvend: Self::CTLVEND.from_data(data)?,
            ctlval: Self::CTLVAL.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            outa: Self::OUTA.from_data(data)?,
            outv: Self::OUTV.from_data(data)?,
            outwh: Self::OUTWH.from_data(data)?,
            outw: Self::OUTW.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            ina: Self::INA.from_data(data)?,
            inv: Self::INV.from_data(data)?,
            inwh: Self::INWH.from_data(data)?,
            inw: Self::INW.from_data(data)?,
        })
    }
}
