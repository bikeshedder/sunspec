//! Watt-PF
/// Type alias for [`WattPf`].
pub type Model131 = WattPf;
/// Watt-PF
///
/// Watt-Power Factor
///
/// Detail: Ref 3: 8.11.1.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct WattPf {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is watt-PF mode active.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for watt-PF change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Max number of points in array.
    pub n_pt: u16,
    /// W_SF
    ///
    /// Scale factor for percent WMax.
    pub w_sf: i16,
    /// PF_SF
    ///
    /// Scale factor for PF.
    pub pf_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmp_inc_dec_sf: Option<i16>,
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl WattPf {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const PF_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
}
impl crate::Group for WattPf {
    const LEN: u16 = 10;
}
impl WattPf {
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
                w_sf: Self::W_SF.from_data(data)?,
                pf_sf: Self::PF_SF.from_data(data)?,
                rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
                curve,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc = " Is watt-PF mode active."] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
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
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Curve {
    /// ActPt
    ///
    /// Number of active points in array.
    pub act_pt: u16,
    /// W1
    ///
    /// Point 1 Watts.
    pub w1: i16,
    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    pub pf1: i16,
    /// W2
    ///
    /// Point 2 Watts.
    pub w2: Option<i16>,
    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    pub pf2: Option<i16>,
    /// W3
    ///
    /// Point 3 Watts.
    pub w3: Option<i16>,
    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    pub pf3: Option<i16>,
    /// W4
    ///
    /// Point 4 Watts.
    pub w4: Option<i16>,
    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    pub pf4: Option<i16>,
    /// W5
    ///
    /// Point 5 Watts.
    pub w5: Option<i16>,
    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    pub pf5: Option<i16>,
    /// W6
    ///
    /// Point 6 Watts.
    pub w6: Option<i16>,
    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    pub pf6: Option<i16>,
    /// W7
    ///
    /// Point 7 Watts.
    pub w7: Option<i16>,
    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    pub pf7: Option<i16>,
    /// W8
    ///
    /// Point 8 Watts.
    pub w8: Option<i16>,
    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    pub pf8: Option<i16>,
    /// W9
    ///
    /// Point 9 Watts.
    pub w9: Option<i16>,
    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    pub pf9: Option<i16>,
    /// W10
    ///
    /// Point 10 Watts.
    pub w10: Option<i16>,
    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    pub pf10: Option<i16>,
    /// W11
    ///
    /// Point 11 Watts.
    pub w11: Option<i16>,
    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    pub pf11: Option<i16>,
    /// W12
    ///
    /// Point 12 Watts.
    pub w12: Option<i16>,
    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    pub pf12: Option<i16>,
    /// W13
    ///
    /// Point 13 Watts.
    pub w13: Option<i16>,
    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    pub pf13: Option<i16>,
    /// W14
    ///
    /// Point 14 Watts.
    pub w14: Option<i16>,
    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    pub pf14: Option<i16>,
    /// W15
    ///
    /// Point 15 Watts.
    pub w15: Option<i16>,
    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    pub pf15: Option<i16>,
    /// W16
    ///
    /// Point 16 Watts.
    pub w16: Option<i16>,
    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    pub pf16: Option<i16>,
    /// W17
    ///
    /// Point 17 Watts.
    pub w17: Option<i16>,
    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    pub pf17: Option<i16>,
    /// W18
    ///
    /// Point 18 Watts.
    pub w18: Option<i16>,
    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    pub pf18: Option<i16>,
    /// W19
    ///
    /// Point 19 Watts.
    pub w19: Option<i16>,
    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    pub pf19: Option<i16>,
    /// W20
    ///
    /// Point 20 Watts.
    pub w20: Option<i16>,
    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    pub pf20: Option<i16>,
    /// CrvNam
    ///
    /// Optional description for curve.
    pub crv_nam: Option<String>,
    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_pt1_tms: Option<u16>,
    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    pub rmp_dec_tmm: Option<u16>,
    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
    pub rmp_inc_tmm: Option<u16>,
    /// ReadOnly
    ///
    /// Enumerated value indicates if curve is read-only or can be modified.
    pub read_only: CurveReadOnly,
}
#[allow(missing_docs)]
impl Curve {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const W1: crate::Point<Self, i16> = crate::Point::new(1, 1, true);
    pub const PF1: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const W2: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, true);
    pub const PF2: crate::Point<Self, Option<i16>> = crate::Point::new(4, 1, true);
    pub const W3: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, true);
    pub const PF3: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, true);
    pub const W4: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, true);
    pub const PF4: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, true);
    pub const W5: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, true);
    pub const PF5: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, true);
    pub const W6: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, true);
    pub const PF6: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, true);
    pub const W7: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, true);
    pub const PF7: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, true);
    pub const W8: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, true);
    pub const PF8: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, true);
    pub const W9: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, true);
    pub const PF9: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, true);
    pub const W10: crate::Point<Self, Option<i16>> = crate::Point::new(19, 1, true);
    pub const PF10: crate::Point<Self, Option<i16>> = crate::Point::new(20, 1, true);
    pub const W11: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, true);
    pub const PF11: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, true);
    pub const W12: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, true);
    pub const PF12: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, true);
    pub const W13: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, true);
    pub const PF13: crate::Point<Self, Option<i16>> = crate::Point::new(26, 1, true);
    pub const W14: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, true);
    pub const PF14: crate::Point<Self, Option<i16>> = crate::Point::new(28, 1, true);
    pub const W15: crate::Point<Self, Option<i16>> = crate::Point::new(29, 1, true);
    pub const PF15: crate::Point<Self, Option<i16>> = crate::Point::new(30, 1, true);
    pub const W16: crate::Point<Self, Option<i16>> = crate::Point::new(31, 1, true);
    pub const PF16: crate::Point<Self, Option<i16>> = crate::Point::new(32, 1, true);
    pub const W17: crate::Point<Self, Option<i16>> = crate::Point::new(33, 1, true);
    pub const PF17: crate::Point<Self, Option<i16>> = crate::Point::new(34, 1, true);
    pub const W18: crate::Point<Self, Option<i16>> = crate::Point::new(35, 1, true);
    pub const PF18: crate::Point<Self, Option<i16>> = crate::Point::new(36, 1, true);
    pub const W19: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, true);
    pub const PF19: crate::Point<Self, Option<i16>> = crate::Point::new(38, 1, true);
    pub const W20: crate::Point<Self, Option<i16>> = crate::Point::new(39, 1, true);
    pub const PF20: crate::Point<Self, Option<i16>> = crate::Point::new(40, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(41, 8, true);
    pub const RMP_PT1_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(49, 1, true);
    pub const RMP_DEC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(50, 1, true);
    pub const RMP_INC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(51, 1, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(52, 1, false);
}
impl crate::Group for Curve {
    const LEN: u16 = 54;
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
                w1: Self::W1.from_data(data)?,
                pf1: Self::PF1.from_data(data)?,
                w2: Self::W2.from_data(data)?,
                pf2: Self::PF2.from_data(data)?,
                w3: Self::W3.from_data(data)?,
                pf3: Self::PF3.from_data(data)?,
                w4: Self::W4.from_data(data)?,
                pf4: Self::PF4.from_data(data)?,
                w5: Self::W5.from_data(data)?,
                pf5: Self::PF5.from_data(data)?,
                w6: Self::W6.from_data(data)?,
                pf6: Self::PF6.from_data(data)?,
                w7: Self::W7.from_data(data)?,
                pf7: Self::PF7.from_data(data)?,
                w8: Self::W8.from_data(data)?,
                pf8: Self::PF8.from_data(data)?,
                w9: Self::W9.from_data(data)?,
                pf9: Self::PF9.from_data(data)?,
                w10: Self::W10.from_data(data)?,
                pf10: Self::PF10.from_data(data)?,
                w11: Self::W11.from_data(data)?,
                pf11: Self::PF11.from_data(data)?,
                w12: Self::W12.from_data(data)?,
                pf12: Self::PF12.from_data(data)?,
                w13: Self::W13.from_data(data)?,
                pf13: Self::PF13.from_data(data)?,
                w14: Self::W14.from_data(data)?,
                pf14: Self::PF14.from_data(data)?,
                w15: Self::W15.from_data(data)?,
                pf15: Self::PF15.from_data(data)?,
                w16: Self::W16.from_data(data)?,
                pf16: Self::PF16.from_data(data)?,
                w17: Self::W17.from_data(data)?,
                pf17: Self::PF17.from_data(data)?,
                w18: Self::W18.from_data(data)?,
                pf18: Self::PF18.from_data(data)?,
                w19: Self::W19.from_data(data)?,
                pf19: Self::PF19.from_data(data)?,
                w20: Self::W20.from_data(data)?,
                pf20: Self::PF20.from_data(data)?,
                crv_nam: Self::CRV_NAM.from_data(data)?,
                rmp_pt1_tms: Self::RMP_PT1_TMS.from_data(data)?,
                rmp_dec_tmm: Self::RMP_DEC_TMM.from_data(data)?,
                rmp_inc_tmm: Self::RMP_INC_TMM.from_data(data)?,
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
impl crate::Model for WattPf {
    const ID: u16 = 131;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m131
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
