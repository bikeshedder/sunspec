//! Dynamic Reactive Current

/// Dynamic Reactive Current
///
/// Dynamic Reactive Current
///
/// Notes: Ref 3: 8.10.1.2; Ref 4: 12
#[derive(Debug)]
pub struct Model128 {
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

impl Model128 {
    pub const AR_GRA_MOD: crate::PointDef<Self, ArGraMod> = crate::PointDef::new(0, 1, true);
    pub const AR_GRA_SAG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const AR_GRA_SWELL: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, ModEna> = crate::PointDef::new(3, 1, true);
    pub const FIL_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const DB_V_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const DB_V_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const BLK_ZN_V: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, true);
    pub const HYS_BLK_ZN_V: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, true);
    pub const BLK_ZN_TMMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, true);
    pub const HOLD_TMMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, true);
    pub const AR_GRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const V_REF_PCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(12, 1, false);
}

impl crate::Model for Model128 {
    const ID: u16 = 128;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
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
        })
    }
}

#[doc = "ArGraMod\n\nIndicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband."]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[repr(u16)]
pub enum ArGraMod {
    #[doc = ""]
    Edge = 0,
    #[doc = ""]
    Center = 1,
}
impl crate::Value for ArGraMod {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ArGraMod> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ArGraMod::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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

bitflags::bitflags! { # [doc = "ModEna\n\nActivate dynamic reactive current model"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ModEna : u16 { # [doc = ""] const Enabled = 1 ; } }
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
