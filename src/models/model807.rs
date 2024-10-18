//! Flow Battery String Model
/// Flow Battery String Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model807 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
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
    /// Notes: Measurement.
    pub mod_v_max: u16,
    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    pub mod_v_max_mod: Option<u16>,
    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Notes: Measurement.
    pub mod_v_min: u16,
    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    pub mod_v_min_mod: Option<u16>,
    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub mod_v_avg: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
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
    /// Notes: Measurement.
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
    /// Notes: Calculation based on measurements.
    pub cell_v_avg: Option<u16>,
    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmp_max: i16,
    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    pub tmp_max_mod: Option<u16>,
    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmp_min: i16,
    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    pub tmp_min_mod: Option<u16>,
    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
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
    pub so_c_sf: i16,
    /// Scale factor for open circuit voltage.
    pub ocv_sf: i16,
}
#[allow(missing_docs)]
impl Model807 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const N_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const N_MOD_CON: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const MOD_V_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MOD_V_MAX_MOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const MOD_V_MIN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const MOD_V_MIN_MOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const MOD_V_AVG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const CELL_V_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, false);
    pub const CELL_V_MAX_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(9, 1, false);
    pub const CELL_V_MAX_STK: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(10, 1, false);
    pub const CELL_V_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, false);
    pub const CELL_V_MIN_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(12, 1, false);
    pub const CELL_V_MIN_STK: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(13, 1, false);
    pub const CELL_V_AVG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const TMP_MAX: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const TMP_MAX_MOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const TMP_MIN: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const TMP_MIN_MOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(18, 1, false);
    pub const TMP_AVG: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const EVT1: crate::PointDef<Self, Evt1> = crate::PointDef::new(20, 2, false);
    pub const EVT2: crate::PointDef<Self, Evt2> = crate::PointDef::new(22, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, EvtVnd1> = crate::PointDef::new(24, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, EvtVnd2> = crate::PointDef::new(26, 2, false);
    pub const MOD_V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const CELL_V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const SO_C_SF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const OCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
}
impl crate::Model for Model807 {
    const ID: u16 = 807;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
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
            so_c_sf: Self::SO_C_SF.from_data(data)?,
            ocv_sf: Self::OCV_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m807
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
    " Notes: See AChaMax in model S 802."] const OverChargeCurrentAlarm = 32; #[doc =
    " Notes: See AChaMax in model S 802."] const OverChargeCurrentWarning = 64; #[doc =
    " Notes: See ADisChaMax in model S 802."] const OverDischargeCurrentAlarm = 128;
    #[doc = " Notes: See ADisChaMax in model S 802."] const OverDischargeCurrentWarning =
    256; #[allow(missing_docs)] const OverVoltAlarm = 512; #[allow(missing_docs)] const
    OverVoltWarning = 1024; #[allow(missing_docs)] const UnderVoltAlarm = 2048;
    #[allow(missing_docs)] const UnderVoltWarning = 4096; #[allow(missing_docs)] const
    UnderSocMinAlarm = 8192; #[allow(missing_docs)] const UnderSocMinWarning = 16384;
    #[allow(missing_docs)] const OverSocMaxAlarm = 32768; #[allow(missing_docs)] const
    OverSocMaxWarning = 65536; #[allow(missing_docs)] const VoltageImbalanceWarning =
    131072; #[doc = " Notes: Do not implement."] const Reserved1 = 262144; #[doc =
    " Notes: Do not implement."] const Reserved2 = 524288; #[allow(missing_docs)] const
    ContactorError = 1048576; #[allow(missing_docs)] const FanError = 2097152;
    #[allow(missing_docs)] const GroundFault = 4194304; #[allow(missing_docs)] const
    OpenDoorError = 8388608; #[doc = " Notes: Do not implement."] const Reserved3 =
    16777216; #[doc = " Notes: See EvtVnd1 and EvtVnd2 for more information."] const
    OtherAlarm = 33554432; #[doc =
    " Notes: See EvtVnd1 and EvtVnd2 for more information."] const OtherWarning =
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
