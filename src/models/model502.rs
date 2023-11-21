/// Solar Module
///
/// A solar module model supporting DC-DC converter
///
/// Notes: Integer
#[derive(Debug)]
pub struct Model502 {
    /// Current scale factor
    pub a_sf: Option<i16>,
    /// Voltage scale factor
    pub v_sf: Option<i16>,
    /// Power scale factor
    pub w_sf: Option<i16>,
    /// Energy scale factor
    pub wh_sf: Option<i16>,
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
    pub outa: Option<i16>,
    /// Output Voltage
    ///
    /// Output Voltage
    pub outv: Option<i16>,
    /// Output Energy
    ///
    /// Output Energy
    pub outwh: Option<u32>,
    /// Output Power
    ///
    /// Output Power
    pub outpw: Option<i16>,
    /// Temp
    ///
    /// Module Temperature
    pub tmp: Option<i16>,
    /// Input Current
    ///
    /// Input Current
    pub ina: Option<i16>,
    /// Input Voltage
    ///
    /// Input Voltage
    pub inv: Option<i16>,
    /// Input Energy
    ///
    /// Input Energy
    pub inwh: Option<u32>,
    /// Input Power
    ///
    /// Input Power
    pub inw: Option<i16>,
}

#[allow(missing_docs)]

impl Model502 {
    pub const A_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const W_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const WH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const STAT: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const STATVEND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(6, 2, false);
    pub const EVTVEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(8, 2, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, true);
    pub const CTLVEND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(11, 2, true);
    pub const CTLVAL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(13, 2, true);
    pub const TMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(15, 2, false);
    pub const OUTA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
    pub const OUTV: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const OUTWH: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(19, 2, false);
    pub const OUTPW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const INA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const INV: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(24, 1, false);
    pub const INWH: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(25, 2, false);
    pub const INW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, false);
}

impl crate::Model for Model502 {
    const ID: u16 = 502;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
            stat: Self::STAT.from_data(data)?,
            statvend: Self::STATVEND.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evtvend: Self::EVTVEND.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            ctlvend: Self::CTLVEND.from_data(data)?,
            ctlval: Self::CTLVAL.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            outa: Self::OUTA.from_data(data)?,
            outv: Self::OUTV.from_data(data)?,
            outwh: Self::OUTWH.from_data(data)?,
            outpw: Self::OUTPW.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            ina: Self::INA.from_data(data)?,
            inv: Self::INV.from_data(data)?,
            inwh: Self::INWH.from_data(data)?,
            inw: Self::INW.from_data(data)?,
        })
    }
}
