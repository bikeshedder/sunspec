//! DER Storage Capacity
/// Type alias for [`DerStorageCapacity`].
pub type Model713 = DerStorageCapacity;
/// DER Storage Capacity
///
/// DER storage capacity.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerStorageCapacity {
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    pub wh_rtg: Option<u16>,
    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    pub wh_avail: Option<u16>,
    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// Detail: SOC shall be fixed to 0% for DER without storage capabilities.
    pub soc: Option<u16>,
    /// State of Health
    ///
    /// State of health of the DER storage.
    pub soh: Option<u16>,
    /// Status
    ///
    /// Storage status.
    pub sta: Option<Sta>,
    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    pub wh_sf: Option<i16>,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    pub pct_sf: Option<i16>,
}
#[allow(missing_docs)]
impl DerStorageCapacity {
    pub const WH_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const WH_AVAIL: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const SOC: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const SOH: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const STA: crate::Point<Self, Option<Sta>> = crate::Point::new(4, 1, false);
    pub const WH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, false);
    pub const PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, false);
}
impl crate::Group for DerStorageCapacity {
    const LEN: u16 = 7;
}
impl DerStorageCapacity {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                wh_rtg: Self::WH_RTG.from_data(data)?,
                wh_avail: Self::WH_AVAIL.from_data(data)?,
                soc: Self::SOC.from_data(data)?,
                soh: Self::SOH.from_data(data)?,
                sta: Self::STA.from_data(data)?,
                wh_sf: Self::WH_SF.from_data(data)?,
                pct_sf: Self::PCT_SF.from_data(data)?,
            },
        ))
    }
}
/// Status
///
/// Storage status.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Sta {
    /// OK
    ///
    /// No warnings or errors pending.
    Ok,
    /// Warning
    ///
    /// One or more warnings pending.
    Warning,
    /// Error
    ///
    /// One or more errors pending.
    Error,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Sta {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::Warning,
            2 => Self::Error,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Ok => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Sta {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for DerStorageCapacity {
    const ID: u16 = 713;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m713
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
