//! Lithium-Ion String Model

/// Lithium-Ion String Model
#[derive(Debug)]
pub struct Model804 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
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
    /// Notes: Measurement.
    pub so_c: u16,
    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub do_d: Option<u16>,
    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    pub n_cyc: Option<u32>,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub so_h: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    ///
    /// Notes: Measurement.
    pub a: i16,
    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Notes: Measurement.
    pub v: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cell_v_max: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    pub cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cell_v_min: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    pub cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub cell_v_avg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub mod_tmp_max: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    pub mod_tmp_max_mod: u16,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub mod_tmp_min: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    pub mod_tmp_min_mod: u16,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
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
    /// Notes: Reserved for future use.
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
    /// Notes: Should reset to 0 upon completion.
    pub set_con: Option<SetCon>,
    /// Scale factor for string state of charge.
    pub so_c_sf: i16,
    /// Scale factor for string state of health.
    pub so_h_sf: Option<i16>,
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
}

#[allow(missing_docs)]

impl Model804 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, St> = crate::PointDef::new(2, 2, false);
    pub const CON_FAIL: crate::PointDef<Self, Option<ConFail>> = crate::PointDef::new(4, 1, false);
    pub const N_CELL_BAL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const SO_C: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const DO_D: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
    pub const N_CYC: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(8, 2, false);
    pub const SO_H: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const V: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const CELL_V_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELL_V_MAX_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(14, 1, false);
    pub const CELL_V_MIN: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const CELL_V_MIN_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(16, 1, false);
    pub const CELL_V_AVG: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const MOD_TMP_MAX: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const MOD_TMP_MAX_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const MOD_TMP_MIN: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MOD_TMP_MIN_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const MOD_TMP_AVG: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const CON_ST: crate::PointDef<Self, Option<ConSt>> = crate::PointDef::new(24, 2, false);
    pub const EVT1: crate::PointDef<Self, Evt1> = crate::PointDef::new(26, 2, false);
    pub const EVT2: crate::PointDef<Self, Option<Evt2>> = crate::PointDef::new(28, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, Option<EvtVnd1>> = crate::PointDef::new(30, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, Option<EvtVnd2>> = crate::PointDef::new(32, 2, false);
    pub const SET_ENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(34, 1, true);
    pub const SET_CON: crate::PointDef<Self, Option<SetCon>> = crate::PointDef::new(35, 1, true);
    pub const SO_C_SF: crate::PointDef<Self, i16> = crate::PointDef::new(36, 1, false);
    pub const SO_H_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(37, 1, false);
    pub const DO_D_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(38, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(39, 1, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(40, 1, false);
    pub const CELL_V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(41, 1, false);
    pub const MOD_TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
}

impl crate::Model for Model804 {
    const ID: u16 = 804;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            idx: Self::IDX.from_data(data)?,
            n_mod: Self::N_MOD.from_data(data)?,
            st: Self::ST.from_data(data)?,
            con_fail: Self::CON_FAIL.from_data(data)?,
            n_cell_bal: Self::N_CELL_BAL.from_data(data)?,
            so_c: Self::SO_C.from_data(data)?,
            do_d: Self::DO_D.from_data(data)?,
            n_cyc: Self::N_CYC.from_data(data)?,
            so_h: Self::SO_H.from_data(data)?,
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
            so_c_sf: Self::SO_C_SF.from_data(data)?,
            so_h_sf: Self::SO_H_SF.from_data(data)?,
            do_d_sf: Self::DO_D_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            cell_v_sf: Self::CELL_V_SF.from_data(data)?,
            mod_tmp_sf: Self::MOD_TMP_SF.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "String Status\n\nCurrent status of the string."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct St : u32 { # [doc = ""] const StringEnabled = 1 ; # [doc = "Notes: If string has multiple contactors, indicates that all contactors are closed."] const ContactorStatus = 2 ; } }
impl crate::Value for St {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<St> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(St::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

#[doc = "Connection Failure Reason"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum ConFail {
    #[doc = ""]
    NoFailure = 0,
    #[doc = ""]
    ButtonPushed = 1,
    #[doc = ""]
    StrGroundFault = 2,
    #[doc = ""]
    OutsideVoltageRange = 3,
    #[doc = ""]
    StringNotEnabled = 4,
    #[doc = ""]
    FuseOpen = 5,
    #[doc = ""]
    ContactorFailure = 6,
    #[doc = ""]
    PrechargeFailure = 7,
    #[doc = "Notes: See Evt1 for more information."]
    StringFault = 8,
}
impl crate::Value for ConFail {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ConFail> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ConFail::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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

bitflags::bitflags! { # [doc = "Contactor Status\n\nStatus of the contactor(s) for the string."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ConSt : u32 { # [doc = ""] const Contactor0 = 1 ; # [doc = ""] const Contactor1 = 2 ; # [doc = ""] const Contactor2 = 4 ; # [doc = ""] const Contactor3 = 8 ; # [doc = ""] const Contactor4 = 16 ; # [doc = ""] const Contactor5 = 32 ; # [doc = ""] const Contactor6 = 64 ; # [doc = ""] const Contactor7 = 128 ; # [doc = ""] const Contactor8 = 256 ; # [doc = ""] const Contactor9 = 512 ; # [doc = ""] const Contactor10 = 1024 ; # [doc = ""] const Contactor11 = 2048 ; # [doc = ""] const Contactor12 = 4096 ; # [doc = ""] const Contactor13 = 8192 ; # [doc = ""] const Contactor14 = 16384 ; # [doc = ""] const Contactor15 = 32768 ; # [doc = ""] const Contactor16 = 65536 ; # [doc = ""] const Contactor17 = 131072 ; # [doc = ""] const Contactor18 = 262144 ; # [doc = ""] const Contactor19 = 524288 ; # [doc = ""] const Contactor20 = 1048576 ; # [doc = ""] const Contactor21 = 2097152 ; # [doc = ""] const Contactor22 = 4194304 ; # [doc = ""] const Contactor23 = 8388608 ; # [doc = ""] const Contactor24 = 16777216 ; # [doc = ""] const Contactor25 = 33554432 ; # [doc = ""] const Contactor26 = 67108864 ; # [doc = ""] const Contactor27 = 134217728 ; # [doc = ""] const Contactor28 = 268435456 ; # [doc = ""] const Contactor29 = 536870912 ; # [doc = ""] const Contactor30 = 1073741824 ; } }
impl crate::Value for ConSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<ConSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(ConSt::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "String Event 1\n\nAlarms, warnings and status values.  Bit flags."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Evt1 : u32 { # [doc = ""] const CommunicationError = 1 ; # [doc = ""] const OverTempAlarm = 2 ; # [doc = ""] const OverTempWarning = 4 ; # [doc = ""] const UnderTempAlarm = 8 ; # [doc = ""] const UnderTempWarning = 16 ; # [doc = "Notes: See AChaMax in model S 802."] const OverChargeCurrentAlarm = 32 ; # [doc = "Notes: See AChaMax in model S 802."] const OverChargeCurrentWarning = 64 ; # [doc = "Notes: See ADisChaMax in model S 802."] const OverDischargeCurrentAlarm = 128 ; # [doc = "Notes: See ADisChaMax in model S 802."] const OverDischargeCurrentWarning = 256 ; # [doc = ""] const OverVoltAlarm = 512 ; # [doc = ""] const OverVoltWarning = 1024 ; # [doc = ""] const UnderVoltAlarm = 2048 ; # [doc = ""] const UnderVoltWarning = 4096 ; # [doc = ""] const UnderSocMinAlarm = 8192 ; # [doc = ""] const UnderSocMinWarning = 16384 ; # [doc = ""] const OverSocMaxAlarm = 32768 ; # [doc = ""] const OverSocMaxWarning = 65536 ; # [doc = ""] const VoltageImbalanceWarning = 131072 ; # [doc = ""] const TemperatureImbalanceAlarm = 262144 ; # [doc = ""] const TemperatureImbalanceWarning = 524288 ; # [doc = ""] const ContactorError = 1048576 ; # [doc = ""] const FanError = 2097152 ; # [doc = ""] const GroundFault = 4194304 ; # [doc = ""] const OpenDoorError = 8388608 ; # [doc = "Notes: Do not implement."] const Reserved1 = 16777216 ; # [doc = "Notes: See EvtVnd1 and EvtVnd2 for more information."] const OtherAlarm = 33554432 ; # [doc = "Notes: See EvtVnd1 and EvtVnd2 for more information."] const OtherWarning = 67108864 ; # [doc = "Notes: Do not implement."] const Reserved2 = 134217728 ; # [doc = ""] const ConfigurationAlarm = 268435456 ; # [doc = ""] const ConfigurationWarning = 536870912 ; } }
impl crate::Value for Evt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Evt1> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt1::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "String Event 2\n\nAlarms, warnings and status values.  Bit flags.\n\nNotes: Reserved for future use."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Evt2 : u32 { } }
impl crate::Value for Evt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Evt2> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt2::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 1\n\nVendor defined events."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct EvtVnd1 : u32 { } }
impl crate::Value for EvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd1> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd1::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "Vendor Event Bitfield 2\n\nVendor defined events."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct EvtVnd2 : u32 { } }
impl crate::Value for EvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<EvtVnd2> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(EvtVnd2::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}

#[doc = "Connect/Disconnect String\n\nConnects and disconnects the string.\n\nNotes: Should reset to 0 upon completion."]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum SetCon {
    #[doc = ""]
    ConnectString = 1,
    #[doc = ""]
    DisconnectString = 2,
}
impl crate::Value for SetCon {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SetCon> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SetCon::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
