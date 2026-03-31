//! Flow Battery String Model
/// Type alias for [`FlowBatteryString`].
pub type Model807 = FlowBatteryString;
/// Flow Battery String Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FlowBatteryString {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Detail: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Number of modules in this string.
    pub n_mod: u16,
    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    pub n_mod_con: u16,
    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Detail: Measurement.
    pub mod_v_max: u16,
    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    pub mod_v_max_mod: Option<u16>,
    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Detail: Measurement.
    pub mod_v_min: u16,
    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    pub mod_v_min_mod: Option<u16>,
    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Detail: Calculation based on measurements.
    pub mod_v_avg: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Detail: Measurement.
    pub cell_v_max: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    pub cell_v_max_mod: Option<u16>,
    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    pub cell_v_max_stk: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Detail: Measurement.
    pub cell_v_min: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    pub cell_v_min_mod: Option<u16>,
    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    pub cell_v_min_stk: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Detail: Calculation based on measurements.
    pub cell_v_avg: Option<u16>,
    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Detail: Measurement.
    pub tmp_max: i16,
    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    pub tmp_max_mod: Option<u16>,
    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Detail: Measurement.
    pub tmp_min: i16,
    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    pub tmp_min_mod: Option<u16>,
    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Detail: Calculation based on measurements.
    pub tmp_avg: i16,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: Evt1,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt2: Evt2,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evt_vnd1: EvtVnd1,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evt_vnd2: EvtVnd2,
    #[allow(missing_docs)]
    pub mod_v_sf: i16,
    /// Scale factor for voltage.
    pub cell_v_sf: i16,
    /// Scale factor for temperature.
    pub tmp_sf: i16,
    /// Scale factor for state of charge.
    pub soc_sf: i16,
    /// Scale factor for open circuit voltage.
    pub ocv_sf: i16,
    #[allow(missing_docs)]
    pub module: Vec<Module>,
}
#[allow(missing_docs)]
impl FlowBatteryString {
    pub const IDX: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const N_MOD: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const N_MOD_CON: crate::Point<Self, u16> = crate::Point::new(2, 1, false);
    pub const MOD_V_MAX: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const MOD_V_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const MOD_V_MIN: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const MOD_V_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const MOD_V_AVG: crate::Point<Self, u16> = crate::Point::new(7, 1, false);
    pub const CELL_V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const CELL_V_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, false);
    pub const CELL_V_MAX_STK: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const CELL_V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const CELL_V_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const CELL_V_MIN_STK: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const CELL_V_AVG: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const TMP_MAX: crate::Point<Self, i16> = crate::Point::new(15, 1, false);
    pub const TMP_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, false);
    pub const TMP_MIN: crate::Point<Self, i16> = crate::Point::new(17, 1, false);
    pub const TMP_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, false);
    pub const TMP_AVG: crate::Point<Self, i16> = crate::Point::new(19, 1, false);
    pub const EVT1: crate::Point<Self, Evt1> = crate::Point::new(20, 2, false);
    pub const EVT2: crate::Point<Self, Evt2> = crate::Point::new(22, 2, false);
    pub const EVT_VND1: crate::Point<Self, EvtVnd1> = crate::Point::new(24, 2, false);
    pub const EVT_VND2: crate::Point<Self, EvtVnd2> = crate::Point::new(26, 2, false);
    pub const MOD_V_SF: crate::Point<Self, i16> = crate::Point::new(28, 1, false);
    pub const CELL_V_SF: crate::Point<Self, i16> = crate::Point::new(29, 1, false);
    pub const TMP_SF: crate::Point<Self, i16> = crate::Point::new(30, 1, false);
    pub const SOC_SF: crate::Point<Self, i16> = crate::Point::new(31, 1, false);
    pub const OCV_SF: crate::Point<Self, i16> = crate::Point::new(32, 1, false);
}
impl crate::Group for FlowBatteryString {
    const LEN: u16 = 34;
}
impl FlowBatteryString {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, module) = Module::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                idx: Self::IDX.from_data(data)?,
                n_mod: Self::N_MOD.from_data(data)?,
                n_mod_con: Self::N_MOD_CON.from_data(data)?,
                mod_v_max: Self::MOD_V_MAX.from_data(data)?,
                mod_v_max_mod: Self::MOD_V_MAX_MOD.from_data(data)?,
                mod_v_min: Self::MOD_V_MIN.from_data(data)?,
                mod_v_min_mod: Self::MOD_V_MIN_MOD.from_data(data)?,
                mod_v_avg: Self::MOD_V_AVG.from_data(data)?,
                cell_v_max: Self::CELL_V_MAX.from_data(data)?,
                cell_v_max_mod: Self::CELL_V_MAX_MOD.from_data(data)?,
                cell_v_max_stk: Self::CELL_V_MAX_STK.from_data(data)?,
                cell_v_min: Self::CELL_V_MIN.from_data(data)?,
                cell_v_min_mod: Self::CELL_V_MIN_MOD.from_data(data)?,
                cell_v_min_stk: Self::CELL_V_MIN_STK.from_data(data)?,
                cell_v_avg: Self::CELL_V_AVG.from_data(data)?,
                tmp_max: Self::TMP_MAX.from_data(data)?,
                tmp_max_mod: Self::TMP_MAX_MOD.from_data(data)?,
                tmp_min: Self::TMP_MIN.from_data(data)?,
                tmp_min_mod: Self::TMP_MIN_MOD.from_data(data)?,
                tmp_avg: Self::TMP_AVG.from_data(data)?,
                evt1: Self::EVT1.from_data(data)?,
                evt2: Self::EVT2.from_data(data)?,
                evt_vnd1: Self::EVT_VND1.from_data(data)?,
                evt_vnd2: Self::EVT_VND2.from_data(data)?,
                mod_v_sf: Self::MOD_V_SF.from_data(data)?,
                cell_v_sf: Self::CELL_V_SF.from_data(data)?,
                tmp_sf: Self::TMP_SF.from_data(data)?,
                soc_sf: Self::SOC_SF.from_data(data)?,
                ocv_sf: Self::OCV_SF.from_data(data)?,
                module,
            },
        ))
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
    131072; #[doc = " Detail: Do not implement."] const Reserved1 = 262144; #[doc =
    " Detail: Do not implement."] const Reserved2 = 524288; #[allow(missing_docs)] const
    ContactorError = 1048576; #[allow(missing_docs)] const FanError = 2097152;
    #[allow(missing_docs)] const GroundFault = 4194304; #[allow(missing_docs)] const
    OpenDoorError = 8388608; #[doc = " Detail: Do not implement."] const Reserved3 =
    16777216; #[doc = " Detail: See EvtVnd1 and EvtVnd2 for more information."] const
    OtherAlarm = 33554432; #[doc =
    " Detail: See EvtVnd1 and EvtVnd2 for more information."] const OtherWarning =
    67108864; #[allow(missing_docs)] const FireAlarm = 134217728; #[allow(missing_docs)]
    const ConfigurationAlarm = 268435456; #[allow(missing_docs)] const
    ConfigurationWarning = 536870912; }
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
    " Alarms, warnings and status values.  Bit flags."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Evt2 : u32 { #[allow(missing_docs)] const
    LeakAlarm = 1; #[allow(missing_docs)] const PumpAlarm = 2; #[allow(missing_docs)]
    const HighPressureAlarm = 4; #[allow(missing_docs)] const HighPressureWarning = 8;
    #[allow(missing_docs)] const LowFlowAlarm = 16; #[allow(missing_docs)] const
    LowFlowWarning = 32; }
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
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Module {
    /// Module Index
    ///
    /// Index of the module within the string.
    pub mod_idx: u16,
    /// Stack Count
    ///
    /// Number of stacks in this module.
    pub mod_n_stk: u16,
    /// Module Status
    ///
    /// Current status of the module.
    pub mod_st: ModuleModSt,
    /// Module State of Charge
    ///
    /// State of charge for this module.
    pub mod_soc: u16,
    /// Open Circuit Voltage
    ///
    /// Open circuit voltage for this module.
    pub mod_ocv: u16,
    /// External Voltage
    ///
    /// External voltage fo this module.
    pub mod_v: u16,
    /// Maximum Cell Voltage
    ///
    /// Maximum voltage for all cells in this module.
    pub mod_cell_v_max: Option<u16>,
    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum cell voltage.
    pub mod_cell_v_max_cell: Option<u16>,
    /// Minimum Cell Voltage
    ///
    /// Minimum voltage for all cells in this module.
    pub mod_cell_v_min: Option<u16>,
    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum cell voltage.
    pub mod_cell_v_min_cell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in this module.
    pub mod_cell_v_avg: Option<u16>,
    /// Anolyte Temperature
    pub mod_ano_tmp: Option<u16>,
    /// Catholyte Temperature
    pub mod_cat_tmp: Option<u16>,
    /// Contactor Status
    pub mod_con_st: Option<ModuleModConSt>,
    /// Module Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub mod_evt1: ModuleModEvt1,
    /// Module Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub mod_evt2: ModuleModEvt2,
    /// Connection Failure Reason
    pub mod_con_fail: Option<ModuleModConFail>,
    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    pub mod_set_ena: Option<ModuleModSetEna>,
    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    pub mod_set_con: Option<ModuleModSetCon>,
    /// Disabled Reason
    ///
    /// Reason why the module is currently disabled.
    pub mod_dis_rsn: Option<ModuleModDisRsn>,
}
#[allow(missing_docs)]
impl Module {
    pub const MOD_IDX: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const MOD_N_STK: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const MOD_ST: crate::Point<Self, ModuleModSt> = crate::Point::new(2, 2, false);
    pub const MOD_SOC: crate::Point<Self, u16> = crate::Point::new(4, 1, false);
    pub const MOD_OCV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const MOD_V: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const MOD_CELL_V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    pub const MOD_CELL_V_MAX_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const MOD_CELL_V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, false);
    pub const MOD_CELL_V_MIN_CELL: crate::Point<Self, Option<u16>> =
        crate::Point::new(10, 1, false);
    pub const MOD_CELL_V_AVG: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const MOD_ANO_TMP: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const MOD_CAT_TMP: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const MOD_CON_ST: crate::Point<Self, Option<ModuleModConSt>> =
        crate::Point::new(14, 2, false);
    pub const MOD_EVT1: crate::Point<Self, ModuleModEvt1> = crate::Point::new(16, 2, false);
    pub const MOD_EVT2: crate::Point<Self, ModuleModEvt2> = crate::Point::new(18, 2, false);
    pub const MOD_CON_FAIL: crate::Point<Self, Option<ModuleModConFail>> =
        crate::Point::new(20, 1, false);
    pub const MOD_SET_ENA: crate::Point<Self, Option<ModuleModSetEna>> =
        crate::Point::new(21, 1, true);
    pub const MOD_SET_CON: crate::Point<Self, Option<ModuleModSetCon>> =
        crate::Point::new(22, 1, true);
    pub const MOD_DIS_RSN: crate::Point<Self, Option<ModuleModDisRsn>> =
        crate::Point::new(23, 1, false);
}
impl crate::Group for Module {
    const LEN: u16 = 24;
}
impl Module {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                mod_idx: Self::MOD_IDX.from_data(data)?,
                mod_n_stk: Self::MOD_N_STK.from_data(data)?,
                mod_st: Self::MOD_ST.from_data(data)?,
                mod_soc: Self::MOD_SOC.from_data(data)?,
                mod_ocv: Self::MOD_OCV.from_data(data)?,
                mod_v: Self::MOD_V.from_data(data)?,
                mod_cell_v_max: Self::MOD_CELL_V_MAX.from_data(data)?,
                mod_cell_v_max_cell: Self::MOD_CELL_V_MAX_CELL.from_data(data)?,
                mod_cell_v_min: Self::MOD_CELL_V_MIN.from_data(data)?,
                mod_cell_v_min_cell: Self::MOD_CELL_V_MIN_CELL.from_data(data)?,
                mod_cell_v_avg: Self::MOD_CELL_V_AVG.from_data(data)?,
                mod_ano_tmp: Self::MOD_ANO_TMP.from_data(data)?,
                mod_cat_tmp: Self::MOD_CAT_TMP.from_data(data)?,
                mod_con_st: Self::MOD_CON_ST.from_data(data)?,
                mod_evt1: Self::MOD_EVT1.from_data(data)?,
                mod_evt2: Self::MOD_EVT2.from_data(data)?,
                mod_con_fail: Self::MOD_CON_FAIL.from_data(data)?,
                mod_set_ena: Self::MOD_SET_ENA.from_data(data)?,
                mod_set_con: Self::MOD_SET_CON.from_data(data)?,
                mod_dis_rsn: Self::MOD_DIS_RSN.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Module as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Module::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Module Status"] #[doc = " "] #[doc = " Current status of the module."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ModuleModSt : u32 {
    #[allow(missing_docs)] const ModuleEnabled = 1; #[allow(missing_docs)] const
    ContactorStatus = 2; }
}
impl crate::Value for ModuleModSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModuleModSt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Contactor Status"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct ModuleModConSt : u32 { #[allow(missing_docs)] const Contactor0 = 1;
    #[allow(missing_docs)] const Contactor1 = 2; #[allow(missing_docs)] const Contactor2
    = 4; #[allow(missing_docs)] const Contactor3 = 8; #[allow(missing_docs)] const
    Contactor4 = 16; #[allow(missing_docs)] const Contactor5 = 32; #[allow(missing_docs)]
    const Contactor6 = 64; #[allow(missing_docs)] const Contactor7 = 128;
    #[allow(missing_docs)] const Contactor8 = 256; #[allow(missing_docs)] const
    Contactor9 = 512; #[allow(missing_docs)] const Contactor10 = 1024;
    #[allow(missing_docs)] const Contactor11 = 2048; #[allow(missing_docs)] const
    Contactor12 = 4096; #[allow(missing_docs)] const Contactor13 = 8192;
    #[allow(missing_docs)] const Contactor14 = 16384; #[allow(missing_docs)] const
    Contactor15 = 32768; #[allow(missing_docs)] const Contactor16 = 65536;
    #[allow(missing_docs)] const Contactor17 = 131072; #[allow(missing_docs)] const
    Contactor18 = 262144; #[allow(missing_docs)] const Contactor19 = 524288;
    #[allow(missing_docs)] const Contactor20 = 1048576; #[allow(missing_docs)] const
    Contactor21 = 2097152; #[allow(missing_docs)] const Contactor22 = 4194304;
    #[allow(missing_docs)] const Contactor23 = 8388608; #[allow(missing_docs)] const
    Contactor24 = 16777216; #[allow(missing_docs)] const Contactor25 = 33554432;
    #[allow(missing_docs)] const Contactor26 = 67108864; #[allow(missing_docs)] const
    Contactor27 = 134217728; #[allow(missing_docs)] const Contactor28 = 268435456;
    #[allow(missing_docs)] const Contactor29 = 536870912; #[allow(missing_docs)] const
    Contactor30 = 1073741824; }
}
impl crate::Value for ModuleModConSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModuleModConSt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Module Event 1"] #[doc = " "] #[doc =
    " Alarms, warnings and status values.  Bit flags."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct ModuleModEvt1 : u32 { #[allow(missing_docs)] const
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
    VoltageImbalanceWarning = 131072; #[allow(missing_docs)] const Reserved1 = 262144;
    #[allow(missing_docs)] const Reserved2 = 524288; #[allow(missing_docs)] const
    ContactorError = 1048576; #[allow(missing_docs)] const FanError = 2097152;
    #[allow(missing_docs)] const GroundFault = 4194304; #[allow(missing_docs)] const
    OpenDoorError = 8388608; #[allow(missing_docs)] const Reserved3 = 16777216;
    #[allow(missing_docs)] const Reserved4 = 33554432; #[allow(missing_docs)] const
    Reserved5 = 67108864; #[allow(missing_docs)] const FireAlarm = 134217728;
    #[allow(missing_docs)] const ModuleConfigurationAlarm = 268435456;
    #[allow(missing_docs)] const ModuleConfigurationWarning = 536870912; }
}
impl crate::Value for ModuleModEvt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModuleModEvt1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Module Event 2"] #[doc = " "] #[doc =
    " Alarms, warnings and status values.  Bit flags."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct ModuleModEvt2 : u32 { #[allow(missing_docs)] const
    LeakAlarm = 1; #[allow(missing_docs)] const PumpAlarm = 2; #[allow(missing_docs)]
    const HighPressureAlarm = 4; #[allow(missing_docs)] const HighPressureWarning = 8;
    #[allow(missing_docs)] const LowFlowAlarm = 16; #[allow(missing_docs)] const
    LowFlowWarning = 32; }
}
impl crate::Value for ModuleModEvt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModuleModEvt2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Connection Failure Reason
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ModuleModConFail {
    #[allow(missing_docs)]
    NoFailure,
    #[allow(missing_docs)]
    ButtonPushed,
    #[allow(missing_docs)]
    ModuleGroundFault,
    #[allow(missing_docs)]
    OutsideVoltageRange,
    #[allow(missing_docs)]
    ModuleNotEnabled,
    #[allow(missing_docs)]
    FuseOpen,
    #[allow(missing_docs)]
    ContactorFailure,
    #[allow(missing_docs)]
    PrechargeFailure,
    #[allow(missing_docs)]
    ModuleFault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ModuleModConFail {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::NoFailure,
            1 => Self::ButtonPushed,
            2 => Self::ModuleGroundFault,
            3 => Self::OutsideVoltageRange,
            4 => Self::ModuleNotEnabled,
            5 => Self::FuseOpen,
            6 => Self::ContactorFailure,
            7 => Self::PrechargeFailure,
            8 => Self::ModuleFault,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::NoFailure => 0,
            Self::ButtonPushed => 1,
            Self::ModuleGroundFault => 2,
            Self::OutsideVoltageRange => 3,
            Self::ModuleNotEnabled => 4,
            Self::FuseOpen => 5,
            Self::ContactorFailure => 6,
            Self::PrechargeFailure => 7,
            Self::ModuleFault => 8,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ModuleModConFail {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Enable/Disable Module
///
/// Enables and disables the module.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ModuleModSetEna {
    #[allow(missing_docs)]
    EnableModule,
    #[allow(missing_docs)]
    DisableModule,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ModuleModSetEna {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::EnableModule,
            2 => Self::DisableModule,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::EnableModule => 1,
            Self::DisableModule => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ModuleModSetEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Connect/Disconnect Module
///
/// Connects and disconnects the module.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ModuleModSetCon {
    #[allow(missing_docs)]
    ConnectModule,
    #[allow(missing_docs)]
    DisconnectModule,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ModuleModSetCon {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::ConnectModule,
            2 => Self::DisconnectModule,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::ConnectModule => 1,
            Self::DisconnectModule => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ModuleModSetCon {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Disabled Reason
///
/// Reason why the module is currently disabled.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ModuleModDisRsn {
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
impl crate::EnumValue for ModuleModDisRsn {
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
impl crate::FixedSize for ModuleModDisRsn {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for FlowBatteryString {
    const ID: u16 = 807;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m807
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
