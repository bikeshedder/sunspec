//! HVRTX
/// HVRTX
///
/// HVRT extended curve
///
/// Notes: Ref 4: 11
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model140 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.  Bitfield value.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Notes: Setting is ignored for LVRT controls.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Notes: Setting is ignored for LVRT controls.
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
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub crv_type: CrvType,
}
#[allow(missing_docs)]
impl Model140 {
    pub const ACT_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MOD_ENA: crate::PointDef<Self, ModEna> = crate::PointDef::new(1, 1, true);
    pub const WIN_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const RVRT_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const RMP_TMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const N_CRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N_PT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const TMS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const CRV_TYPE: crate::PointDef<Self, CrvType> = crate::PointDef::new(9, 1, false);
}
impl crate::Model for Model140 {
    const ID: u16 = 140;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            act_crv: Self::ACT_CRV.from_data(data)?,
            mod_ena: Self::MOD_ENA.from_data(data)?,
            win_tms: Self::WIN_TMS.from_data(data)?,
            rvrt_tms: Self::RVRT_TMS.from_data(data)?,
            rmp_tms: Self::RMP_TMS.from_data(data)?,
            n_crv: Self::N_CRV.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
            tms_sf: Self::TMS_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            crv_type: Self::CRV_TYPE.from_data(data)?,
        })
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc =
    " LVRT control mode. Enable active curve.  Bitfield value."] #[derive(Copy, Clone,
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
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CrvType {
    #[allow(missing_docs)]
    CeaseToEnergize = 1,
}
impl crate::Value for CrvType {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CrvType> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CrvType::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
