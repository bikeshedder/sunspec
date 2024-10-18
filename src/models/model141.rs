//! LFRTC
/// LFRTC
///
/// LFRT must remain connected
///
/// Notes: Ref 4: 11
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model141 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.  Bitfield value.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LFRT controls.
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub n_pt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    pub tms_sf: i16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
}
#[allow(missing_docs)]
impl Model141 {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const HZ_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
}
impl crate::Model for Model141 {
    const ID: u16 = 141;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m141
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc =
    " LHzRT control mode. Enable active curve.  Bitfield value."] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
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
