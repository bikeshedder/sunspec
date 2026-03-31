//! String Combiner (Advanced)
/// Type alias for [`StringCombinerAdvanced`].
pub type Model402 = StringCombinerAdvanced;
/// String Combiner (Advanced)
///
/// An advanced string combiner
///
/// Detail: This model is SUPERSEDED by model 404
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StringCombinerAdvanced {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Power scale factor
    pub dcw_sf: Option<i16>,
    /// Energy scale factor
    pub dc_wh_sf: i16,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dca_max: Option<u16>,
    /// N
    ///
    /// Number of Inputs
    pub n: Option<u16>,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: Evt,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evt_vnd: Option<EvtVnd>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dcpr: Option<u16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dc_wh: u32,
    #[allow(missing_docs)]
    pub string: Vec<String>,
}
#[allow(missing_docs)]
impl StringCombinerAdvanced {
    pub const DCA_SF: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const DCV_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const DCW_SF: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const DC_WH_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const DCA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const N: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(7, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(9, 2, false);
    pub const DCA: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
    pub const DCV: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const DCW: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const DCPR: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const DC_WH: crate::Point<Self, u32> = crate::Point::new(18, 2, false);
}
static STRING_COMBINER_ADVANCED_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "dca_sf",
        label: "DCA_SF",
        description: "Current scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_ahr_sf",
        label: "DCAhr_SF",
        description: "Amp-hour scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcv_sf",
        label: "DCV_SF",
        description: "Voltage scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcw_sf",
        label: "DCW_SF",
        description: "Power scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh_sf",
        label: "DCWh_SF",
        description: "Energy scale factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dca_max",
        label: "Rating",
        description: "Maximum DC Current Rating",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n",
        label: "N",
        description: "Number of Inputs",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt",
        label: "Event",
        description: "Bitmask value.  Events",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd",
        label: "Vendor Event",
        description: "Bitmask value.  Vendor defined events",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dca",
        label: "Amps",
        description: "Total measured current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_ahr",
        label: "Amp-hours",
        description: "Total metered Amp-hours",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcv",
        label: "Voltage",
        description: "Output Voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp",
        label: "Temp",
        description: "Internal operating temperature",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcw",
        label: "Watts",
        description: "Output power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcpr",
        label: "PR",
        description: "DC Performance ratio value",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh",
        label: "Watt-hours",
        description: "Output energy",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "string",
        label: "string",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<String as crate::GroupMeta>::group_info),
    },
];
static STRING_COMBINER_ADVANCED_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "string_combiner_advanced",
    label: "String Combiner (Advanced)",
    description: "An advanced string combiner",
    fields: STRING_COMBINER_ADVANCED_FIELDS,
};
impl crate::GroupMeta for StringCombinerAdvanced {
    fn group_info() -> &'static crate::GroupInfo {
        &STRING_COMBINER_ADVANCED_GROUP_INFO
    }
}
impl crate::Group for StringCombinerAdvanced {
    const LEN: u16 = 20;
}
impl StringCombinerAdvanced {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, string) = String::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                dca_sf: Self::DCA_SF.from_data(data)?,
                dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
                dcv_sf: Self::DCV_SF.from_data(data)?,
                dcw_sf: Self::DCW_SF.from_data(data)?,
                dc_wh_sf: Self::DC_WH_SF.from_data(data)?,
                dca_max: Self::DCA_MAX.from_data(data)?,
                n: Self::N.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                dca: Self::DCA.from_data(data)?,
                dc_ahr: Self::DC_AHR.from_data(data)?,
                dcv: Self::DCV.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                dcw: Self::DCW.from_data(data)?,
                dcpr: Self::DCPR.from_data(data)?,
                dc_wh: Self::DC_WH.from_data(data)?,
                string,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Event"] #[doc = " "] #[doc = " Bitmask value.  Events"] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct Evt : u32 {
    #[allow(missing_docs)] const LowVoltage = 1; #[allow(missing_docs)] const LowPower =
    2; #[allow(missing_docs)] const LowEfficiency = 4; #[allow(missing_docs)] const
    Current = 8; #[allow(missing_docs)] const Voltage = 16; #[allow(missing_docs)] const
    Power = 32; #[allow(missing_docs)] const Pr = 64; #[allow(missing_docs)] const
    Disconnected = 128; #[allow(missing_docs)] const FuseFault = 256;
    #[allow(missing_docs)] const CombinerFuseFault = 512; #[allow(missing_docs)] const
    CombinerCabinetOpen = 1024; #[allow(missing_docs)] const Temp = 2048;
    #[allow(missing_docs)] const Groundfault = 4096; #[allow(missing_docs)] const
    ReversedPolarity = 8192; #[allow(missing_docs)] const Incompatible = 16384;
    #[allow(missing_docs)] const CommError = 32768; #[allow(missing_docs)] const
    InternalError = 65536; #[allow(missing_docs)] const Theft = 131072;
    #[allow(missing_docs)] const ArcDetected = 262144; }
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
bitflags::bitflags! {
    #[doc = " Vendor Event"] #[doc = " "] #[doc =
    " Bitmask value.  Vendor defined events"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct EvtVnd : u32 {}
}
impl crate::Value for EvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct String {
    /// ID
    ///
    /// Uniquely identifies this input set
    pub in_id: u16,
    /// Input Event
    ///
    /// String Input Event Flags
    pub in_evt: StringInEvt,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evt_vnd: Option<StringEvtVnd>,
    /// Amps
    ///
    /// String Input Current
    pub in_dca: i16,
    /// Amp-hours
    ///
    /// String Input Amp-Hours
    pub in_dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// String Input Voltage
    pub in_dcv: Option<u16>,
    /// Watts
    ///
    /// String Input Power
    pub in_dcw: Option<i16>,
    /// Watt-hours
    ///
    /// String Input Energy
    pub in_dc_wh: Option<u32>,
    /// PR
    ///
    /// String Performance Ratio
    pub in_dcpr: Option<u16>,
    /// N
    ///
    /// Number of modules in this input string
    pub in_n: Option<u16>,
}
#[allow(missing_docs)]
impl String {
    pub const IN_ID: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const IN_EVT: crate::Point<Self, StringInEvt> = crate::Point::new(1, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<StringEvtVnd>> = crate::Point::new(3, 2, false);
    pub const IN_DCA: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const IN_DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2, false);
    pub const IN_DCV: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const IN_DCW: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const IN_DC_WH: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2, false);
    pub const IN_DCPR: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const IN_N: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
}
static STRING_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "in_id",
        label: "ID",
        description: "Uniquely identifies this input set",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_evt",
        label: "Input Event",
        description: "String Input Event Flags",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd",
        label: "Vendor Event",
        description: "Bitmask value.  Vendor defined events",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dca",
        label: "Amps",
        description: "String Input Current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dc_ahr",
        label: "Amp-hours",
        description: "String Input Amp-Hours",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dcv",
        label: "Voltage",
        description: "String Input Voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dcw",
        label: "Watts",
        description: "String Input Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dc_wh",
        label: "Watt-hours",
        description: "String Input Energy",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_dcpr",
        label: "PR",
        description: "String Performance Ratio",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "in_n",
        label: "N",
        description: "Number of modules in this input string",
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
    const LEN: u16 = 14;
}
impl String {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                in_id: Self::IN_ID.from_data(data)?,
                in_evt: Self::IN_EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                in_dca: Self::IN_DCA.from_data(data)?,
                in_dc_ahr: Self::IN_DC_AHR.from_data(data)?,
                in_dcv: Self::IN_DCV.from_data(data)?,
                in_dcw: Self::IN_DCW.from_data(data)?,
                in_dc_wh: Self::IN_DC_WH.from_data(data)?,
                in_dcpr: Self::IN_DCPR.from_data(data)?,
                in_n: Self::IN_N.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<String as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = String::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Input Event"] #[doc = " "] #[doc = " String Input Event Flags"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct StringInEvt : u32 {
    #[allow(missing_docs)] const LowVoltage = 1; #[allow(missing_docs)] const LowPower =
    2; #[allow(missing_docs)] const LowEfficiency = 4; #[allow(missing_docs)] const
    Current = 8; #[allow(missing_docs)] const Voltage = 16; #[allow(missing_docs)] const
    Power = 32; #[allow(missing_docs)] const Pr = 64; #[allow(missing_docs)] const
    Disconnected = 128; #[allow(missing_docs)] const FuseFault = 256;
    #[allow(missing_docs)] const CombinerFuseFault = 512; #[allow(missing_docs)] const
    CombinerCabinetOpen = 1024; #[allow(missing_docs)] const Temp = 2048;
    #[allow(missing_docs)] const Groundfault = 4096; #[allow(missing_docs)] const
    ReversedPolarity = 8192; #[allow(missing_docs)] const Incompatible = 16384;
    #[allow(missing_docs)] const CommError = 32768; #[allow(missing_docs)] const
    InternalError = 65536; #[allow(missing_docs)] const Theft = 131072;
    #[allow(missing_docs)] const ArcDetected = 262144; }
}
impl crate::Value for StringInEvt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringInEvt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor Event"] #[doc = " "] #[doc =
    " Bitmask value.  Vendor defined events"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct StringEvtVnd : u32 {}
}
impl crate::Value for StringEvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringEvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for StringCombinerAdvanced {
    const ID: u16 = 402;
    const NAME: &'static str = "string_combiner_advanced";
    const LABEL: &'static str = "String Combiner (Advanced)";
    const DESCRIPTION: &'static str = "An advanced string combiner";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m402
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
