//! Basic Charge Controller
/// Basic Charge Controller
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
    pub batt_v: u16,
    /// Array Voltage
    pub array_v: u16,
    /// Output Current
    pub output_a: u16,
    /// Array Current
    pub input_a: u16,
    /// Operating State
    pub charger_st: ChargerSt,
    /// Output Wattage
    pub output_w: u16,
    /// Today's Minimum Battery Voltage
    pub today_min_bat_v: u16,
    /// Today's Maximum Battery Voltage
    pub today_max_bat_v: u16,
    /// VOC
    pub vocv: u16,
    /// Today's Maximum VOC
    pub today_max_voc: u16,
    /// Today's kWh
    pub todayk_wh_output: u16,
    /// Today's AH
    pub today_ah_output: u16,
    /// Lifetime kWh
    pub life_time_kwh_out: u16,
    /// Lifetime kAH
    pub life_time_ah_out: u16,
    /// Lifetime Maximum Output Wattage
    pub life_time_max_out: u16,
    /// Lifetime Maximum Battery Voltage
    pub life_time_max_batt: u16,
    /// Lifetime Maximum VOC Voltage
    pub life_time_max_voc: u16,
}
#[allow(missing_docs)]
impl Model64111 {
    pub const PORT: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(2, 1, false);
    pub const P_SF: crate::Point<Self, i16> = crate::Point::new(3, 1, false);
    pub const AH_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const KWH_SF: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const BATT_V: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const ARRAY_V: crate::Point<Self, u16> = crate::Point::new(7, 1, false);
    pub const OUTPUT_A: crate::Point<Self, u16> = crate::Point::new(8, 1, false);
    pub const INPUT_A: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const CHARGER_ST: crate::Point<Self, ChargerSt> = crate::Point::new(10, 1, false);
    pub const OUTPUT_W: crate::Point<Self, u16> = crate::Point::new(11, 1, false);
    pub const TODAY_MIN_BAT_V: crate::Point<Self, u16> = crate::Point::new(12, 1, false);
    pub const TODAY_MAX_BAT_V: crate::Point<Self, u16> = crate::Point::new(13, 1, false);
    pub const VOCV: crate::Point<Self, u16> = crate::Point::new(14, 1, false);
    pub const TODAY_MAX_VOC: crate::Point<Self, u16> = crate::Point::new(15, 1, false);
    pub const TODAYK_WH_OUTPUT: crate::Point<Self, u16> = crate::Point::new(16, 1, false);
    pub const TODAY_AH_OUTPUT: crate::Point<Self, u16> = crate::Point::new(17, 1, false);
    pub const LIFE_TIME_KWH_OUT: crate::Point<Self, u16> = crate::Point::new(18, 1, false);
    pub const LIFE_TIME_AH_OUT: crate::Point<Self, u16> = crate::Point::new(19, 1, false);
    pub const LIFE_TIME_MAX_OUT: crate::Point<Self, u16> = crate::Point::new(20, 1, false);
    pub const LIFE_TIME_MAX_BATT: crate::Point<Self, u16> = crate::Point::new(21, 1, false);
    pub const LIFE_TIME_MAX_VOC: crate::Point<Self, u16> = crate::Point::new(22, 1, false);
}
impl crate::Model for Model64111 {
    const ID: u16 = 64111;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            port: Self::PORT.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            p_sf: Self::P_SF.from_data(data)?,
            ah_sf: Self::AH_SF.from_data(data)?,
            kwh_sf: Self::KWH_SF.from_data(data)?,
            batt_v: Self::BATT_V.from_data(data)?,
            array_v: Self::ARRAY_V.from_data(data)?,
            output_a: Self::OUTPUT_A.from_data(data)?,
            input_a: Self::INPUT_A.from_data(data)?,
            charger_st: Self::CHARGER_ST.from_data(data)?,
            output_w: Self::OUTPUT_W.from_data(data)?,
            today_min_bat_v: Self::TODAY_MIN_BAT_V.from_data(data)?,
            today_max_bat_v: Self::TODAY_MAX_BAT_V.from_data(data)?,
            vocv: Self::VOCV.from_data(data)?,
            today_max_voc: Self::TODAY_MAX_VOC.from_data(data)?,
            todayk_wh_output: Self::TODAYK_WH_OUTPUT.from_data(data)?,
            today_ah_output: Self::TODAY_AH_OUTPUT.from_data(data)?,
            life_time_kwh_out: Self::LIFE_TIME_KWH_OUT.from_data(data)?,
            life_time_ah_out: Self::LIFE_TIME_AH_OUT.from_data(data)?,
            life_time_max_out: Self::LIFE_TIME_MAX_OUT.from_data(data)?,
            life_time_max_batt: Self::LIFE_TIME_MAX_BATT.from_data(data)?,
            life_time_max_voc: Self::LIFE_TIME_MAX_VOC.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64111
    }
}
/// Operating State
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ChargerSt {
    #[allow(missing_docs)]
    Off = 0,
    #[allow(missing_docs)]
    Float = 1,
    #[allow(missing_docs)]
    Bulk = 2,
    #[allow(missing_docs)]
    Absorb = 3,
    #[allow(missing_docs)]
    Eq = 4,
}
impl crate::Value for ChargerSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ChargerSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ChargerSt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
