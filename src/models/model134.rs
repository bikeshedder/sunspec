//! Freq-Watt Crv

/// Freq-Watt Crv
///
/// Curve-Based Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
pub struct Model134 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for freq-watt change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 10).
    pub n_pt: u16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    /// W_SF
    ///
    /// Scale factor for percent WRef.
    pub w_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmp_inc_dec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model134 {
    pub const ACT_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, ModEna> = crate::PointDef::new(1, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model134 {
    const ID: u16 = 134;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
        })
    }
}

bitflags::bitflags! { # [doc = "ModEna\n\nIs curve-based Frequency-Watt control active."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ModEna : u16 { # [doc = ""] const Enabled = 1 ; } }
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
