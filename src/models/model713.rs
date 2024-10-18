//! DER Storage Capacity
/// DER Storage Capacity
///
/// DER storage capacity.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model713 {
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
    pub so_c: Option<u16>,
    /// State of Health
    ///
    /// State of health of the DER storage.
    pub so_h: Option<u16>,
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
impl Model713 {
    pub const WH_RTG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, false);
    pub const WH_AVAIL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, false);
    pub const SO_C: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const SO_H: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
    pub const STA: crate::PointDef<Self, Option<Sta>> = crate::PointDef::new(4, 1, false);
    pub const WH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(5, 1, false);
    pub const PCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, false);
}
impl crate::Model for Model713 {
    const ID: u16 = 713;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            wh_rtg: Self::WH_RTG.from_data(data)?,
            wh_avail: Self::WH_AVAIL.from_data(data)?,
            so_c: Self::SO_C.from_data(data)?,
            so_h: Self::SO_H.from_data(data)?,
            sta: Self::STA.from_data(data)?,
            wh_sf: Self::WH_SF.from_data(data)?,
            pct_sf: Self::PCT_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m713
    }
}
/// Status
///
/// Storage status.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Sta {
    /// OK
    ///
    /// No warnings or errors pending.
    Ok = 0,
    /// Warning
    ///
    /// One or more warnings pending.
    Warning = 1,
    /// Error
    ///
    /// One or more errors pending.
    Error = 2,
}
impl crate::Value for Sta {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Sta> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Sta::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
