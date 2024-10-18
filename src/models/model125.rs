//! Pricing
/// Pricing
///
/// Pricing Signal
///
/// Notes: Ref 3: 8.7.5.1; Ref 4: 6
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model125 {
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
impl Model125 {
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(0, 1, true);
    pub const SIG_TYPE: crate::Point<Self, Option<SigType>> = crate::Point::new(1, 1, true);
    pub const SIG: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RVT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const SIG_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
}
impl crate::Model for Model125 {
    const ID: u16 = 125;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            mod_ena: Self::MOD_ENA.from_data(data)?,
            sig_type: Self::SIG_TYPE.from_data(data)?,
            sig: Self::SIG.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvt_tms: Self::RVT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            sig_sf: Self::SIG_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m125
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
impl crate::Value for Option<ModEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(ModEna::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
/// SigType
///
/// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SigType {
    #[allow(missing_docs)]
    Unknown = 0,
    #[allow(missing_docs)]
    Absolute = 1,
    #[allow(missing_docs)]
    Relative = 2,
    #[allow(missing_docs)]
    Multiplier = 3,
    #[allow(missing_docs)]
    Level = 4,
}
impl crate::Value for SigType {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SigType> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SigType::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
