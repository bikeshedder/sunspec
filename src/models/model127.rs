//! Freq-Watt Param
/// Type alias for [`FreqWattParam`].
pub type Model127 = FreqWattParam;
/// Freq-Watt Param
///
/// Parameterized Frequency-Watt
///
/// Detail: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FreqWattParam {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    pub w_gra: u16,
    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    pub hz_str: i16,
    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    pub hz_stop: i16,
    /// HysEna
    ///
    /// Enable hysteresis
    pub hys_ena: HysEna,
    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    pub mod_ena: ModEna,
    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    pub hz_stop_w_gra: Option<u16>,
    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    pub w_gra_sf: Option<i16>,
    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    pub hz_str_stop_sf: Option<i16>,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmp_inc_dec_sf: Option<i16>,
}
#[allow(missing_docs)]
impl FreqWattParam {
    pub const W_GRA: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const HZ_STR: crate::Point<Self, i16> = crate::Point::new(1, 1, true);
    pub const HZ_STOP: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const HYS_ENA: crate::Point<Self, HysEna> = crate::Point::new(3, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(4, 1, true);
    pub const HZ_STOP_W_GRA: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const W_GRA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, false);
    pub const HZ_STR_STOP_SF: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, false);
    pub const RMP_INC_DEC_SF: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
}
static FREQ_WATT_PARAM_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "w_gra",
        label: "WGra",
        description: "The slope of the reduction in the maximum allowed watts output as a function of frequency.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_str",
        label: "HzStr",
        description: "The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_stop",
        label: "HzStop",
        description: "The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hys_ena",
        label: "HysEna",
        description: "Enable hysteresis",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mod_ena",
        label: "ModEna",
        description: "Is Parameterized Frequency-Watt control active.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_stop_w_gra",
        label: "HzStopWGra",
        description: "The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_gra_sf",
        label: "WGra_SF",
        description: "Scale factor for output gradient.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_str_stop_sf",
        label: "HzStrStop_SF",
        description: "Scale factor for frequency deviations.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rmp_inc_dec_sf",
        label: "RmpIncDec_SF",
        description: "Scale factor for increment and decrement ramps.",
        kind: crate::FieldKind::Point,
    },
];
static FREQ_WATT_PARAM_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "freq_watt_param",
    label: "Freq-Watt Param",
    description: "Parameterized Frequency-Watt ",
    fields: FREQ_WATT_PARAM_FIELDS,
};
impl crate::GroupMeta for FreqWattParam {
    fn group_info() -> &'static crate::GroupInfo {
        &FREQ_WATT_PARAM_GROUP_INFO
    }
}
impl crate::Group for FreqWattParam {
    const LEN: u16 = 10;
}
impl FreqWattParam {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                w_gra: Self::W_GRA.from_data(data)?,
                hz_str: Self::HZ_STR.from_data(data)?,
                hz_stop: Self::HZ_STOP.from_data(data)?,
                hys_ena: Self::HYS_ENA.from_data(data)?,
                mod_ena: Self::MOD_ENA.from_data(data)?,
                hz_stop_w_gra: Self::HZ_STOP_W_GRA.from_data(data)?,
                w_gra_sf: Self::W_GRA_SF.from_data(data)?,
                hz_str_stop_sf: Self::HZ_STR_STOP_SF.from_data(data)?,
                rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " HysEna"] #[doc = " "] #[doc = " Enable hysteresis"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct HysEna : u16 { #[allow(missing_docs)] const
    Enabled = 1; }
}
impl crate::Value for HysEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for HysEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc =
    " Is Parameterized Frequency-Watt control active."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct ModEna : u16 { #[allow(missing_docs)] const
    Enabled = 1; }
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
impl crate::Model for FreqWattParam {
    const ID: u16 = 127;
    const NAME: &'static str = "freq_watt_param";
    const LABEL: &'static str = "Freq-Watt Param";
    const DESCRIPTION: &'static str = "Parameterized Frequency-Watt ";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m127
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
