//! HVRTX
/// Type alias for [`Hvrtx`].
pub type Model140 = Hvrtx;
/// HVRTX
///
/// HVRT extended curve
///
/// Detail: Ref 4: 11
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Hvrtx {
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
    /// Detail: Setting is ignored for LVRT controls.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Detail: Setting is ignored for LVRT controls.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Detail: Setting is ignored for LVRT controls.
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
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl Hvrtx {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const CRV_TYPE: crate::Point<Self, CrvType> = crate::Point::new(9, 1, false);
}
impl crate::Group for Hvrtx {
    const LEN: u16 = 10;
}
impl Hvrtx {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, curve) = Curve::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
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
                curve,
            },
        ))
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
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Curve {
    /// ActPt
    ///
    /// Number of active points in array.
    pub act_pt: u16,
    /// Tms1
    ///
    /// Point 1 duration.
    pub tms1: u16,
    /// V1
    ///
    /// Point 1 voltage.
    pub v1: u16,
    /// Tms2
    ///
    /// Point 2 duration.
    pub tms2: Option<u16>,
    /// V2
    ///
    /// Point 2 voltage.
    pub v2: Option<u16>,
    /// Tms3
    ///
    /// Point 3 duration.
    pub tms3: Option<u16>,
    /// V3
    ///
    /// Point 3 voltage.
    pub v3: Option<u16>,
    /// Tms4
    ///
    /// Point 4 duration.
    pub tms4: Option<u16>,
    /// V4
    ///
    /// Point 4 voltage.
    pub v4: Option<u16>,
    /// Tms5
    ///
    /// Point 5 duration.
    pub tms5: Option<u16>,
    /// V5
    ///
    /// Point 5 voltage.
    pub v5: Option<u16>,
    /// Tms6
    ///
    /// Point 6 duration.
    pub tms6: Option<u16>,
    /// V6
    ///
    /// Point 6 voltage.
    pub v6: Option<u16>,
    /// Tms7
    ///
    /// Point 7 duration.
    pub tms7: Option<u16>,
    /// V7
    ///
    /// Point 7 voltage.
    pub v7: Option<u16>,
    /// Tms8
    ///
    /// Point 8 duration.
    pub tms8: Option<u16>,
    /// V8
    ///
    /// Point 8 voltage.
    pub v8: Option<u16>,
    /// Tms9
    ///
    /// Point 9 duration.
    pub tms9: Option<u16>,
    /// V9
    ///
    /// Point 9 voltage.
    pub v9: Option<u16>,
    /// Tms10
    ///
    /// Point 10 duration.
    pub tms10: Option<u16>,
    /// V10
    ///
    /// Point 10 voltage.
    pub v10: Option<u16>,
    /// Tms11
    ///
    /// Point 11 duration.
    pub tms11: Option<u16>,
    /// V11
    ///
    /// Point 11 voltage.
    pub v11: Option<u16>,
    /// Tms12
    ///
    /// Point 12 duration.
    pub tms12: Option<u16>,
    /// V12
    ///
    /// Point 12 voltage.
    pub v12: Option<u16>,
    /// Tms13
    ///
    /// Point 13 duration.
    pub tms13: Option<u16>,
    /// V13
    ///
    /// Point 13 voltage.
    pub v13: Option<u16>,
    /// Tms14
    ///
    /// Point 14 duration.
    pub tms14: Option<u16>,
    /// V14
    ///
    /// Point 14 voltage.
    pub v14: Option<u16>,
    /// Tms15
    ///
    /// Point 15 duration.
    pub tms15: Option<u16>,
    /// V15
    ///
    /// Point 15 voltage.
    pub v15: Option<u16>,
    /// Tms16
    ///
    /// Point 16 duration.
    pub tms16: Option<u16>,
    /// V16
    ///
    /// Point 16 voltage.
    pub v16: Option<u16>,
    /// Tms17
    ///
    /// Point 17 duration.
    pub tms17: Option<u16>,
    /// V17
    ///
    /// Point 17 voltage.
    pub v17: Option<u16>,
    /// Tms18
    ///
    /// Point 18 duration.
    pub tms18: Option<u16>,
    /// V18
    ///
    /// Point 18 voltage.
    pub v18: Option<u16>,
    /// Tms19
    ///
    /// Point 19 duration.
    pub tms19: Option<u16>,
    /// V19
    ///
    /// Point 19 voltage.
    pub v19: Option<u16>,
    /// Tms20
    ///
    /// Point 20 duration.
    pub tms20: Option<u16>,
    /// V20
    ///
    /// Point 20 voltage.
    pub v20: Option<u16>,
    /// CrvNam
    ///
    /// Optional description for curve.
    pub crv_nam: Option<String>,
    /// ReadOnly
    ///
    /// Enumerated value indicates if curve is read-only or can be modified.
    pub read_only: CurveReadOnly,
}
#[allow(missing_docs)]
impl Curve {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const TMS1: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const V1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const TMS2: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const V2: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const TMS3: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const V3: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const TMS4: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const V4: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const TMS5: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const V5: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const TMS6: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const V6: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const TMS7: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const V7: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const TMS8: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, true);
    pub const V8: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, true);
    pub const TMS9: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const V9: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const TMS10: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, true);
    pub const V10: crate::Point<Self, Option<u16>> = crate::Point::new(20, 1, true);
    pub const TMS11: crate::Point<Self, Option<u16>> = crate::Point::new(21, 1, true);
    pub const V11: crate::Point<Self, Option<u16>> = crate::Point::new(22, 1, true);
    pub const TMS12: crate::Point<Self, Option<u16>> = crate::Point::new(23, 1, true);
    pub const V12: crate::Point<Self, Option<u16>> = crate::Point::new(24, 1, true);
    pub const TMS13: crate::Point<Self, Option<u16>> = crate::Point::new(25, 1, true);
    pub const V13: crate::Point<Self, Option<u16>> = crate::Point::new(26, 1, true);
    pub const TMS14: crate::Point<Self, Option<u16>> = crate::Point::new(27, 1, true);
    pub const V14: crate::Point<Self, Option<u16>> = crate::Point::new(28, 1, true);
    pub const TMS15: crate::Point<Self, Option<u16>> = crate::Point::new(29, 1, true);
    pub const V15: crate::Point<Self, Option<u16>> = crate::Point::new(30, 1, true);
    pub const TMS16: crate::Point<Self, Option<u16>> = crate::Point::new(31, 1, true);
    pub const V16: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, true);
    pub const TMS17: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, true);
    pub const V17: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const TMS18: crate::Point<Self, Option<u16>> = crate::Point::new(35, 1, true);
    pub const V18: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, true);
    pub const TMS19: crate::Point<Self, Option<u16>> = crate::Point::new(37, 1, true);
    pub const V19: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, true);
    pub const TMS20: crate::Point<Self, Option<u16>> = crate::Point::new(39, 1, true);
    pub const V20: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(41, 8, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(49, 1, false);
}
impl crate::Group for Curve {
    const LEN: u16 = 50;
}
impl Curve {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                tms1: Self::TMS1.from_data(data)?,
                v1: Self::V1.from_data(data)?,
                tms2: Self::TMS2.from_data(data)?,
                v2: Self::V2.from_data(data)?,
                tms3: Self::TMS3.from_data(data)?,
                v3: Self::V3.from_data(data)?,
                tms4: Self::TMS4.from_data(data)?,
                v4: Self::V4.from_data(data)?,
                tms5: Self::TMS5.from_data(data)?,
                v5: Self::V5.from_data(data)?,
                tms6: Self::TMS6.from_data(data)?,
                v6: Self::V6.from_data(data)?,
                tms7: Self::TMS7.from_data(data)?,
                v7: Self::V7.from_data(data)?,
                tms8: Self::TMS8.from_data(data)?,
                v8: Self::V8.from_data(data)?,
                tms9: Self::TMS9.from_data(data)?,
                v9: Self::V9.from_data(data)?,
                tms10: Self::TMS10.from_data(data)?,
                v10: Self::V10.from_data(data)?,
                tms11: Self::TMS11.from_data(data)?,
                v11: Self::V11.from_data(data)?,
                tms12: Self::TMS12.from_data(data)?,
                v12: Self::V12.from_data(data)?,
                tms13: Self::TMS13.from_data(data)?,
                v13: Self::V13.from_data(data)?,
                tms14: Self::TMS14.from_data(data)?,
                v14: Self::V14.from_data(data)?,
                tms15: Self::TMS15.from_data(data)?,
                v15: Self::V15.from_data(data)?,
                tms16: Self::TMS16.from_data(data)?,
                v16: Self::V16.from_data(data)?,
                tms17: Self::TMS17.from_data(data)?,
                v17: Self::V17.from_data(data)?,
                tms18: Self::TMS18.from_data(data)?,
                v18: Self::V18.from_data(data)?,
                tms19: Self::TMS19.from_data(data)?,
                v19: Self::V19.from_data(data)?,
                tms20: Self::TMS20.from_data(data)?,
                v20: Self::V20.from_data(data)?,
                crv_nam: Self::CRV_NAM.from_data(data)?,
                read_only: Self::READ_ONLY.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Curve as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Curve::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// ReadOnly
///
/// Enumerated value indicates if curve is read-only or can be modified.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum CurveReadOnly {
    #[allow(missing_docs)]
    Readwrite = 0,
    #[allow(missing_docs)]
    Readonly = 1,
}
impl crate::Value for CurveReadOnly {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<CurveReadOnly> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                CurveReadOnly::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
impl crate::Model for Hvrtx {
    const ID: u16 = 140;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m140
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
