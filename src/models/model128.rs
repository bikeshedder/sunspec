//! Dynamic Reactive Current
/// Type alias for [`ReactiveCurrent`].
pub type Model128 = ReactiveCurrent;
/// Dynamic Reactive Current
///
/// Dynamic Reactive Current
///
/// Detail: Ref 3: 8.10.1.2; Ref 4: 12
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ReactiveCurrent {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    pub ar_gra_mod: ArGraMod,
    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    pub ar_gra_sag: u16,
    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current.  A value of 0 indicates no additional reactive current support.
    pub ar_gra_swell: u16,
    /// ModEna
    ///
    /// Activate dynamic reactive current model
    pub mod_ena: ModEna,
    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    pub fil_tms: Option<u16>,
    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    pub db_v_min: Option<u16>,
    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    pub db_v_max: Option<u16>,
    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    pub blk_zn_v: Option<u16>,
    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    pub hys_blk_zn_v: Option<u16>,
    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    pub blk_zn_tmms: Option<u16>,
    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    pub hold_tmms: Option<u16>,
    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    pub ar_gra_sf: i16,
    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    pub v_ref_pct_sf: Option<i16>,
}
#[allow(missing_docs)]
impl ReactiveCurrent {
    pub const AR_GRA_MOD: crate::Point<Self, ArGraMod> = crate::Point::new(0, 1, true);
    pub const AR_GRA_SAG: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const AR_GRA_SWELL: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(3, 1, true);
    pub const FIL_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const DB_V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const DB_V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const BLK_ZN_V: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const HYS_BLK_ZN_V: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const BLK_ZN_TMMS: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const HOLD_TMMS: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const AR_GRA_SF: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const V_REF_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::AR_GRA_MOD.is_invalid(&self.ar_gra_mod)
            || Self::AR_GRA_SAG.is_invalid(&self.ar_gra_sag)
            || Self::AR_GRA_SWELL.is_invalid(&self.ar_gra_swell)
            || Self::MOD_ENA.is_invalid(&self.mod_ena)
            || Self::AR_GRA_SF.is_invalid(&self.ar_gra_sf)
    }
}
impl crate::Group for ReactiveCurrent {
    const LEN: u16 = 14;
}
impl ReactiveCurrent {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                ar_gra_mod: Self::AR_GRA_MOD.from_data(data)?,
                ar_gra_sag: Self::AR_GRA_SAG.from_data(data)?,
                ar_gra_swell: Self::AR_GRA_SWELL.from_data(data)?,
                mod_ena: Self::MOD_ENA.from_data(data)?,
                fil_tms: Self::FIL_TMS.from_data(data)?,
                db_v_min: Self::DB_V_MIN.from_data(data)?,
                db_v_max: Self::DB_V_MAX.from_data(data)?,
                blk_zn_v: Self::BLK_ZN_V.from_data(data)?,
                hys_blk_zn_v: Self::HYS_BLK_ZN_V.from_data(data)?,
                blk_zn_tmms: Self::BLK_ZN_TMMS.from_data(data)?,
                hold_tmms: Self::HOLD_TMMS.from_data(data)?,
                ar_gra_sf: Self::AR_GRA_SF.from_data(data)?,
                v_ref_pct_sf: Self::V_REF_PCT_SF.from_data(data)?,
            },
        ))
    }
}
/// ArGraMod
///
/// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ArGraMod {
    #[allow(missing_docs)]
    Edge,
    #[allow(missing_docs)]
    Center,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ArGraMod {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Edge,
            1 => Self::Center,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Edge => 0,
            Self::Center => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ArGraMod {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc = " Activate dynamic reactive current model"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct ModEna : u16 {
    #[allow(missing_docs)] const Enabled = 1; }
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
impl crate::Model for ReactiveCurrent {
    const ID: u16 = 128;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m128
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        if model.has_invalid_points() {
            Err(crate::ParseError::InvalidPointData(
                crate::InvalidPointData { model },
            ))
        } else {
            Ok(model)
        }
    }
}
