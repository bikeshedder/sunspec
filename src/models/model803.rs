//! Lithium-Ion Battery Bank Model
/// Type alias for [`LithiumIonBank`].
pub type Model803 = LithiumIonBank;
struct Counts {
    n_str: u16,
}
/// Lithium-Ion Battery Bank Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct LithiumIonBank {
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
    /// Detail: Measurement.
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
    /// Detail: Measurement.
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
    /// Detail: Calculation based on measurements.
    pub mod_tmp_avg: Option<i16>,
    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Detail: Measurement.
    pub str_v_max: Option<u16>,
    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    pub str_v_max_str: Option<u16>,
    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Detail: Measurement.
    pub str_v_min: Option<u16>,
    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    pub str_v_min_str: Option<u16>,
    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Detail: Calculation based on measurements.
    pub str_v_avg: Option<u16>,
    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Detail: Measurement.
    pub str_a_max: Option<i16>,
    /// Max String Current String
    ///
    /// String with the maximum current.
    pub str_a_max_str: Option<u16>,
    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Detail: Measurement.
    pub str_a_min: Option<i16>,
    /// Min String Current String
    ///
    /// String with the minimum current.
    pub str_a_min_str: Option<u16>,
    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Detail: Calculation based on measurements.
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
    #[allow(missing_docs)]
    pub string: Vec<String>,
}
#[allow(missing_docs)]
impl LithiumIonBank {
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
static LITHIUM_ION_BANK_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "n_str",
        label: "String Count",
        description: "Number of strings in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_str_con",
        label: "Connected String Count",
        description: "Number of strings with contactor closed.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_max",
        label: "Max Module Temperature",
        description: "Maximum temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_max_str",
        label: "Max Module Temperature String",
        description: "String containing the module with maximum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_max_mod",
        label: "Max Module Temperature Module",
        description: "Module with maximum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_min",
        label: "Min Module Temperature",
        description: "Minimum temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_min_str",
        label: "Min Module Temperature String",
        description: "String containing the module with minimum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_min_mod",
        label: "Min Module Temperature Module",
        description: "Module with minimum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_avg",
        label: "Average Module Temperature",
        description: "Average temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_v_max",
        label: "Max String Voltage",
        description: "Maximum string voltage for all strings in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_v_max_str",
        label: "Max String Voltage String",
        description: "String with maximum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_v_min",
        label: "Min String Voltage",
        description: "Minimum string voltage for all strings in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_v_min_str",
        label: "Min String Voltage String",
        description: "String with minimum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_v_avg",
        label: "Average String Voltage",
        description: "Average string voltage for all strings in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a_max",
        label: "Max String Current",
        description: "Maximum current of any string in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a_max_str",
        label: "Max String Current String",
        description: "String with the maximum current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a_min",
        label: "Min String Current",
        description: "Minimum current of any string in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a_min_str",
        label: "Min String Current String",
        description: "String with the minimum current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a_avg",
        label: "Average String Current",
        description: "Average string current for all strings in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_cell_bal",
        label: "Battery Cell Balancing Count",
        description: "Total number of cells that are currently being balanced.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_sf",
        label: "CellV_SF",
        description: "Scale factor for cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_sf",
        label: "ModTmp_SF",
        description: "Scale factor for module temperatures.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "A_SF",
        description: "Scale factor for string currents.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soh_sf",
        label: "SoH_SF",
        description: "Scale factor for string state of health.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_sf",
        label: "SoC_SF",
        description: "Scale factor for string state of charge.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "V_SF",
        description: "Scale factor for string voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "string",
        label: "string",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<String as crate::GroupMeta>::group_info),
    },
];
static LITHIUM_ION_BANK_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "lithium_ion_bank",
    label: "Lithium-Ion Battery Bank Model",
    description: "",
    fields: LITHIUM_ION_BANK_FIELDS,
};
impl crate::GroupMeta for LithiumIonBank {
    fn group_info() -> &'static crate::GroupInfo {
        &LITHIUM_ION_BANK_GROUP_INFO
    }
}
impl crate::Group for LithiumIonBank {
    const LEN: u16 = 26;
}
impl LithiumIonBank {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            n_str: Self::N_STR.from_data(data)?,
        };
        let (nested_data, string) = String::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
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
                string,
            },
        ))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct String {
    /// Module Count
    ///
    /// Count of modules in the string.
    pub str_n_mod: u16,
    /// String Status
    ///
    /// Current status of the string.
    pub str_st: StringStrSt,
    /// Connection Failure Reason
    pub str_con_fail: Option<StringStrConFail>,
    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    pub str_soc: u16,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    pub str_soh: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    pub str_a: i16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    pub str_cell_v_max: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the maximum cell voltage.
    pub str_cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    pub str_cell_v_min: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the minimum cell voltage.
    pub str_cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    pub str_cell_v_avg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    pub str_mod_tmp_max: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    pub str_mod_tmp_max_mod: Option<u16>,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    pub str_mod_tmp_min: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    pub str_mod_tmp_min_mod: Option<u16>,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    pub str_mod_tmp_avg: i16,
    /// Disabled Reason
    ///
    /// Reason why the string is currently disabled.
    pub str_dis_rsn: Option<StringStrDisRsn>,
    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    pub str_con_st: Option<StringStrConSt>,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub str_evt1: StringStrEvt1,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub str_evt2: Option<StringStrEvt2>,
    /// Vendor String Event Bitfield 1
    ///
    /// Vendor defined events.
    pub str_evt_vnd1: Option<StringStrEvtVnd1>,
    /// Vendor String Event Bitfield 2
    ///
    /// Vendor defined events.
    pub str_evt_vnd2: Option<StringStrEvtVnd2>,
    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    pub str_set_ena: Option<StringStrSetEna>,
    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    pub str_set_con: Option<StringStrSetCon>,
}
#[allow(missing_docs)]
impl String {
    pub const STR_N_MOD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const STR_ST: crate::Point<Self, StringStrSt> = crate::Point::new(1, 2, false);
    pub const STR_CON_FAIL: crate::Point<Self, Option<StringStrConFail>> =
        crate::Point::new(3, 1, false);
    pub const STR_SOC: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const STR_SOH: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const STR_A: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
    pub const STR_CELL_V_MAX: crate::Point<Self, u16> = crate::Point::new(7, 1, false);
    pub const STR_CELL_V_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const STR_CELL_V_MIN: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const STR_CELL_V_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const STR_CELL_V_AVG: crate::Point<Self, u16> = crate::Point::new(11, 1, false);
    pub const STR_MOD_TMP_MAX: crate::Point<Self, i16> = crate::Point::new(12, 1, false);
    pub const STR_MOD_TMP_MAX_MOD: crate::Point<Self, Option<u16>> =
        crate::Point::new(13, 1, false);
    pub const STR_MOD_TMP_MIN: crate::Point<Self, i16> = crate::Point::new(14, 1, false);
    pub const STR_MOD_TMP_MIN_MOD: crate::Point<Self, Option<u16>> =
        crate::Point::new(15, 1, false);
    pub const STR_MOD_TMP_AVG: crate::Point<Self, i16> = crate::Point::new(16, 1, false);
    pub const STR_DIS_RSN: crate::Point<Self, Option<StringStrDisRsn>> =
        crate::Point::new(17, 1, false);
    pub const STR_CON_ST: crate::Point<Self, Option<StringStrConSt>> =
        crate::Point::new(18, 2, false);
    pub const STR_EVT1: crate::Point<Self, StringStrEvt1> = crate::Point::new(20, 2, false);
    pub const STR_EVT2: crate::Point<Self, Option<StringStrEvt2>> = crate::Point::new(22, 2, false);
    pub const STR_EVT_VND1: crate::Point<Self, Option<StringStrEvtVnd1>> =
        crate::Point::new(24, 2, false);
    pub const STR_EVT_VND2: crate::Point<Self, Option<StringStrEvtVnd2>> =
        crate::Point::new(26, 2, false);
    pub const STR_SET_ENA: crate::Point<Self, Option<StringStrSetEna>> =
        crate::Point::new(28, 1, true);
    pub const STR_SET_CON: crate::Point<Self, Option<StringStrSetCon>> =
        crate::Point::new(29, 1, true);
}
static STRING_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "str_n_mod",
        label: "Module Count",
        description: "Count of modules in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_st",
        label: "String Status",
        description: "Current status of the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_con_fail",
        label: "Connection Failure Reason",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_soc",
        label: "String State of Charge",
        description: "Battery string state of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_soh",
        label: "String State of Health",
        description: "Battery string state of health, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_a",
        label: "String Current",
        description: "String current measurement.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_cell_v_max",
        label: "Max Cell Voltage",
        description: "Maximum voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_cell_v_max_mod",
        label: "Max Cell Voltage Module",
        description: "Module containing the maximum cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_cell_v_min",
        label: "Min Cell Voltage",
        description: "Minimum voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_cell_v_min_mod",
        label: "Min Cell Voltage Module",
        description: "Module containing the minimum cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_cell_v_avg",
        label: "Average Cell Voltage",
        description: "Average voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_mod_tmp_max",
        label: "Max Module Temperature",
        description: "Maximum temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_mod_tmp_max_mod",
        label: "Max Module Temperature Module",
        description: "Module with the maximum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_mod_tmp_min",
        label: "Min Module Temperature",
        description: "Minimum temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_mod_tmp_min_mod",
        label: "Min Module Temperature Module",
        description: "Module with the minimum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_mod_tmp_avg",
        label: "Average Module Temperature",
        description: "Average temperature for all modules in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_dis_rsn",
        label: "Disabled Reason",
        description: "Reason why the string is currently disabled.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_con_st",
        label: "Contactor Status",
        description: "Status of the contactor(s) for the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_evt1",
        label: "String Event 1",
        description: "Alarms, warnings and status values.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_evt2",
        label: "String Event 2",
        description: "Alarms, warnings and status values.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_evt_vnd1",
        label: "Vendor String Event Bitfield 1",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_evt_vnd2",
        label: "Vendor String Event Bitfield 2",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_set_ena",
        label: "Enable/Disable String",
        description: "Enables and disables the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "str_set_con",
        label: "Connect/Disconnect String",
        description: "Connects and disconnects the string.",
        kind: crate::FieldKind::Point,
    },
];
static STRING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "string",
    label: "string",
    description: "",
    fields: STRING_FIELDS,
};
impl crate::GroupMeta for String {
    fn group_info() -> &'static crate::GroupInfo {
        &STRING_GROUP_INFO
    }
}
impl crate::Group for String {
    const LEN: u16 = 32;
}
impl String {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                str_n_mod: Self::STR_N_MOD.from_data(data)?,
                str_st: Self::STR_ST.from_data(data)?,
                str_con_fail: Self::STR_CON_FAIL.from_data(data)?,
                str_soc: Self::STR_SOC.from_data(data)?,
                str_soh: Self::STR_SOH.from_data(data)?,
                str_a: Self::STR_A.from_data(data)?,
                str_cell_v_max: Self::STR_CELL_V_MAX.from_data(data)?,
                str_cell_v_max_mod: Self::STR_CELL_V_MAX_MOD.from_data(data)?,
                str_cell_v_min: Self::STR_CELL_V_MIN.from_data(data)?,
                str_cell_v_min_mod: Self::STR_CELL_V_MIN_MOD.from_data(data)?,
                str_cell_v_avg: Self::STR_CELL_V_AVG.from_data(data)?,
                str_mod_tmp_max: Self::STR_MOD_TMP_MAX.from_data(data)?,
                str_mod_tmp_max_mod: Self::STR_MOD_TMP_MAX_MOD.from_data(data)?,
                str_mod_tmp_min: Self::STR_MOD_TMP_MIN.from_data(data)?,
                str_mod_tmp_min_mod: Self::STR_MOD_TMP_MIN_MOD.from_data(data)?,
                str_mod_tmp_avg: Self::STR_MOD_TMP_AVG.from_data(data)?,
                str_dis_rsn: Self::STR_DIS_RSN.from_data(data)?,
                str_con_st: Self::STR_CON_ST.from_data(data)?,
                str_evt1: Self::STR_EVT1.from_data(data)?,
                str_evt2: Self::STR_EVT2.from_data(data)?,
                str_evt_vnd1: Self::STR_EVT_VND1.from_data(data)?,
                str_evt_vnd2: Self::STR_EVT_VND2.from_data(data)?,
                str_set_ena: Self::STR_SET_ENA.from_data(data)?,
                str_set_con: Self::STR_SET_CON.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_str).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = String::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " String Status"] #[doc = " "] #[doc = " Current status of the string."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct StringStrSt : u32 {
    #[allow(missing_docs)] const StringEnabled = 1; #[allow(missing_docs)] const
    ContactorStatus = 2; }
}
impl crate::Value for StringStrSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrSt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Connection Failure Reason
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum StringStrConFail {
    #[allow(missing_docs)]
    NoFailure,
    #[allow(missing_docs)]
    ButtonPushed,
    #[allow(missing_docs)]
    StrGroundFault,
    #[allow(missing_docs)]
    OutsideVoltageRange,
    #[allow(missing_docs)]
    StringNotEnabled,
    #[allow(missing_docs)]
    FuseOpen,
    #[allow(missing_docs)]
    ContactorFailure,
    #[allow(missing_docs)]
    PrechargeFailure,
    #[allow(missing_docs)]
    StringFault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for StringStrConFail {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::NoFailure,
            1 => Self::ButtonPushed,
            2 => Self::StrGroundFault,
            3 => Self::OutsideVoltageRange,
            4 => Self::StringNotEnabled,
            5 => Self::FuseOpen,
            6 => Self::ContactorFailure,
            7 => Self::PrechargeFailure,
            8 => Self::StringFault,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::NoFailure => 0,
            Self::ButtonPushed => 1,
            Self::StrGroundFault => 2,
            Self::OutsideVoltageRange => 3,
            Self::StringNotEnabled => 4,
            Self::FuseOpen => 5,
            Self::ContactorFailure => 6,
            Self::PrechargeFailure => 7,
            Self::StringFault => 8,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for StringStrConFail {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Disabled Reason
///
/// Reason why the string is currently disabled.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum StringStrDisRsn {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    Fault,
    #[allow(missing_docs)]
    Maintenance,
    #[allow(missing_docs)]
    External,
    #[allow(missing_docs)]
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for StringStrDisRsn {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Fault,
            2 => Self::Maintenance,
            3 => Self::External,
            4 => Self::Other,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::Fault => 1,
            Self::Maintenance => 2,
            Self::External => 3,
            Self::Other => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for StringStrDisRsn {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Contactor Status"] #[doc = " "] #[doc =
    " Status of the contactor(s) for the string."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct StringStrConSt : u32 { #[allow(missing_docs)]
    const Contactor0 = 1; #[allow(missing_docs)] const Contactor1 = 2;
    #[allow(missing_docs)] const Contactor2 = 4; #[allow(missing_docs)] const Contactor3
    = 8; #[allow(missing_docs)] const Contactor4 = 16; #[allow(missing_docs)] const
    Contactor5 = 32; #[allow(missing_docs)] const Contactor6 = 64; #[allow(missing_docs)]
    const Contactor7 = 128; #[allow(missing_docs)] const Contactor8 = 256;
    #[allow(missing_docs)] const Contactor9 = 512; #[allow(missing_docs)] const
    Contactor10 = 1024; #[allow(missing_docs)] const Contactor11 = 2048;
    #[allow(missing_docs)] const Contactor12 = 4096; #[allow(missing_docs)] const
    Contactor13 = 8192; #[allow(missing_docs)] const Contactor14 = 16384;
    #[allow(missing_docs)] const Contactor15 = 32768; #[allow(missing_docs)] const
    Contactor16 = 65536; #[allow(missing_docs)] const Contactor17 = 131072;
    #[allow(missing_docs)] const Contactor18 = 262144; #[allow(missing_docs)] const
    Contactor19 = 524288; #[allow(missing_docs)] const Contactor20 = 1048576;
    #[allow(missing_docs)] const Contactor21 = 2097152; #[allow(missing_docs)] const
    Contactor22 = 4194304; #[allow(missing_docs)] const Contactor23 = 8388608;
    #[allow(missing_docs)] const Contactor24 = 16777216; #[allow(missing_docs)] const
    Contactor25 = 33554432; #[allow(missing_docs)] const Contactor26 = 67108864;
    #[allow(missing_docs)] const Contactor27 = 134217728; #[allow(missing_docs)] const
    Contactor28 = 268435456; #[allow(missing_docs)] const Contactor29 = 536870912;
    #[allow(missing_docs)] const Contactor30 = 1073741824; }
}
impl crate::Value for StringStrConSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrConSt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " String Event 1"] #[doc = " "] #[doc =
    " Alarms, warnings and status values.  Bit flags."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct StringStrEvt1 : u32 { #[allow(missing_docs)] const
    CommunicationError = 1; #[allow(missing_docs)] const OverTempAlarm = 2;
    #[allow(missing_docs)] const OverTempWarning = 4; #[allow(missing_docs)] const
    UnderTempAlarm = 8; #[allow(missing_docs)] const UnderTempWarning = 16;
    #[allow(missing_docs)] const OverChargeCurrentAlarm = 32; #[allow(missing_docs)]
    const OverChargeCurrentWarning = 64; #[allow(missing_docs)] const
    OverDischargeCurrentAlarm = 128; #[allow(missing_docs)] const
    OverDischargeCurrentWarning = 256; #[allow(missing_docs)] const OverVoltAlarm = 512;
    #[allow(missing_docs)] const OverVoltWarning = 1024; #[allow(missing_docs)] const
    UnderVoltAlarm = 2048; #[allow(missing_docs)] const UnderVoltWarning = 4096;
    #[allow(missing_docs)] const UnderSocMinAlarm = 8192; #[allow(missing_docs)] const
    UnderSocMinWarning = 16384; #[allow(missing_docs)] const OverSocMaxAlarm = 32768;
    #[allow(missing_docs)] const OverSocMaxWarning = 65536; #[allow(missing_docs)] const
    VoltageImbalanceWarning = 131072; #[allow(missing_docs)] const
    TemperatureImbalanceAlarm = 262144; #[allow(missing_docs)] const
    TemperatureImbalanceWarning = 524288; #[allow(missing_docs)] const ContactorError =
    1048576; #[allow(missing_docs)] const FanError = 2097152; #[allow(missing_docs)]
    const GroundFault = 4194304; #[allow(missing_docs)] const OpenDoorError = 8388608;
    #[allow(missing_docs)] const Reserved1 = 16777216; #[allow(missing_docs)] const
    OtherAlarm = 33554432; #[allow(missing_docs)] const OtherWarning = 67108864;
    #[allow(missing_docs)] const Reserved2 = 134217728; #[allow(missing_docs)] const
    ConfigurationAlarm = 268435456; #[allow(missing_docs)] const ConfigurationWarning =
    536870912; }
}
impl crate::Value for StringStrEvt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrEvt1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " String Event 2"] #[doc = " "] #[doc =
    " Alarms, warnings and status values.  Bit flags."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct StringStrEvt2 : u32 {}
}
impl crate::Value for StringStrEvt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrEvt2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor String Event Bitfield 1"] #[doc = " "] #[doc =
    " Vendor defined events."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct StringStrEvtVnd1 : u32 {}
}
impl crate::Value for StringStrEvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrEvtVnd1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor String Event Bitfield 2"] #[doc = " "] #[doc =
    " Vendor defined events."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct StringStrEvtVnd2 : u32 {}
}
impl crate::Value for StringStrEvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringStrEvtVnd2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Enable/Disable String
///
/// Enables and disables the string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum StringStrSetEna {
    #[allow(missing_docs)]
    EnableString,
    #[allow(missing_docs)]
    DisableString,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for StringStrSetEna {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::EnableString,
            2 => Self::DisableString,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::EnableString => 1,
            Self::DisableString => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for StringStrSetEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Connect/Disconnect String
///
/// Connects and disconnects the string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum StringStrSetCon {
    #[allow(missing_docs)]
    ConnectString,
    #[allow(missing_docs)]
    DisconnectString,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for StringStrSetCon {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::ConnectString,
            2 => Self::DisconnectString,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::ConnectString => 1,
            Self::DisconnectString => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for StringStrSetCon {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for LithiumIonBank {
    const ID: u16 = 803;
    const NAME: &'static str = "lithium_ion_bank";
    const LABEL: &'static str = "Lithium-Ion Battery Bank Model";
    const DESCRIPTION: &'static str = "";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m803
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
