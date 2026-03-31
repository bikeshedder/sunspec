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
impl crate::Group for Model64111 {
    const LEN: u16 = 23;
}
impl Model64111 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
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
            },
        ))
    }
}
/// Operating State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ChargerSt {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Float,
    #[allow(missing_docs)]
    Bulk,
    #[allow(missing_docs)]
    Absorb,
    #[allow(missing_docs)]
    Eq,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ChargerSt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Float,
            2 => Self::Bulk,
            3 => Self::Absorb,
            4 => Self::Eq,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::Float => 1,
            Self::Bulk => 2,
            Self::Absorb => 3,
            Self::Eq => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ChargerSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Model64111 {
    const ID: u16 = 64111;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64111
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
