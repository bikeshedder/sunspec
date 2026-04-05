//! DER DC Measurement
/// Type alias for [`DerMeasureDc`].
pub type Model714 = DerMeasureDc;
struct Counts {
    n_prt: Option<u16>,
}
/// DER DC Measurement
///
/// DER DC measurement.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerMeasureDc {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    ///
    /// Comments: DC General
    pub prt_alrms: Option<PrtAlrms>,
    /// Number Of Ports
    ///
    /// Number of DC ports.
    pub n_prt: Option<u16>,
    /// DC Current
    ///
    /// Total DC current for all ports.
    pub dca: Option<i16>,
    /// DC Power
    ///
    /// Total DC power for all ports.
    pub dcw: Option<i16>,
    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    pub dc_wh_inj: Option<u64>,
    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    pub dc_wh_abs: Option<u64>,
    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    pub dca_sf: Option<i16>,
    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    pub dcv_sf: Option<i16>,
    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    pub dcw_sf: Option<i16>,
    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    pub dcwh_sf: Option<i16>,
    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    pub tmp_sf: Option<i16>,
    /// Comments: DC Port
    pub prt: Vec<Prt>,
}
#[allow(missing_docs)]
impl DerMeasureDc {
    pub const PRT_ALRMS: crate::Point<Self, Option<PrtAlrms>> = crate::Point::new(0, 2, false);
    pub const N_PRT: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const DCA: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const DCW: crate::Point<Self, Option<i16>> = crate::Point::new(4, 1, false);
    pub const DC_WH_INJ: crate::Point<Self, Option<u64>> = crate::Point::new(5, 4, false);
    pub const DC_WH_ABS: crate::Point<Self, Option<u64>> = crate::Point::new(9, 4, false);
    pub const DCA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, false);
    pub const DCV_SF: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, false);
    pub const DCW_SF: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const DCWH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const TMP_SF: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, false);
}
static DER_MEASURE_DC_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "prt_alrms",
        label: "Port Alarms",
        description: "Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_prt",
        label: "Number Of Ports",
        description: "Number of DC ports.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dca",
        label: "DC Current",
        description: "Total DC current for all ports.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcw",
        label: "DC Power",
        description: "Total DC power for all ports.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh_inj",
        label: "DC Energy Injected",
        description: "Total cumulative DC energy injected for all ports.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh_abs",
        label: "DC Energy Absorbed",
        description: "Total cumulative DC energy absorbed for all ports.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dca_sf",
        label: "DC Current Scale Factor",
        description: "DC current scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcv_sf",
        label: "DC Voltage Scale Factor",
        description: "DC voltage scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcw_sf",
        label: "DC Power Scale Factor",
        description: "DC power scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcwh_sf",
        label: "DC Energy Scale Factor",
        description: "DC energy scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_sf",
        label: "Temperature Scale Factor",
        description: "Temperature Scale Factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "prt",
        label: "Prt",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Prt as crate::GroupMeta>::group_info),
    },
];
static DER_MEASURE_DC_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DERMeasureDC",
    label: "DER DC Measurement",
    description: "DER DC measurement.",
    fields: DER_MEASURE_DC_FIELDS,
};
impl crate::GroupMeta for DerMeasureDc {
    fn group_info() -> &'static crate::GroupInfo {
        &DER_MEASURE_DC_GROUP_INFO
    }
}
impl crate::Group for DerMeasureDc {
    const LEN: u16 = 18;
}
impl DerMeasureDc {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            n_prt: Self::N_PRT.from_data(data)?,
        };
        let (nested_data, prt) = Prt::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                prt_alrms: Self::PRT_ALRMS.from_data(data)?,
                n_prt: Self::N_PRT.from_data(data)?,
                dca: Self::DCA.from_data(data)?,
                dcw: Self::DCW.from_data(data)?,
                dc_wh_inj: Self::DC_WH_INJ.from_data(data)?,
                dc_wh_abs: Self::DC_WH_ABS.from_data(data)?,
                dca_sf: Self::DCA_SF.from_data(data)?,
                dcv_sf: Self::DCV_SF.from_data(data)?,
                dcw_sf: Self::DCW_SF.from_data(data)?,
                dcwh_sf: Self::DCWH_SF.from_data(data)?,
                tmp_sf: Self::TMP_SF.from_data(data)?,
                prt,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Port Alarms"] #[doc = " "] #[doc =
    " Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port."]
    #[doc = " "] #[doc = " Comments: DC General"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct PrtAlrms : u32 {}
}
impl crate::Value for PrtAlrms {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for PrtAlrms {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Comments: DC Port
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Prt {
    /// Port Type
    ///
    /// Port type.
    pub prt_typ: Option<PrtPrtTyp>,
    /// Port ID
    ///
    /// Port ID.
    pub id: Option<u16>,
    /// Port ID String
    ///
    /// Port ID string.
    pub id_str: Option<String>,
    /// DC Current
    ///
    /// DC current for the port.
    pub dca: Option<i16>,
    /// DC Voltage
    ///
    /// DC voltage for the port.
    pub dcv: Option<u16>,
    /// DC Power
    ///
    /// DC power for the port.
    pub dcw: Option<i16>,
    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for the port.
    pub dc_wh_inj: Option<u64>,
    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for the port.
    pub dc_wh_abs: Option<u64>,
    /// DC Port Temperature
    ///
    /// DC port temperature.
    pub tmp: Option<i16>,
    /// DC Port Status
    ///
    /// DC port status.
    pub dc_sta: Option<PrtDcSta>,
    /// DC Port Alarm
    ///
    /// DC port alarm.
    pub dc_alrm: Option<PrtDcAlrm>,
}
#[allow(missing_docs)]
impl Prt {
    pub const PRT_TYP: crate::Point<Self, Option<PrtPrtTyp>> = crate::Point::new(0, 1, false);
    pub const ID: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const ID_STR: crate::Point<Self, Option<String>> = crate::Point::new(2, 8, false);
    pub const DCA: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, false);
    pub const DCV: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const DCW: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, false);
    pub const DC_WH_INJ: crate::Point<Self, Option<u64>> = crate::Point::new(13, 4, false);
    pub const DC_WH_ABS: crate::Point<Self, Option<u64>> = crate::Point::new(17, 4, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, false);
    pub const DC_STA: crate::Point<Self, Option<PrtDcSta>> = crate::Point::new(22, 1, false);
    pub const DC_ALRM: crate::Point<Self, Option<PrtDcAlrm>> = crate::Point::new(23, 2, false);
}
static PRT_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "prt_typ",
        label: "Port Type",
        description: "Port type.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "id",
        label: "Port ID",
        description: "Port ID.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "id_str",
        label: "Port ID String",
        description: "Port ID string.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dca",
        label: "DC Current",
        description: "DC current for the port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcv",
        label: "DC Voltage",
        description: "DC voltage for the port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dcw",
        label: "DC Power",
        description: "DC power for the port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh_inj",
        label: "DC Energy Injected",
        description: "Total cumulative DC energy injected for the port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_wh_abs",
        label: "DC Energy Absorbed",
        description: "Total cumulative DC energy absorbed for the port.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp",
        label: "DC Port Temperature",
        description: "DC port temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_sta",
        label: "DC Port Status",
        description: "DC port status.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dc_alrm",
        label: "DC Port Alarm",
        description: "DC port alarm.",
        kind: crate::FieldKind::Point,
    },
];
static PRT_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "Prt",
    label: "Prt",
    description: "",
    fields: PRT_FIELDS,
};
impl crate::GroupMeta for Prt {
    fn group_info() -> &'static crate::GroupInfo {
        &PRT_GROUP_INFO
    }
}
impl crate::Group for Prt {
    const LEN: u16 = 25;
}
impl Prt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                prt_typ: Self::PRT_TYP.from_data(data)?,
                id: Self::ID.from_data(data)?,
                id_str: Self::ID_STR.from_data(data)?,
                dca: Self::DCA.from_data(data)?,
                dcv: Self::DCV.from_data(data)?,
                dcw: Self::DCW.from_data(data)?,
                dc_wh_inj: Self::DC_WH_INJ.from_data(data)?,
                dc_wh_abs: Self::DC_WH_ABS.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                dc_sta: Self::DC_STA.from_data(data)?,
                dc_alrm: Self::DC_ALRM.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) = (0..counts.n_prt.unwrap_or_default()).try_fold(
            (data, Vec::new()),
            |(data, mut groups), _| {
                let (data, group) = Prt::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            },
        )?;
        Ok((data, groups))
    }
}
/// Port Type
///
/// Port type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PrtPrtTyp {
    /// Photovoltaic
    Pv,
    /// Energy Storage System
    Ess,
    /// Electric Vehicle
    Ev,
    /// Generic Injecting
    Inj,
    /// Generic Absorbing
    Abs,
    /// Generic Bidirectional
    Bidir,
    /// DC to DC
    DcDc,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for PrtPrtTyp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Pv,
            1 => Self::Ess,
            2 => Self::Ev,
            3 => Self::Inj,
            4 => Self::Abs,
            5 => Self::Bidir,
            6 => Self::DcDc,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Pv => 0,
            Self::Ess => 1,
            Self::Ev => 2,
            Self::Inj => 3,
            Self::Abs => 4,
            Self::Bidir => 5,
            Self::DcDc => 6,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PrtPrtTyp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// DC Port Status
///
/// DC port status.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum PrtDcSta {
    /// Off
    Off,
    /// On
    On,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for PrtDcSta {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            2 => Self::Warning,
            3 => Self::Error,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for PrtDcSta {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " DC Port Alarm"] #[doc = " "] #[doc = " DC port alarm."] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct PrtDcAlrm : u32 { #[doc
    = " Ground Fault"] const GroundFault = 1; #[doc = " Input Over Voltage"] const
    InputOverVoltage = 2; #[doc = " DC Disconnect"] const DcDisconnect = 8; #[doc =
    " Cabinet Open"] const CabinetOpen = 32; #[doc = " Manual Shutdown"] const
    ManualShutdown = 64; #[doc = " Over Temperature"] const OverTemp = 128; #[doc =
    " Blown Fuse"] const BlownFuse = 4096; #[doc = " Under Temperature"] const UnderTemp
    = 8192; #[doc = " Memory Loss"] const MemoryLoss = 16384; #[doc = " Arc Detection"]
    const ArcDetection = 32768; #[doc = " Reserved"] const Reserved = 524288; #[doc =
    " Test Failed"] const TestFailed = 1048576; #[doc = " Under Voltage"] const
    InputUnderVoltage = 2097152; #[doc = " Over Current"] const InputOverCurrent =
    4194304; }
}
impl crate::Value for PrtDcAlrm {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for PrtDcAlrm {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for DerMeasureDc {
    const ID: u16 = 714;
    const NAME: &'static str = "DERMeasureDC";
    const LABEL: &'static str = "DER DC Measurement";
    const DESCRIPTION: &'static str = "DER DC measurement.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m714
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
