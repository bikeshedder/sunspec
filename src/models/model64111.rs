/// Basic Charge Controller
#[derive(Debug)]
pub struct Model64111 {
    /// Port Number
    pub port: u16,
    #[allow(missing_docs)]
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub a_sf: i16,
    #[allow(missing_docs)]
    pub p_sf: i16,
    #[allow(missing_docs)]
    pub ah_sf: i16,
    #[allow(missing_docs)]
    pub kwh_sf: i16,
    /// Battery Voltage
    pub battv: u16,
    /// Array Voltage
    pub arrayv: u16,
    /// Output Current
    pub outputa: u16,
    /// Array Current
    pub inputa: u16,
    /// Operating State
    pub chargerst: u16,
    /// Output Wattage
    pub outputw: u16,
    /// Today's Minimum Battery Voltage
    pub todayminbatv: u16,
    /// Today's Maximum Battery Voltage
    pub todaymaxbatv: u16,
    /// VOC
    pub vocv: u16,
    /// Today's Maximum VOC
    pub todaymaxvoc: u16,
    /// Today's kWh
    pub todaykwhoutput: u16,
    /// Today's AH
    pub todayahoutput: u16,
    /// Lifetime kWh
    pub lifetimekwhout: u16,
    /// Lifetime kAH
    pub lifetimeahout: u16,
    /// Lifetime Maximum Output Wattage
    pub lifetimemaxout: u16,
    /// Lifetime Maximum Battery Voltage
    pub lifetimemaxbatt: u16,
    /// Lifetime Maximum VOC Voltage
    pub lifetimemaxvoc: u16,
}

#[allow(missing_docs)]

impl Model64111 {
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const P_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const AH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const KWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const BATTV: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const ARRAYV: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const OUTPUTA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const INPUTA: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CHARGERST: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const OUTPUTW: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const TODAYMINBATV: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const TODAYMAXBATV: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const VOCV: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const TODAYMAXVOC: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const TODAYKWHOUTPUT: crate::PointDef<Self, u16> = crate::PointDef::new(16, 1, false);
    pub const TODAYAHOUTPUT: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const LIFETIMEKWHOUT: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, false);
    pub const LIFETIMEAHOUT: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const LIFETIMEMAXOUT: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const LIFETIMEMAXBATT: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const LIFETIMEMAXVOC: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, false);
}

impl crate::Model for Model64111 {
    const ID: u16 = 64111;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            port: Self::PORT.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            p_sf: Self::P_SF.from_data(data)?,
            ah_sf: Self::AH_SF.from_data(data)?,
            kwh_sf: Self::KWH_SF.from_data(data)?,
            battv: Self::BATTV.from_data(data)?,
            arrayv: Self::ARRAYV.from_data(data)?,
            outputa: Self::OUTPUTA.from_data(data)?,
            inputa: Self::INPUTA.from_data(data)?,
            chargerst: Self::CHARGERST.from_data(data)?,
            outputw: Self::OUTPUTW.from_data(data)?,
            todayminbatv: Self::TODAYMINBATV.from_data(data)?,
            todaymaxbatv: Self::TODAYMAXBATV.from_data(data)?,
            vocv: Self::VOCV.from_data(data)?,
            todaymaxvoc: Self::TODAYMAXVOC.from_data(data)?,
            todaykwhoutput: Self::TODAYKWHOUTPUT.from_data(data)?,
            todayahoutput: Self::TODAYAHOUTPUT.from_data(data)?,
            lifetimekwhout: Self::LIFETIMEKWHOUT.from_data(data)?,
            lifetimeahout: Self::LIFETIMEAHOUT.from_data(data)?,
            lifetimemaxout: Self::LIFETIMEMAXOUT.from_data(data)?,
            lifetimemaxbatt: Self::LIFETIMEMAXBATT.from_data(data)?,
            lifetimemaxvoc: Self::LIFETIMEMAXVOC.from_data(data)?,
        })
    }
}
