//! DC Simulator Control Interface
/// Type alias for [`DcSimInterface`].
pub type Model64410 = DcSimInterface;
struct Counts {
    n_pt: u16,
    n_prof: u16,
}
/// DC Simulator Control Interface
///
/// A generic DC simulator/power supply control interface for DER electrical testing.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DcSimInterface {
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    pub v_max_lim: Option<u16>,
    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    pub p_max_lim: Option<u16>,
    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    pub i_max_lim: Option<u16>,
    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    pub mode: Option<Mode>,
    /// Power On/Off
    ///
    /// Power On/Off
    pub ena: Option<Ena>,
    /// Reset Device
    ///
    /// Reset Device
    pub reset: Option<Reset>,
    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    pub v_set: Option<u16>,
    /// Power Setpoint
    ///
    /// Power Setpoint
    pub p_set: Option<u16>,
    /// Current Setpoint
    ///
    /// Current Setpoint
    pub i_set: Option<u16>,
    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    pub en50530: Option<En50530>,
    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    pub vmpp: Option<u16>,
    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    pub pmpp: Option<u16>,
    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    pub g_set: Option<u16>,
    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    pub v_slew_rate: Option<u16>,
    /// Power Slew Rate
    ///
    /// Power Slew Rate
    pub p_slew_rate: Option<u16>,
    /// Current Slew Rate
    ///
    /// Current Slew Rate
    pub i_slew_rate: Option<u16>,
    /// Enable Profile
    ///
    /// Start/Stop the Profile
    pub ena_prof: Option<EnaProf>,
    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    pub adpt_prof_req: Option<u16>,
    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    pub adpt_prof_rslt: AdptProfRslt,
    /// Measured Voltage
    ///
    /// Measured Voltage
    pub v: Option<i32>,
    /// Measured Power
    ///
    /// Measured Power
    pub p: Option<i32>,
    /// Measured Current
    ///
    /// Measured Current
    pub i: Option<i32>,
    /// Errors
    ///
    /// Error States
    pub errors: Option<String>,
    /// Number Of Points
    ///
    /// Number of profile points supported.
    pub n_pt: u16,
    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    pub n_prof: u16,
    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    pub w_sf: i16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    pub v_sf: i16,
    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    pub a_sf: i16,
    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    pub g_sf: i16,
    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    pub tms_sf: i16,
    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    pub v_slew_sf: i16,
    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    pub p_slew_sf: i16,
    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    pub i_slew_sf: i16,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    pub pct_sf: i16,
    /// Stored Profiles
    ///
    /// Stored profile sets.
    ///
    /// Comments: Stored Pofile Sets - Number of profile sets = NProf - The first set is read-only and indicates the current settings.
    pub prof: Vec<Prof>,
}
#[allow(missing_docs)]
impl DcSimInterface {
    pub const V_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const P_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const I_MAX_LIM: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const MODE: crate::Point<Self, Option<Mode>> = crate::Point::new(3, 1, true);
    pub const ENA: crate::Point<Self, Option<Ena>> = crate::Point::new(4, 1, true);
    pub const RESET: crate::Point<Self, Option<Reset>> = crate::Point::new(5, 1, true);
    pub const V_SET: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const P_SET: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const I_SET: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const EN50530: crate::Point<Self, Option<En50530>> = crate::Point::new(9, 1, true);
    pub const VMPP: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const PMPP: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const G_SET: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const V_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const P_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const I_SLEW_RATE: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, true);
    pub const ENA_PROF: crate::Point<Self, Option<EnaProf>> = crate::Point::new(16, 1, true);
    pub const ADPT_PROF_REQ: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const ADPT_PROF_RSLT: crate::Point<Self, AdptProfRslt> = crate::Point::new(18, 1, false);
    pub const V: crate::Point<Self, Option<i32>> = crate::Point::new(19, 2, false);
    pub const P: crate::Point<Self, Option<i32>> = crate::Point::new(21, 2, false);
    pub const I: crate::Point<Self, Option<i32>> = crate::Point::new(23, 2, false);
    pub const ERRORS: crate::Point<Self, Option<String>> = crate::Point::new(25, 32, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(57, 1, false);
    pub const N_PROF: crate::Point<Self, u16> = crate::Point::new(58, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(59, 1, true);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(60, 1, true);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(61, 1, true);
    pub const G_SF: crate::Point<Self, i16> = crate::Point::new(62, 1, true);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(63, 1, true);
    pub const V_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(64, 1, true);
    pub const P_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(65, 1, true);
    pub const I_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(66, 1, true);
    pub const PCT_SF: crate::Point<Self, i16> = crate::Point::new(67, 1, true);
}
static DC_SIM_INTERFACE_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "v_max_lim",
        label: "Maximum Voltage",
        description: "Upper Voltage Protection Limit",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p_max_lim",
        label: "Maximum Power",
        description: "Upper Power Protection Limit",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i_max_lim",
        label: "Maximum Current",
        description: "Upper Current Protection Limit",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mode",
        label: "CV or CC Mode",
        description: "Constant Voltage (CV) or Constant Current (CC) Mode",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ena",
        label: "Power On/Off",
        description: "Power On/Off",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "reset",
        label: "Reset Device",
        description: "Reset Device",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_set",
        label: "Voltage Setpoint",
        description: "Voltage Setpoint",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p_set",
        label: "Power Setpoint",
        description: "Power Setpoint",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i_set",
        label: "Current Setpoint",
        description: "Current Setpoint",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "en50530",
        label: "EN50530 Mode",
        description: "EN50530 Mode - Enable or disable EN50530 profile mode",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vmpp",
        label: "EN50530 MPP Voltage",
        description: "EN50530 MPP Voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pmpp",
        label: "EN50530 MPP Power",
        description: "EN50530 MPP Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "g_set",
        label: "Irradiance Setpoint",
        description: "Irradiance Setpoint",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_slew_rate",
        label: "Voltage Slew Rate",
        description: "Voltage Slew Rate",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p_slew_rate",
        label: "Power Slew Rate",
        description: "Power Slew Rate",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i_slew_rate",
        label: "Current Slew Rate",
        description: "Current Slew Rate",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ena_prof",
        label: "Enable Profile",
        description: "Start/Stop the Profile",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "adpt_prof_req",
        label: "Profile Adoption Request",
        description: "Index of profile points to adopt. First curve index is 1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "adpt_prof_rslt",
        label: "Adopt Profile Result",
        description: "Result of last adopt profile operation.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "Measured Voltage",
        description: "Measured Voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p",
        label: "Measured Power",
        description: "Measured Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i",
        label: "Measured Current",
        description: "Measured Current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "errors",
        label: "Errors",
        description: "Error States",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_pt",
        label: "Number Of Points",
        description: "Number of profile points supported.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_prof",
        label: "Stored Profile Count",
        description: "Number of stored profiles supported.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_sf",
        label: "Power Scale Factor",
        description: "Scale factor for power points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "Voltage Scale Factor",
        description: "Scale factor for voltage points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "Current Scale Factor",
        description: "Scale factor for current points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "g_sf",
        label: "Irradiance Scale Factor",
        description: "Scale factor for irradiance.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tms_sf",
        label: "Time Scale Factor",
        description: "Scale factor for time points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_slew_sf",
        label: "Voltage Slew Rate Scale Factor",
        description: "Scale factor for voltage slew rate.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p_slew_sf",
        label: "Power Slew Rate Scale Factor",
        description: "Scale factor for power slew rate.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i_slew_sf",
        label: "Current Slew Rate Scale Factor",
        description: "Scale factor for current slew rate.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pct_sf",
        label: "Percent Scale Factor",
        description: "Scale factor for percentages.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "prof",
        label: "Stored Profiles",
        description: "Stored profile sets.",
        kind: crate::FieldKind::RepeatingGroup(<Prof as crate::GroupMeta>::group_info),
    },
];
static DC_SIM_INTERFACE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DCSimInterface",
    label: "DC Simulator Control Interface",
    description:
        "A generic DC simulator/power supply control interface for DER electrical testing.",
    fields: DC_SIM_INTERFACE_FIELDS,
};
impl crate::GroupMeta for DcSimInterface {
    fn group_info() -> &'static crate::GroupInfo {
        &DC_SIM_INTERFACE_GROUP_INFO
    }
}
impl crate::Group for DcSimInterface {
    const LEN: u16 = 68;
}
impl DcSimInterface {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            n_pt: Self::N_PT.from_data(data)?,
            n_prof: Self::N_PROF.from_data(data)?,
        };
        let (nested_data, prof) = Prof::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                v_max_lim: Self::V_MAX_LIM.from_data(data)?,
                p_max_lim: Self::P_MAX_LIM.from_data(data)?,
                i_max_lim: Self::I_MAX_LIM.from_data(data)?,
                mode: Self::MODE.from_data(data)?,
                ena: Self::ENA.from_data(data)?,
                reset: Self::RESET.from_data(data)?,
                v_set: Self::V_SET.from_data(data)?,
                p_set: Self::P_SET.from_data(data)?,
                i_set: Self::I_SET.from_data(data)?,
                en50530: Self::EN50530.from_data(data)?,
                vmpp: Self::VMPP.from_data(data)?,
                pmpp: Self::PMPP.from_data(data)?,
                g_set: Self::G_SET.from_data(data)?,
                v_slew_rate: Self::V_SLEW_RATE.from_data(data)?,
                p_slew_rate: Self::P_SLEW_RATE.from_data(data)?,
                i_slew_rate: Self::I_SLEW_RATE.from_data(data)?,
                ena_prof: Self::ENA_PROF.from_data(data)?,
                adpt_prof_req: Self::ADPT_PROF_REQ.from_data(data)?,
                adpt_prof_rslt: Self::ADPT_PROF_RSLT.from_data(data)?,
                v: Self::V.from_data(data)?,
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                errors: Self::ERRORS.from_data(data)?,
                n_pt: Self::N_PT.from_data(data)?,
                n_prof: Self::N_PROF.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                g_sf: Self::G_SF.from_data(data)?,
                tms_sf: Self::TMS_SF.from_data(data)?,
                v_slew_sf: Self::V_SLEW_SF.from_data(data)?,
                p_slew_sf: Self::P_SLEW_SF.from_data(data)?,
                i_slew_sf: Self::I_SLEW_SF.from_data(data)?,
                pct_sf: Self::PCT_SF.from_data(data)?,
                prof,
            },
        ))
    }
}
/// CV or CC Mode
///
/// Constant Voltage (CV) or Constant Current (CC) Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Mode {
    /// CV Mode
    ///
    /// Constant Voltage (CV) Mode
    Cv,
    /// CC Mode
    ///
    /// Constant Current (CC) Mode.
    Cc,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Mode {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Cv,
            1 => Self::Cc,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Cv => 0,
            Self::Cc => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Mode {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Power On/Off
///
/// Power On/Off
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Ena {
    /// Power On
    ///
    /// Power On
    On,
    /// Power Off
    ///
    /// Power Off
    Off,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Ena {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::On,
            0 => Self::Off,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::On => 1,
            Self::Off => 0,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Ena {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Reset Device
///
/// Reset Device
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Reset {
    /// Reset Device
    ///
    /// Reset Device
    Reset,
    /// Do Not Reset Device
    ///
    /// Do Not Reset Device
    DoNotReset,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Reset {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Reset,
            0 => Self::DoNotReset,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Reset => 1,
            Self::DoNotReset => 0,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Reset {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// EN50530 Mode
///
/// EN50530 Mode - Enable or disable EN50530 profile mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum En50530 {
    /// EN50530 Mode
    ///
    /// EN50530 Mode
    En50530,
    /// Do Not Use EN50530 Mode
    ///
    /// Do Not Use EN50530 Mode
    DoNotEn50530,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for En50530 {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::En50530,
            0 => Self::DoNotEn50530,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::En50530 => 1,
            Self::DoNotEn50530 => 0,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for En50530 {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Enable Profile
///
/// Start/Stop the Profile
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum EnaProf {
    /// Start Profile
    ///
    /// Start the Profile
    Start,
    /// Stop Profile
    ///
    /// Stop the Profile
    Stop,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for EnaProf {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Start,
            0 => Self::Stop,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Start => 1,
            Self::Stop => 0,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for EnaProf {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Adopt Profile Result
///
/// Result of last adopt profile operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AdptProfRslt {
    /// Update In Progress
    ///
    /// Profile update in progress.
    InProgress,
    /// Update Complete
    ///
    /// Profile update completed successfully.
    Completed,
    /// Update Failed
    ///
    /// Profile update failed.
    Failed,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for AdptProfRslt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::InProgress,
            1 => Self::Completed,
            2 => Self::Failed,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::InProgress => 0,
            Self::Completed => 1,
            Self::Failed => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for AdptProfRslt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Stored Profiles
///
/// Stored profile sets.
///
/// Comments: Stored Pofile Sets - Number of profile sets = NProf - The first set is read-only and indicates the current settings.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Prof {
    /// Active Points
    ///
    /// Number of active points.
    pub act_pt: u16,
    /// Dependent References
    ///
    /// Profile references.
    pub dept_ref: ProfDeptRef,
    /// Stored Profile Points
    ///
    /// Stored profile points.
    ///
    /// Comments: Stored Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl Prof {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, ProfDeptRef> = crate::Point::new(1, 1, true);
}
static PROF_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "act_pt",
        label: "Active Points",
        description: "Number of active points.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dept_ref",
        label: "Dependent References",
        description: "Profile references.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pt",
        label: "Stored Profile Points",
        description: "Stored profile points.",
        kind: crate::FieldKind::RepeatingGroup(<Pt as crate::GroupMeta>::group_info),
    },
];
static PROF_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "Prof",
    label: "Stored Profiles",
    description: "Stored profile sets.",
    fields: PROF_FIELDS,
};
impl crate::GroupMeta for Prof {
    fn group_info() -> &'static crate::GroupInfo {
        &PROF_GROUP_INFO
    }
}
impl crate::Group for Prof {
    const LEN: u16 = 2;
}
impl Prof {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                dept_ref: Self::DEPT_REF.from_data(data)?,
                pt,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_prof).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Prof::parse_group(data, counts)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Dependent References"] #[doc = " "] #[doc = " Profile references."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ProfDeptRef : u32 {
    #[doc = " Voltage"] const Voltage = 1; #[doc = " Power"] const Power = 2; #[doc =
    " Current"] const Current = 4; #[doc = " Irradiance"] const Irradiance = 8; }
}
impl crate::Value for ProfDeptRef {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ProfDeptRef {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Stored Profile Points
///
/// Stored profile points.
///
/// Comments: Stored Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Profile Time
    ///
    /// Profile time.
    pub tms: Option<u16>,
    /// Voltage Point
    ///
    /// Profile voltage point in Volts.
    pub v: Option<u16>,
    /// Power Point
    ///
    /// Profile power point in Watts.
    pub p: Option<u16>,
    /// Current Point
    ///
    /// Profile current point in Amps.
    pub i: Option<u16>,
    /// Irradiance Point
    ///
    /// Profile irradiance point as percentage.
    pub g: Option<u16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const TMS: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const V: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const P: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const I: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const G: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
}
static PT_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "tms",
        label: "Profile Time",
        description: "Profile time.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "Voltage Point",
        description: "Profile voltage point in Volts.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "p",
        label: "Power Point",
        description: "Profile power point in Watts.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i",
        label: "Current Point",
        description: "Profile current point in Amps.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "g",
        label: "Irradiance Point",
        description: "Profile irradiance point as percentage.",
        kind: crate::FieldKind::Point,
    },
];
static PT_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "Pt",
    label: "Stored Profile Points",
    description: "Stored profile points.",
    fields: PT_FIELDS,
};
impl crate::GroupMeta for Pt {
    fn group_info() -> &'static crate::GroupInfo {
        &PT_GROUP_INFO
    }
}
impl crate::Group for Pt {
    const LEN: u16 = 5;
}
impl Pt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                tms: Self::TMS.from_data(data)?,
                v: Self::V.from_data(data)?,
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                g: Self::G.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_pt).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Pt::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for DcSimInterface {
    const ID: u16 = 64410;
    const NAME: &'static str = "DCSimInterface";
    const LABEL: &'static str = "DC Simulator Control Interface";
    const DESCRIPTION: &'static str =
        "A generic DC simulator/power supply control interface for DER electrical testing.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64410
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
