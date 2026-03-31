//! Pricing
/// Type alias for [`Pricing`].
pub type Model125 = Pricing;
/// Pricing
///
/// Pricing Signal
///
/// Detail: Ref 3: 8.7.5.1; Ref 4: 6
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pricing {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    pub mod_ena: ModEna,
    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    pub sig_type: Option<SigType>,
    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    pub sig: i16,
    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    pub win_tms: Option<u16>,
    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    pub rvt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    pub rmp_tms: Option<u16>,
    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    pub sig_sf: i16,
}
#[allow(missing_docs)]
impl Pricing {
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(0, 1, true);
    pub const SIG_TYPE: crate::Point<Self, Option<SigType>> = crate::Point::new(1, 1, true);
    pub const SIG: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RVT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const SIG_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
}
static PRICING_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "mod_ena",
        label: "ModEna",
        description: "Is price-based charge/discharge mode active?",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "sig_type",
        label: "SigType",
        description: "Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "sig",
        label: "Sig",
        description: "Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "win_tms",
        label: "WinTms",
        description: "Time window for charge/discharge pricing change.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rvt_tms",
        label: "RvtTms",
        description: "Timeout period for charge/discharge pricing change.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rmp_tms",
        label: "RmpTms",
        description: "Ramp time for moving from current charge or discharge level to new level.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "sig_sf",
        label: "Sig_SF",
        description: "Pricing signal scale factor.",
        kind: crate::FieldKind::Point,
    },
];
static PRICING_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "pricing",
    label: "Pricing",
    description: "Pricing Signal  ",
    fields: PRICING_FIELDS,
};
impl crate::GroupMeta for Pricing {
    fn group_info() -> &'static crate::GroupInfo {
        &PRICING_GROUP_INFO
    }
}
impl crate::Group for Pricing {
    const LEN: u16 = 8;
}
impl Pricing {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                mod_ena: Self::MOD_ENA.from_data(data)?,
                sig_type: Self::SIG_TYPE.from_data(data)?,
                sig: Self::SIG.from_data(data)?,
                win_tms: Self::WIN_TMS.from_data(data)?,
                rvt_tms: Self::RVT_TMS.from_data(data)?,
                rmp_tms: Self::RMP_TMS.from_data(data)?,
                sig_sf: Self::SIG_SF.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc =
    " Is price-based charge/discharge mode active?"] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct ModEna : u16 { #[allow(missing_docs)] const Enable
    = 1; }
}
impl crate::Value for ModEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ModEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
/// SigType
///
/// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SigType {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Absolute,
    #[allow(missing_docs)]
    Relative,
    #[allow(missing_docs)]
    Multiplier,
    #[allow(missing_docs)]
    Level,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SigType {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Absolute,
            2 => Self::Relative,
            3 => Self::Multiplier,
            4 => Self::Level,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Absolute => 1,
            Self::Relative => 2,
            Self::Multiplier => 3,
            Self::Level => 4,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SigType {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Pricing {
    const ID: u16 = 125;
    const NAME: &'static str = "pricing";
    const LABEL: &'static str = "Pricing";
    const DESCRIPTION: &'static str = "Pricing Signal  ";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m125
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
