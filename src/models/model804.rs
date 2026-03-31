//! Lithium-Ion String Model
/// Type alias for [`LithiumIonString`].
pub type Model804 = LithiumIonString;
struct Counts {
    n_mod: u16,
}
/// Lithium-Ion String Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct LithiumIonString {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Detail: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Count of modules in the string.
    pub n_mod: u16,
    /// String Status
    ///
    /// Current status of the string.
    pub st: St,
    /// Connection Failure Reason
    pub con_fail: Option<ConFail>,
    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    pub n_cell_bal: Option<u16>,
    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Detail: Measurement.
    pub soc: u16,
    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Detail: Measurement.
    pub do_d: Option<u16>,
    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    pub n_cyc: Option<u32>,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Detail: Measurement.
    pub soh: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    ///
    /// Detail: Measurement.
    pub a: i16,
    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Detail: Measurement.
    pub v: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Detail: Measurement.
    pub cell_v_max: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    pub cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Detail: Measurement.
    pub cell_v_min: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    pub cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Detail: Calculation based on measurements.
    pub cell_v_avg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Detail: Measurement.
    pub mod_tmp_max: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    pub mod_tmp_max_mod: u16,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Detail: Measurement.
    pub mod_tmp_min: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    pub mod_tmp_min_mod: u16,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Detail: Calculation based on measurements.
    pub mod_tmp_avg: i16,
    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    pub con_st: Option<ConSt>,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: Evt1,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    ///
    /// Detail: Reserved for future use.
    pub evt2: Option<Evt2>,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evt_vnd1: Option<EvtVnd1>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evt_vnd2: Option<EvtVnd2>,
    /// Enable/Disable String
    ///
    /// Enables and disables the string.  Should reset to 0 upon completion.
    pub set_ena: Option<u16>,
    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Detail: Should reset to 0 upon completion.
    pub set_con: Option<SetCon>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string depth of discharge.
    pub do_d_sf: Option<i16>,
    /// Scale factor for string current.
    pub a_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
    /// Scale factor for cell voltage.
    pub cell_v_sf: i16,
    /// Scale factor for module temperature.
    pub mod_tmp_sf: i16,
    #[allow(missing_docs)]
    pub lithium_ion_string_module: Vec<LithiumIonStringModule>,
}
#[allow(missing_docs)]
impl LithiumIonString {
    pub const IDX: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const N_MOD: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const ST: crate::Point<Self, St> = crate::Point::new(2, 2, false);
    pub const CON_FAIL: crate::Point<Self, Option<ConFail>> = crate::Point::new(4, 1, false);
    pub const N_CELL_BAL: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const SOC: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const DO_D: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    pub const N_CYC: crate::Point<Self, Option<u32>> = crate::Point::new(8, 2, false);
    pub const SOH: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const A: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const CELL_V_MAX: crate::Point<Self, u16> = crate::Point::new(13, 1, false);
    pub const CELL_V_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const CELL_V_MIN: crate::Point<Self, u16> = crate::Point::new(15, 1, false);
    pub const CELL_V_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, false);
    pub const CELL_V_AVG: crate::Point<Self, u16> = crate::Point::new(17, 1, false);
    pub const MOD_TMP_MAX: crate::Point<Self, i16> = crate::Point::new(18, 1, false);
    pub const MOD_TMP_MAX_MOD: crate::Point<Self, u16> = crate::Point::new(19, 1, false);
    pub const MOD_TMP_MIN: crate::Point<Self, i16> = crate::Point::new(20, 1, false);
    pub const MOD_TMP_MIN_MOD: crate::Point<Self, u16> = crate::Point::new(21, 1, false);
    pub const MOD_TMP_AVG: crate::Point<Self, i16> = crate::Point::new(22, 1, false);
    pub const CON_ST: crate::Point<Self, Option<ConSt>> = crate::Point::new(24, 2, false);
    pub const EVT1: crate::Point<Self, Evt1> = crate::Point::new(26, 2, false);
    pub const EVT2: crate::Point<Self, Option<Evt2>> = crate::Point::new(28, 2, false);
    pub const EVT_VND1: crate::Point<Self, Option<EvtVnd1>> = crate::Point::new(30, 2, false);
    pub const EVT_VND2: crate::Point<Self, Option<EvtVnd2>> = crate::Point::new(32, 2, false);
    pub const SET_ENA: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const SET_CON: crate::Point<Self, Option<SetCon>> = crate::Point::new(35, 1, true);
    pub const SOC_SF: crate::Point<Self, i16> = crate::Point::new(36, 1, false);
    pub const SOH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, false);
    pub const DO_D_SF: crate::Point<Self, Option<i16>> = crate::Point::new(38, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(39, 1, false);
    pub const V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(40, 1, false);
    pub const CELL_V_SF: crate::Point<Self, i16> = crate::Point::new(41, 1, false);
    pub const MOD_TMP_SF: crate::Point<Self, i16> = crate::Point::new(42, 1, false);
}
static LITHIUM_ION_STRING_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "idx",
        label: "String Index",
        description: "Index of the string within the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_mod",
        label: "Module Count",
        description: "Count of modules in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "st",
        label: "String Status",
        description: "Current status of the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "con_fail",
        label: "Connection Failure Reason",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_cell_bal",
        label: "String Cell Balancing Count",
        description: "Number of cells currently being balanced in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc",
        label: "String State of Charge",
        description: "Battery string state of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "do_d",
        label: "String Depth of Discharge",
        description: "Depth of discharge for the string, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_cyc",
        label: "String Cycle Count",
        description: "Number of discharge cycles executed upon the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soh",
        label: "String State of Health",
        description: "Battery string state of health, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a",
        label: "String Current",
        description: "String current measurement.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "String Voltage",
        description: "String voltage measurement.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_max",
        label: "Max Cell Voltage",
        description: "Maximum voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_max_mod",
        label: "Max Cell Voltage Module",
        description: "Module containing the cell with maximum cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_min",
        label: "Min Cell Voltage",
        description: "Minimum voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_min_mod",
        label: "Min Cell Voltage Module",
        description: "Module containing the cell with minimum cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_avg",
        label: "Average Cell Voltage",
        description: "Average voltage for all cells in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_max",
        label: "Max Module Temperature",
        description: "Maximum temperature for all modules in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_max_mod",
        label: "Max Module Temperature Module",
        description: "Module with the maximum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_min",
        label: "Min Module Temperature",
        description: "Minimum temperature for all modules in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_min_mod",
        label: "Min Module Temperature Module",
        description: "Module with the minimum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_tmp_avg",
        label: "Average Module Temperature",
        description: "Average temperature for all modules in the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "con_st",
        label: "Contactor Status",
        description: "Status of the contactor(s) for the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt1",
        label: "String Event 1",
        description: "Alarms, warnings and status values.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt2",
        label: "String Event 2",
        description: "Alarms, warnings and status values.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd1",
        label: "Vendor Event Bitfield 1",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd2",
        label: "Vendor Event Bitfield 2",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "set_ena",
        label: "Enable/Disable String",
        description: "Enables and disables the string.  Should reset to 0 upon completion.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "set_con",
        label: "Connect/Disconnect String",
        description: "Connects and disconnects the string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_sf",
        label: "SoC_SF",
        description: "Scale factor for string state of charge.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soh_sf",
        label: "SoH_SF",
        description: "Scale factor for string state of health.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "do_d_sf",
        label: "DoD_SF",
        description: "Scale factor for string depth of discharge.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "A_SF",
        description: "Scale factor for string current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "V_SF",
        description: "Scale factor for string voltage.",
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
        description: "Scale factor for module temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "lithium_ion_string_module",
        label: "lithium_ion_string_module",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(
            <LithiumIonStringModule as crate::GroupMeta>::group_info,
        ),
    },
];
static LITHIUM_ION_STRING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "lithium_ion_string",
    label: "Lithium-Ion String Model",
    description: "",
    fields: LITHIUM_ION_STRING_FIELDS,
};
impl crate::GroupMeta for LithiumIonString {
    fn group_info() -> &'static crate::GroupInfo {
        &LITHIUM_ION_STRING_GROUP_INFO
    }
}
impl crate::Group for LithiumIonString {
    const LEN: u16 = 46;
}
impl LithiumIonString {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            n_mod: Self::N_MOD.from_data(data)?,
        };
        let (nested_data, lithium_ion_string_module) =
            LithiumIonStringModule::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                idx: Self::IDX.from_data(data)?,
                n_mod: Self::N_MOD.from_data(data)?,
                st: Self::ST.from_data(data)?,
                con_fail: Self::CON_FAIL.from_data(data)?,
                n_cell_bal: Self::N_CELL_BAL.from_data(data)?,
                soc: Self::SOC.from_data(data)?,
                do_d: Self::DO_D.from_data(data)?,
                n_cyc: Self::N_CYC.from_data(data)?,
                soh: Self::SOH.from_data(data)?,
                a: Self::A.from_data(data)?,
                v: Self::V.from_data(data)?,
                cell_v_max: Self::CELL_V_MAX.from_data(data)?,
                cell_v_max_mod: Self::CELL_V_MAX_MOD.from_data(data)?,
                cell_v_min: Self::CELL_V_MIN.from_data(data)?,
                cell_v_min_mod: Self::CELL_V_MIN_MOD.from_data(data)?,
                cell_v_avg: Self::CELL_V_AVG.from_data(data)?,
                mod_tmp_max: Self::MOD_TMP_MAX.from_data(data)?,
                mod_tmp_max_mod: Self::MOD_TMP_MAX_MOD.from_data(data)?,
                mod_tmp_min: Self::MOD_TMP_MIN.from_data(data)?,
                mod_tmp_min_mod: Self::MOD_TMP_MIN_MOD.from_data(data)?,
                mod_tmp_avg: Self::MOD_TMP_AVG.from_data(data)?,
                con_st: Self::CON_ST.from_data(data)?,
                evt1: Self::EVT1.from_data(data)?,
                evt2: Self::EVT2.from_data(data)?,
                evt_vnd1: Self::EVT_VND1.from_data(data)?,
                evt_vnd2: Self::EVT_VND2.from_data(data)?,
                set_ena: Self::SET_ENA.from_data(data)?,
                set_con: Self::SET_CON.from_data(data)?,
                soc_sf: Self::SOC_SF.from_data(data)?,
                soh_sf: Self::SOH_SF.from_data(data)?,
                do_d_sf: Self::DO_D_SF.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                cell_v_sf: Self::CELL_V_SF.from_data(data)?,
                mod_tmp_sf: Self::MOD_TMP_SF.from_data(data)?,
                lithium_ion_string_module,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " String Status"] #[doc = " "] #[doc = " Current status of the string."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct St : u32 {
    #[allow(missing_docs)] const StringEnabled = 1; #[doc =
    " Detail: If string has multiple contactors, indicates that all contactors are closed."]
    const ContactorStatus = 2; }
}
impl crate::Value for St {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for St {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Connection Failure Reason
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ConFail {
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
    /// Detail: See Evt1 for more information.
    StringFault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ConFail {
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
impl crate::FixedSize for ConFail {
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
    ::serde::Deserialize))] pub struct ConSt : u32 { #[allow(missing_docs)] const
    Contactor0 = 1; #[allow(missing_docs)] const Contactor1 = 2; #[allow(missing_docs)]
    const Contactor2 = 4; #[allow(missing_docs)] const Contactor3 = 8;
    #[allow(missing_docs)] const Contactor4 = 16; #[allow(missing_docs)] const Contactor5
    = 32; #[allow(missing_docs)] const Contactor6 = 64; #[allow(missing_docs)] const
    Contactor7 = 128; #[allow(missing_docs)] const Contactor8 = 256;
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
impl crate::Value for ConSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ConSt {
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
    ::serde::Deserialize))] pub struct Evt1 : u32 { #[allow(missing_docs)] const
    CommunicationError = 1; #[allow(missing_docs)] const OverTempAlarm = 2;
    #[allow(missing_docs)] const OverTempWarning = 4; #[allow(missing_docs)] const
    UnderTempAlarm = 8; #[allow(missing_docs)] const UnderTempWarning = 16; #[doc =
    " Detail: See AChaMax in model S 802."] const OverChargeCurrentAlarm = 32; #[doc =
    " Detail: See AChaMax in model S 802."] const OverChargeCurrentWarning = 64; #[doc =
    " Detail: See ADisChaMax in model S 802."] const OverDischargeCurrentAlarm = 128;
    #[doc = " Detail: See ADisChaMax in model S 802."] const OverDischargeCurrentWarning
    = 256; #[allow(missing_docs)] const OverVoltAlarm = 512; #[allow(missing_docs)] const
    OverVoltWarning = 1024; #[allow(missing_docs)] const UnderVoltAlarm = 2048;
    #[allow(missing_docs)] const UnderVoltWarning = 4096; #[allow(missing_docs)] const
    UnderSocMinAlarm = 8192; #[allow(missing_docs)] const UnderSocMinWarning = 16384;
    #[allow(missing_docs)] const OverSocMaxAlarm = 32768; #[allow(missing_docs)] const
    OverSocMaxWarning = 65536; #[allow(missing_docs)] const VoltageImbalanceWarning =
    131072; #[allow(missing_docs)] const TemperatureImbalanceAlarm = 262144;
    #[allow(missing_docs)] const TemperatureImbalanceWarning = 524288;
    #[allow(missing_docs)] const ContactorError = 1048576; #[allow(missing_docs)] const
    FanError = 2097152; #[allow(missing_docs)] const GroundFault = 4194304;
    #[allow(missing_docs)] const OpenDoorError = 8388608; #[doc =
    " Detail: Do not implement."] const Reserved1 = 16777216; #[doc =
    " Detail: See EvtVnd1 and EvtVnd2 for more information."] const OtherAlarm =
    33554432; #[doc = " Detail: See EvtVnd1 and EvtVnd2 for more information."] const
    OtherWarning = 67108864; #[doc = " Detail: Do not implement."] const Reserved2 =
    134217728; #[allow(missing_docs)] const ConfigurationAlarm = 268435456;
    #[allow(missing_docs)] const ConfigurationWarning = 536870912; }
}
impl crate::Value for Evt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " String Event 2"] #[doc = " "] #[doc =
    " Alarms, warnings and status values.  Bit flags."] #[doc = " "] #[doc =
    " Detail: Reserved for future use."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Evt2 : u32 {}
}
impl crate::Value for Evt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 1"] #[doc = " "] #[doc = " Vendor defined events."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd1 : u32 {}
}
impl crate::Value for EvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 2"] #[doc = " "] #[doc = " Vendor defined events."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd2 : u32 {}
}
impl crate::Value for EvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Connect/Disconnect String
///
/// Connects and disconnects the string.
///
/// Detail: Should reset to 0 upon completion.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SetCon {
    #[allow(missing_docs)]
    ConnectString,
    #[allow(missing_docs)]
    DisconnectString,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SetCon {
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
impl crate::FixedSize for SetCon {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct LithiumIonStringModule {
    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    pub mod_n_cell: u16,
    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    pub mod_soc: Option<u16>,
    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    pub mod_soh: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    pub mod_cell_v_max: u16,
    /// Max Cell Voltage Cell
    ///
    /// Cell with maximum voltage.
    pub mod_cell_v_max_cell: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    pub mod_cell_v_min: u16,
    /// Min Cell Voltage Cell
    ///
    /// Cell with minimum voltage.
    pub mod_cell_v_min_cell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    pub mod_cell_v_avg: u16,
    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    pub mod_cell_tmp_max: i16,
    /// Max Cell Temperature Cell
    ///
    /// Cell with maximum temperature.
    pub mod_cell_tmp_max_cell: Option<u16>,
    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    pub mod_cell_tmp_min: i16,
    /// Min Cell Temperature Cell
    ///
    /// Cell with minimum temperature.
    pub mod_cell_tmp_min_cell: Option<u16>,
    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    pub mod_cell_tmp_avg: i16,
}
#[allow(missing_docs)]
impl LithiumIonStringModule {
    pub const MOD_N_CELL: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const MOD_SOC: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const MOD_SOH: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const MOD_CELL_V_MAX: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const MOD_CELL_V_MAX_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const MOD_CELL_V_MIN: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const MOD_CELL_V_MIN_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const MOD_CELL_V_AVG: crate::Point<Self, u16> = crate::Point::new(7, 1, false);
    pub const MOD_CELL_TMP_MAX: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const MOD_CELL_TMP_MAX_CELL: crate::Point<Self, Option<u16>> =
        crate::Point::new(9, 1, false);
    pub const MOD_CELL_TMP_MIN: crate::Point<Self, i16> = crate::Point::new(10, 1, false);
    pub const MOD_CELL_TMP_MIN_CELL: crate::Point<Self, Option<u16>> =
        crate::Point::new(11, 1, false);
    pub const MOD_CELL_TMP_AVG: crate::Point<Self, i16> = crate::Point::new(12, 1, false);
}
static LITHIUM_ION_STRING_MODULE_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "mod_n_cell",
        label: "Module Cell Count",
        description: "Count of all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_soc",
        label: "Module SoC",
        description: "Module state of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_soh",
        label: "Module SoH",
        description: "Module state of health, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_v_max",
        label: "Max Cell Voltage",
        description: "Maximum voltage for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_v_max_cell",
        label: "Max Cell Voltage Cell",
        description: "Cell with maximum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_v_min",
        label: "Min Cell Voltage",
        description: "Minimum voltage for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_v_min_cell",
        label: "Min Cell Voltage Cell",
        description: "Cell with minimum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_v_avg",
        label: "Average Cell Voltage",
        description: "Average voltage for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_tmp_max",
        label: "Max Cell Temperature",
        description: "Maximum temperature for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_tmp_max_cell",
        label: "Max Cell Temperature Cell",
        description: "Cell with maximum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_tmp_min",
        label: "Min Cell Temperature",
        description: "Minimum temperature for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_tmp_min_cell",
        label: "Min Cell Temperature Cell",
        description: "Cell with minimum temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_cell_tmp_avg",
        label: "Average Cell Temperature",
        description: "Average temperature for all cells in the module.",
        kind: crate::FieldKind::Point,
    },
];
static LITHIUM_ION_STRING_MODULE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "lithium_ion_string_module",
    label: "lithium_ion_string_module",
    description: "",
    fields: LITHIUM_ION_STRING_MODULE_FIELDS,
};
impl crate::GroupMeta for LithiumIonStringModule {
    fn group_info() -> &'static crate::GroupInfo {
        &LITHIUM_ION_STRING_MODULE_GROUP_INFO
    }
}
impl crate::Group for LithiumIonStringModule {
    const LEN: u16 = 16;
}
impl LithiumIonStringModule {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                mod_n_cell: Self::MOD_N_CELL.from_data(data)?,
                mod_soc: Self::MOD_SOC.from_data(data)?,
                mod_soh: Self::MOD_SOH.from_data(data)?,
                mod_cell_v_max: Self::MOD_CELL_V_MAX.from_data(data)?,
                mod_cell_v_max_cell: Self::MOD_CELL_V_MAX_CELL.from_data(data)?,
                mod_cell_v_min: Self::MOD_CELL_V_MIN.from_data(data)?,
                mod_cell_v_min_cell: Self::MOD_CELL_V_MIN_CELL.from_data(data)?,
                mod_cell_v_avg: Self::MOD_CELL_V_AVG.from_data(data)?,
                mod_cell_tmp_max: Self::MOD_CELL_TMP_MAX.from_data(data)?,
                mod_cell_tmp_max_cell: Self::MOD_CELL_TMP_MAX_CELL.from_data(data)?,
                mod_cell_tmp_min: Self::MOD_CELL_TMP_MIN.from_data(data)?,
                mod_cell_tmp_min_cell: Self::MOD_CELL_TMP_MIN_CELL.from_data(data)?,
                mod_cell_tmp_avg: Self::MOD_CELL_TMP_AVG.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_mod).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = LithiumIonStringModule::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for LithiumIonString {
    const ID: u16 = 804;
    const NAME: &'static str = "lithium_ion_string";
    const LABEL: &'static str = "Lithium-Ion String Model";
    const DESCRIPTION: &'static str = "";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m804
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
