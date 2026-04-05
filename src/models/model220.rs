//! Secure AC Meter Selected Readings
/// Type alias for [`AcMeterSecure`].
pub type Model220 = AcMeterSecure;
/// Secure AC Meter Selected Readings
///
/// Include this model for secure metering
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AcMeterSecure {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage
    ///
    /// Average phase or line voltage
    pub ph_v: Option<i16>,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub tot_wh_exp: u32,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub tot_wh_imp: u32,
    /// Real Energy scale factor
    pub tot_wh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub tot_v_ah_exp: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_v_ah_imp: Option<u32>,
    /// Apparent Energy scale factor
    pub tot_v_ah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_v_arh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_v_arh_imp_q2: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_v_arh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_v_arh_exp_q4: Option<u32>,
    /// Reactive Energy scale factor
    pub tot_v_arh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: Evt,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Detail: Shall be advanced for each request
    pub seq: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Detail: For future proof
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Detail: The value of N must be at least 4 (64 bits)
    pub n: u16,
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl AcMeterSecure {
    pub const A: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const PH_V: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(3, 1, false);
    pub const HZ: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, false);
    pub const W: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const VA: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const VA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, false);
    pub const VAR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, false);
    pub const PF: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, false);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, false);
    pub const TOT_WH_EXP: crate::Point<Self, u32> = crate::Point::new(14, 2, false);
    pub const TOT_WH_IMP: crate::Point<Self, u32> = crate::Point::new(16, 2, false);
    pub const TOT_WH_SF: crate::Point<Self, i16> = crate::Point::new(18, 1, false);
    pub const TOT_V_AH_EXP: crate::Point<Self, Option<u32>> = crate::Point::new(19, 2, false);
    pub const TOT_V_AH_IMP: crate::Point<Self, Option<u32>> = crate::Point::new(21, 2, false);
    pub const TOT_V_AH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const TOT_V_ARH_IMP_Q1: crate::Point<Self, Option<u32>> = crate::Point::new(24, 2, false);
    pub const TOT_V_ARH_IMP_Q2: crate::Point<Self, Option<u32>> = crate::Point::new(26, 2, false);
    pub const TOT_V_ARH_EXP_Q3: crate::Point<Self, Option<u32>> = crate::Point::new(28, 2, false);
    pub const TOT_V_ARH_EXP_Q4: crate::Point<Self, Option<u32>> = crate::Point::new(30, 2, false);
    pub const TOT_V_ARH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(32, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(33, 2, false);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(36, 2, false);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(38, 1, false);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(39, 1, false);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(40, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(41, 1, false);
}
static AC_METER_SECURE_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "a",
        label: "Amps",
        description: "Total AC Current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "A_SF",
        description: "Current scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ph_v",
        label: "Voltage",
        description: "Average phase or line voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "V_SF",
        description: "Voltage scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz",
        label: "Hz",
        description: "Frequency",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_sf",
        label: "Hz_SF",
        description: "Frequency scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w",
        label: "Watts",
        description: "Total Real Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_sf",
        label: "W_SF",
        description: "Real Power scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "va",
        label: "VA",
        description: "AC Apparent Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "va_sf",
        label: "VA_SF",
        description: "Apparent Power scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var",
        label: "VAR",
        description: "Reactive Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var_sf",
        label: "VAR_SF",
        description: "Reactive Power scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pf",
        label: "PF",
        description: "Power Factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pf_sf",
        label: "PF_SF",
        description: "Power Factor scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_exp",
        label: "Total Watt-hours Exported",
        description: "Total Real Energy Exported",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_imp",
        label: "Total Watt-hours Imported",
        description: "Total Real Energy Imported",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_sf",
        label: "TotWh_SF",
        description: "Real Energy scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_ah_exp",
        label: "Total VA-hours Exported",
        description: "Total Apparent Energy Exported",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_ah_imp",
        label: "Total VA-hours Imported",
        description: "Total Apparent Energy Imported",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_ah_sf",
        label: "TotVAh_SF",
        description: "Apparent Energy scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_arh_imp_q1",
        label: "Total VAR-hours Imported Q1",
        description: "Total Reactive Energy Imported Quadrant 1",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_arh_imp_q2",
        label: "Total VAr-hours Imported Q2",
        description: "Total Reactive Power Imported Quadrant 2",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_arh_exp_q3",
        label: "Total VAr-hours Exported Q3",
        description: "Total Reactive Power Exported Quadrant 3",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_arh_exp_q4",
        label: "Total VAr-hours Exported Q4",
        description: "Total Reactive Power Exported Quadrant 4",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_v_arh_sf",
        label: "TotVArh_SF",
        description: "Reactive Energy scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt",
        label: "Events",
        description: "Meter Event Flags",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ts",
        label: "Timestamp",
        description: "Timestamp value is the number of seconds since January 1, 2000",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ms",
        label: "Milliseconds",
        description: "Millisecond counter 0-999",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "seq",
        label: "Sequence",
        description: "Sequence number of request",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alg",
        label: "Algorithm",
        description: "Algorithm used to compute the digital signature",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n",
        label: "N",
        description: "Number of registers comprising the digital signature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "repeating",
        label: "repeating",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Repeating as crate::GroupMeta>::group_info),
    },
];
static AC_METER_SECURE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "ac_meter_secure",
    label: "Secure AC Meter Selected Readings",
    description: "Include this model for secure metering",
    fields: AC_METER_SECURE_FIELDS,
};
impl crate::GroupMeta for AcMeterSecure {
    fn group_info() -> &'static crate::GroupInfo {
        &AC_METER_SECURE_GROUP_INFO
    }
}
impl crate::Group for AcMeterSecure {
    const LEN: u16 = 42;
}
impl AcMeterSecure {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                a: Self::A.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                ph_v: Self::PH_V.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
                w: Self::W.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                va: Self::VA.from_data(data)?,
                va_sf: Self::VA_SF.from_data(data)?,
                var: Self::VAR.from_data(data)?,
                var_sf: Self::VAR_SF.from_data(data)?,
                pf: Self::PF.from_data(data)?,
                pf_sf: Self::PF_SF.from_data(data)?,
                tot_wh_exp: Self::TOT_WH_EXP.from_data(data)?,
                tot_wh_imp: Self::TOT_WH_IMP.from_data(data)?,
                tot_wh_sf: Self::TOT_WH_SF.from_data(data)?,
                tot_v_ah_exp: Self::TOT_V_AH_EXP.from_data(data)?,
                tot_v_ah_imp: Self::TOT_V_AH_IMP.from_data(data)?,
                tot_v_ah_sf: Self::TOT_V_AH_SF.from_data(data)?,
                tot_v_arh_imp_q1: Self::TOT_V_ARH_IMP_Q1.from_data(data)?,
                tot_v_arh_imp_q2: Self::TOT_V_ARH_IMP_Q2.from_data(data)?,
                tot_v_arh_exp_q3: Self::TOT_V_ARH_EXP_Q3.from_data(data)?,
                tot_v_arh_exp_q4: Self::TOT_V_ARH_EXP_Q4.from_data(data)?,
                tot_v_arh_sf: Self::TOT_V_ARH_SF.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                ts: Self::TS.from_data(data)?,
                ms: Self::MS.from_data(data)?,
                seq: Self::SEQ.from_data(data)?,
                alg: Self::ALG.from_data(data)?,
                n: Self::N.from_data(data)?,
                repeating,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Events"] #[doc = " "] #[doc = " Meter Event Flags"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Evt : u32 { #[allow(missing_docs)] const
    PowerFailure = 4; #[allow(missing_docs)] const UnderVoltage = 8;
    #[allow(missing_docs)] const LowPf = 16; #[allow(missing_docs)] const OverCurrent =
    32; #[allow(missing_docs)] const OverVoltage = 64; #[allow(missing_docs)] const
    MissingSensor = 128; #[allow(missing_docs)] const Oem01 = 65536;
    #[allow(missing_docs)] const Oem02 = 131072; #[allow(missing_docs)] const Oem03 =
    262144; #[allow(missing_docs)] const Oem04 = 524288; #[allow(missing_docs)] const
    Oem05 = 1048576; #[allow(missing_docs)] const Oem06 = 2097152; #[allow(missing_docs)]
    const Oem07 = 4194304; #[allow(missing_docs)] const Oem08 = 8388608;
    #[allow(missing_docs)] const Oem09 = 16777216; #[allow(missing_docs)] const Oem10 =
    33554432; #[allow(missing_docs)] const Oem11 = 67108864; #[allow(missing_docs)] const
    Oem12 = 134217728; #[allow(missing_docs)] const Oem13 = 268435456;
    #[allow(missing_docs)] const Oem14 = 536870912; #[allow(missing_docs)] const Oem15 =
    1073741824; }
}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Detail: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Alg {
    /// Detail: For test purposes only
    None,
    #[allow(missing_docs)]
    AesGmac64,
    #[allow(missing_docs)]
    Ecc256,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Alg {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::AesGmac64,
            2 => Self::Ecc256,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::AesGmac64 => 1,
            Self::Ecc256 => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Alg {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    #[allow(missing_docs)]
    pub ds: u16,
}
#[allow(missing_docs)]
impl Repeating {
    pub const DS: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
static REPEATING_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "ds",
    label: "DS",
    description: "",
    kind: crate::FieldKind::Point,
}];
static REPEATING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "repeating",
    label: "repeating",
    description: "",
    fields: REPEATING_FIELDS,
};
impl crate::GroupMeta for Repeating {
    fn group_info() -> &'static crate::GroupInfo {
        &REPEATING_GROUP_INFO
    }
}
impl crate::Group for Repeating {
    const LEN: u16 = 1;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ds: Self::DS.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for AcMeterSecure {
    const ID: u16 = 220;
    const NAME: &'static str = "ac_meter_secure";
    const LABEL: &'static str = "Secure AC Meter Selected Readings";
    const DESCRIPTION: &'static str = "Include this model for secure metering";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m220
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
