//! Freq-Watt Param
/// Freq-Watt Param
///
/// Parameterized Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model127 {
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
impl Model127 {
    pub const W_GRA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const HZ_STR: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, true);
    pub const HZ_STOP: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const HYS_ENA: crate::PointDef<Self, HysEna> = crate::PointDef::new(3, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, ModEna> = crate::PointDef::new(4, 1, true);
    pub const HZ_STOP_W_GRA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const W_GRA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, false);
    pub const HZ_STR_STOP_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(7, 1, false);
    pub const RMP_INC_DEC_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(8, 1, false);
}
impl crate::Model for Model127 {
    const ID: u16 = 127;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            w_gra: Self::W_GRA.from_data(data)?,
            hz_str: Self::HZ_STR.from_data(data)?,
            hz_stop: Self::HZ_STOP.from_data(data)?,
            hys_ena: Self::HYS_ENA.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            hz_stop_w_gra: Self::HZ_STOP_W_GRA.from_data(data)?,
            w_gra_sf: Self::W_GRA_SF.from_data(data)?,
            hz_str_stop_sf: Self::HZ_STR_STOP_SF.from_data(data)?,
            rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
        })
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
impl crate::Value for Option<HysEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(HysEna::from_bits_retain(value)))
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
