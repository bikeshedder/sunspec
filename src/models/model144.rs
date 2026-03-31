//! HFRTX
/// Type alias for [`Hfrtx`].
pub type Model144 = Hfrtx;
/// HFRTX
///
/// HFRT extended curve
///
/// Detail: Ref 4: 11
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Hfrtx {
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
    /// Detail: Setting is ignored for LFRT controls.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Detail: Setting is ignored for LFRT controls.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Detail: Setting is ignored for LFRT controls.
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
    #[allow(missing_docs)]
    pub crv_type: CrvType,
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl Hfrtx {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const HZ_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const CRV_TYPE: crate::Point<Self, CrvType> = crate::Point::new(9, 1, false);
}
impl crate::Group for Hfrtx {
    const LEN: u16 = 10;
}
impl Hfrtx {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
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
                hz_sf: Self::HZ_SF.from_data(data)?,
                crv_type: Self::CRV_TYPE.from_data(data)?,
                curve,
            },
        ))
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
impl crate::FixedSize for ModEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CrvType {
    #[allow(missing_docs)]
    CeaseToEnergize,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CrvType {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::CeaseToEnergize,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::CeaseToEnergize => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CrvType {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
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
    /// Hz1
    ///
    /// Point 1 frequency.
    pub hz1: u16,
    /// Tms2
    ///
    /// Point 2 duration.
    pub tms2: Option<u16>,
    /// Hz2
    ///
    /// Point 2 frequency.
    pub hz2: Option<u16>,
    /// Tms3
    ///
    /// Point 3 duration.
    pub tms3: Option<u16>,
    /// Hz3
    ///
    /// Point 3 frequency.
    pub hz3: Option<u16>,
    /// Tms4
    ///
    /// Point 4 duration.
    pub tms4: Option<u16>,
    /// Hz4
    ///
    /// Point 4 frequency.
    pub hz4: Option<u16>,
    /// Tms5
    ///
    /// Point 5 duration.
    pub tms5: Option<u16>,
    /// Hz5
    ///
    /// Point 5 frequency.
    pub hz5: Option<u16>,
    /// Tms6
    ///
    /// Point 6 duration.
    pub tms6: Option<u16>,
    /// Hz6
    ///
    /// Point 6 frequency.
    pub hz6: Option<u16>,
    /// Tms7
    ///
    /// Point 7 duration.
    pub tms7: Option<u16>,
    /// Hz7
    ///
    /// Point 7 frequency.
    pub hz7: Option<u16>,
    /// Tms8
    ///
    /// Point 8 duration.
    pub tms8: Option<u16>,
    /// Hz8
    ///
    /// Point 8 frequency.
    pub hz8: Option<u16>,
    /// Tms9
    ///
    /// Point 9 duration.
    pub tms9: Option<u16>,
    /// Hz9
    ///
    /// Point 9 frequency.
    pub hz9: Option<u16>,
    /// Tms10
    ///
    /// Point 10 duration.
    pub tms10: Option<u16>,
    /// Hz10
    ///
    /// Point 10 frequency.
    pub hz10: Option<u16>,
    /// Tms11
    ///
    /// Point 11 duration.
    pub tms11: Option<u16>,
    /// Hz11
    ///
    /// Point 11 frequency.
    pub hz11: Option<u16>,
    /// Tms12
    ///
    /// Point 12 duration.
    pub tms12: Option<u16>,
    /// Hz12
    ///
    /// Point 12 frequency.
    pub hz12: Option<u16>,
    /// Tms13
    ///
    /// Point 13 duration.
    pub tms13: Option<u16>,
    /// Hz13
    ///
    /// Point 13 frequency.
    pub hz13: Option<u16>,
    /// Tms14
    ///
    /// Point 14 duration.
    pub tms14: Option<u16>,
    /// Hz14
    ///
    /// Point 14 frequency.
    pub hz14: Option<u16>,
    /// Tms15
    ///
    /// Point 15 duration.
    pub tms15: Option<u16>,
    /// Hz15
    ///
    /// Point 15 frequency.
    pub hz15: Option<u16>,
    /// Tms16
    ///
    /// Point 16 duration.
    pub tms16: Option<u16>,
    /// Hz16
    ///
    /// Point 16 frequency.
    pub hz16: Option<u16>,
    /// Tms17
    ///
    /// Point 17 duration.
    pub tms17: Option<u16>,
    /// Hz17
    ///
    /// Point 17 frequency.
    pub hz17: Option<u16>,
    /// Tms18
    ///
    /// Point 18 duration.
    pub tms18: Option<u16>,
    /// Hz18
    ///
    /// Point 18 frequency.
    pub hz18: Option<u16>,
    /// Tms19
    ///
    /// Point 19 duration.
    pub tms19: Option<u16>,
    /// Hz19
    ///
    /// Point 19 frequency.
    pub hz19: Option<u16>,
    /// Tms20
    ///
    /// Point 20 duration.
    pub tms20: Option<u16>,
    /// Hz20
    ///
    /// Point 20 frequency.
    pub hz20: Option<u16>,
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
    pub const HZ1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const TMS2: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const HZ2: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const TMS3: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const HZ3: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const TMS4: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const HZ4: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const TMS5: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const HZ5: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const TMS6: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const HZ6: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const TMS7: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const HZ7: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const TMS8: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, true);
    pub const HZ8: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, true);
    pub const TMS9: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const HZ9: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const TMS10: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, true);
    pub const HZ10: crate::Point<Self, Option<u16>> = crate::Point::new(20, 1, true);
    pub const TMS11: crate::Point<Self, Option<u16>> = crate::Point::new(21, 1, true);
    pub const HZ11: crate::Point<Self, Option<u16>> = crate::Point::new(22, 1, true);
    pub const TMS12: crate::Point<Self, Option<u16>> = crate::Point::new(23, 1, true);
    pub const HZ12: crate::Point<Self, Option<u16>> = crate::Point::new(24, 1, true);
    pub const TMS13: crate::Point<Self, Option<u16>> = crate::Point::new(25, 1, true);
    pub const HZ13: crate::Point<Self, Option<u16>> = crate::Point::new(26, 1, true);
    pub const TMS14: crate::Point<Self, Option<u16>> = crate::Point::new(27, 1, true);
    pub const HZ14: crate::Point<Self, Option<u16>> = crate::Point::new(28, 1, true);
    pub const TMS15: crate::Point<Self, Option<u16>> = crate::Point::new(29, 1, true);
    pub const HZ15: crate::Point<Self, Option<u16>> = crate::Point::new(30, 1, true);
    pub const TMS16: crate::Point<Self, Option<u16>> = crate::Point::new(31, 1, true);
    pub const HZ16: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, true);
    pub const TMS17: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, true);
    pub const HZ17: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const TMS18: crate::Point<Self, Option<u16>> = crate::Point::new(35, 1, true);
    pub const HZ18: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, true);
    pub const TMS19: crate::Point<Self, Option<u16>> = crate::Point::new(37, 1, true);
    pub const HZ19: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, true);
    pub const TMS20: crate::Point<Self, Option<u16>> = crate::Point::new(39, 1, true);
    pub const HZ20: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(41, 8, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(49, 1, false);
}
impl crate::Group for Curve {
    const LEN: u16 = 50;
}
impl Curve {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                tms1: Self::TMS1.from_data(data)?,
                hz1: Self::HZ1.from_data(data)?,
                tms2: Self::TMS2.from_data(data)?,
                hz2: Self::HZ2.from_data(data)?,
                tms3: Self::TMS3.from_data(data)?,
                hz3: Self::HZ3.from_data(data)?,
                tms4: Self::TMS4.from_data(data)?,
                hz4: Self::HZ4.from_data(data)?,
                tms5: Self::TMS5.from_data(data)?,
                hz5: Self::HZ5.from_data(data)?,
                tms6: Self::TMS6.from_data(data)?,
                hz6: Self::HZ6.from_data(data)?,
                tms7: Self::TMS7.from_data(data)?,
                hz7: Self::HZ7.from_data(data)?,
                tms8: Self::TMS8.from_data(data)?,
                hz8: Self::HZ8.from_data(data)?,
                tms9: Self::TMS9.from_data(data)?,
                hz9: Self::HZ9.from_data(data)?,
                tms10: Self::TMS10.from_data(data)?,
                hz10: Self::HZ10.from_data(data)?,
                tms11: Self::TMS11.from_data(data)?,
                hz11: Self::HZ11.from_data(data)?,
                tms12: Self::TMS12.from_data(data)?,
                hz12: Self::HZ12.from_data(data)?,
                tms13: Self::TMS13.from_data(data)?,
                hz13: Self::HZ13.from_data(data)?,
                tms14: Self::TMS14.from_data(data)?,
                hz14: Self::HZ14.from_data(data)?,
                tms15: Self::TMS15.from_data(data)?,
                hz15: Self::HZ15.from_data(data)?,
                tms16: Self::TMS16.from_data(data)?,
                hz16: Self::HZ16.from_data(data)?,
                tms17: Self::TMS17.from_data(data)?,
                hz17: Self::HZ17.from_data(data)?,
                tms18: Self::TMS18.from_data(data)?,
                hz18: Self::HZ18.from_data(data)?,
                tms19: Self::TMS19.from_data(data)?,
                hz19: Self::HZ19.from_data(data)?,
                tms20: Self::TMS20.from_data(data)?,
                hz20: Self::HZ20.from_data(data)?,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CurveReadOnly {
    #[allow(missing_docs)]
    Readwrite,
    #[allow(missing_docs)]
    Readonly,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CurveReadOnly {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Readwrite,
            1 => Self::Readonly,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Readwrite => 0,
            Self::Readonly => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CurveReadOnly {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Hfrtx {
    const ID: u16 = 144;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m144
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
