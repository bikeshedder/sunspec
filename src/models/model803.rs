//! Lithium-Ion Battery Bank Model
/// Lithium-Ion Battery Bank Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model803 {
    /// String Count
    ///
    /// Number of strings in the bank.
    pub n_str: u16,
    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    pub n_str_con: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub mod_tmp_max: i16,
    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    pub mod_tmp_max_str: Option<u16>,
    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    pub mod_tmp_max_mod: Option<u16>,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub mod_tmp_min: i16,
    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    pub mod_tmp_min_str: Option<u16>,
    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    pub mod_tmp_min_mod: Option<u16>,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub mod_tmp_avg: Option<i16>,
    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub str_v_max: Option<u16>,
    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    pub str_v_max_str: Option<u16>,
    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub str_v_min: Option<u16>,
    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    pub str_v_min_str: Option<u16>,
    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub str_v_avg: Option<u16>,
    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub str_a_max: Option<i16>,
    /// Max String Current String
    ///
    /// String with the maximum current.
    pub str_a_max_str: Option<u16>,
    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub str_a_min: Option<i16>,
    /// Min String Current String
    ///
    /// String with the minimum current.
    pub str_a_min_str: Option<u16>,
    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub str_a_avg: Option<i16>,
    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    pub n_cell_bal: Option<u16>,
    /// Scale factor for cell voltage.
    pub cell_v_sf: i16,
    /// Scale factor for module temperatures.
    pub mod_tmp_sf: i16,
    /// Scale factor for string currents.
    pub a_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model803 {
    pub const N_STR: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const N_STR_CON: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const MOD_TMP_MAX: crate::Point<Self, i16> = crate::Point::new(2, 1, false);
    pub const MOD_TMP_MAX_STR: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const MOD_TMP_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const MOD_TMP_MIN: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const MOD_TMP_MIN_STR: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const MOD_TMP_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    pub const MOD_TMP_AVG: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const STR_V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, false);
    pub const STR_V_MAX_STR: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const STR_V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const STR_V_MIN_STR: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const STR_V_AVG: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const STR_A_MAX: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, false);
    pub const STR_A_MAX_STR: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, false);
    pub const STR_A_MIN: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const STR_A_MIN_STR: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const STR_A_AVG: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, false);
    pub const N_CELL_BAL: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, false);
    pub const CELL_V_SF: crate::Point<Self, i16> = crate::Point::new(20, 1, false);
    pub const MOD_TMP_SF: crate::Point<Self, i16> = crate::Point::new(21, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(22, 1, false);
    pub const SOH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const SOC_SF: crate::Point<Self, i16> = crate::Point::new(24, 1, false);
    pub const V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, false);
}
impl crate::Model for Model803 {
    const ID: u16 = 803;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            n_str: Self::N_STR.from_data(data)?,
            n_str_con: Self::N_STR_CON.from_data(data)?,
            mod_tmp_max: Self::MOD_TMP_MAX.from_data(data)?,
            mod_tmp_max_str: Self::MOD_TMP_MAX_STR.from_data(data)?,
            mod_tmp_max_mod: Self::MOD_TMP_MAX_MOD.from_data(data)?,
            mod_tmp_min: Self::MOD_TMP_MIN.from_data(data)?,
            mod_tmp_min_str: Self::MOD_TMP_MIN_STR.from_data(data)?,
            mod_tmp_min_mod: Self::MOD_TMP_MIN_MOD.from_data(data)?,
            mod_tmp_avg: Self::MOD_TMP_AVG.from_data(data)?,
            str_v_max: Self::STR_V_MAX.from_data(data)?,
            str_v_max_str: Self::STR_V_MAX_STR.from_data(data)?,
            str_v_min: Self::STR_V_MIN.from_data(data)?,
            str_v_min_str: Self::STR_V_MIN_STR.from_data(data)?,
            str_v_avg: Self::STR_V_AVG.from_data(data)?,
            str_a_max: Self::STR_A_MAX.from_data(data)?,
            str_a_max_str: Self::STR_A_MAX_STR.from_data(data)?,
            str_a_min: Self::STR_A_MIN.from_data(data)?,
            str_a_min_str: Self::STR_A_MIN_STR.from_data(data)?,
            str_a_avg: Self::STR_A_AVG.from_data(data)?,
            n_cell_bal: Self::N_CELL_BAL.from_data(data)?,
            cell_v_sf: Self::CELL_V_SF.from_data(data)?,
            mod_tmp_sf: Self::MOD_TMP_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m803
    }
}
