//! Enter Service
/// Type alias for [`DerEnterService`].
pub type Model703 = DerEnterService;
/// Enter Service
///
/// Enter service model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerEnterService {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    pub es: Option<Es>,
    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    pub esv_hi: Option<u16>,
    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    pub esv_lo: Option<u16>,
    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    pub es_hz_hi: Option<u32>,
    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    pub es_hz_lo: Option<u32>,
    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    pub es_dly_tms: Option<u32>,
    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    pub es_rnd_tms: Option<u32>,
    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    pub es_rmp_tms: Option<u32>,
    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    pub es_dly_rem_tms: Option<u32>,
    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    pub v_sf: Option<i16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    pub hz_sf: Option<i16>,
}
#[allow(missing_docs)]
impl DerEnterService {
    pub const ES: crate::Point<Self, Option<Es>> = crate::Point::new(0, 1, true);
    pub const ESV_HI: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const ESV_LO: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const ES_HZ_HI: crate::Point<Self, Option<u32>> = crate::Point::new(3, 2, true);
    pub const ES_HZ_LO: crate::Point<Self, Option<u32>> = crate::Point::new(5, 2, true);
    pub const ES_DLY_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(7, 2, true);
    pub const ES_RND_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(9, 2, true);
    pub const ES_RMP_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(11, 2, true);
    pub const ES_DLY_REM_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(13, 2, false);
    pub const V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
}
static DER_ENTER_SERVICE_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "es",
        label: "Permit Enter Service",
        description: "Permit enter service.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "esv_hi",
        label: "Enter Service Voltage High",
        description: "Enter service voltage high threshold as percent of normal voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "esv_lo",
        label: "Enter Service Voltage Low",
        description: "Enter service voltage low threshold as percent of normal voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_hz_hi",
        label: "Enter Service Frequency High",
        description: "Enter service frequency high threshold.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_hz_lo",
        label: "Enter Service Frequency Low",
        description: "Enter service frequency low threshold.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_dly_tms",
        label: "Enter Service Delay Time",
        description: "Enter service delay time in seconds.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_rnd_tms",
        label: "Enter Service Random Delay",
        description: "Enter service random delay in seconds.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_rmp_tms",
        label: "Enter Service Ramp Time",
        description: "Enter service ramp time in seconds.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "es_dly_rem_tms",
        label: "Enter Service Delay Remaining",
        description: "Enter service delay time remaining in seconds.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "Voltage Scale Factor",
        description: "Voltage percentage scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_sf",
        label: "Frequency Scale Factor",
        description: "Frequency scale factor.",
        kind: crate::FieldKind::Point,
    },
];
static DER_ENTER_SERVICE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DEREnterService",
    label: "Enter Service",
    description: "Enter service model.",
    fields: DER_ENTER_SERVICE_FIELDS,
};
impl crate::GroupMeta for DerEnterService {
    fn group_info() -> &'static crate::GroupInfo {
        &DER_ENTER_SERVICE_GROUP_INFO
    }
}
impl crate::Group for DerEnterService {
    const LEN: u16 = 17;
}
impl DerEnterService {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                es: Self::ES.from_data(data)?,
                esv_hi: Self::ESV_HI.from_data(data)?,
                esv_lo: Self::ESV_LO.from_data(data)?,
                es_hz_hi: Self::ES_HZ_HI.from_data(data)?,
                es_hz_lo: Self::ES_HZ_LO.from_data(data)?,
                es_dly_tms: Self::ES_DLY_TMS.from_data(data)?,
                es_rnd_tms: Self::ES_RND_TMS.from_data(data)?,
                es_rmp_tms: Self::ES_RMP_TMS.from_data(data)?,
                es_dly_rem_tms: Self::ES_DLY_REM_TMS.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
            },
        ))
    }
}
/// Permit Enter Service
///
/// Permit enter service.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Es {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Es {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Es {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for DerEnterService {
    const ID: u16 = 703;
    const NAME: &'static str = "DEREnterService";
    const LABEL: &'static str = "Enter Service";
    const DESCRIPTION: &'static str = "Enter service model.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m703
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
