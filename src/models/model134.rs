//! Freq-Watt Crv
/// Type alias for [`FreqWatt`].
pub type Model134 = FreqWatt;
/// Freq-Watt Crv
///
/// Curve-Based Frequency-Watt
///
/// Detail: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FreqWatt {
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
    /// Number of curve points supported (maximum of 20).
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
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl FreqWatt {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const HZ_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
}
impl crate::Group for FreqWatt {
    const LEN: u16 = 10;
}
impl FreqWatt {
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
                hz_sf: Self::HZ_SF.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
                curve,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc =
    " Is curve-based Frequency-Watt control active."] #[derive(Copy, Clone, Debug, Eq,
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
    /// Hz1
    ///
    /// Point 1 Hertz.
    pub hz1: u16,
    /// W1
    ///
    /// Point 1 Watts.
    pub w1: i16,
    /// Hz2
    ///
    /// Point 2 Hertz.
    pub hz2: Option<u16>,
    /// W2
    ///
    /// Point 2 Watts.
    pub w2: Option<i16>,
    /// Hz3
    ///
    /// Point 3 Hertz.
    pub hz3: Option<u16>,
    /// W3
    ///
    /// Point 3 Watts.
    pub w3: Option<i16>,
    /// Hz4
    ///
    /// Point 4 Hertz.
    pub hz4: Option<u16>,
    /// W4
    ///
    /// Point 4 Watts.
    pub w4: Option<i16>,
    /// Hz5
    ///
    /// Point 5 Hertz.
    pub hz5: Option<u16>,
    /// W5
    ///
    /// Point 5 Watts.
    pub w5: Option<i16>,
    /// Hz6
    ///
    /// Point 6 Hertz.
    pub hz6: Option<u16>,
    /// W6
    ///
    /// Point 6 Watts.
    pub w6: Option<i16>,
    /// Hz7
    ///
    /// Point 7 Hertz.
    pub hz7: Option<u16>,
    /// W7
    ///
    /// Point 7 Watts.
    pub w7: Option<i16>,
    /// Hz8
    ///
    /// Point 8 Hertz.
    pub hz8: Option<u16>,
    /// W8
    ///
    /// Point 8 Watts.
    pub w8: Option<i16>,
    /// Hz9
    ///
    /// Point 9 Hertz.
    pub hz9: Option<u16>,
    /// W9
    ///
    /// Point 9 Watts.
    pub w9: Option<i16>,
    /// Hz10
    ///
    /// Point 10 Hertz.
    pub hz10: Option<u16>,
    /// W10
    ///
    /// Point 10 Watts.
    pub w10: Option<i16>,
    /// Hz11
    ///
    /// Point 11 Hertz.
    pub hz11: Option<u16>,
    /// W11
    ///
    /// Point 11 Watts.
    pub w11: Option<i16>,
    /// Hz12
    ///
    /// Point 12 Hertz.
    pub hz12: Option<u16>,
    /// W12
    ///
    /// Point 12 Watts.
    pub w12: Option<i16>,
    /// Hz13
    ///
    /// Point 13 Hertz.
    pub hz13: Option<u16>,
    /// W13
    ///
    /// Point 13 Watts.
    pub w13: Option<i16>,
    /// Hz14
    ///
    /// Point 14 Hertz.
    pub hz14: Option<u16>,
    /// W14
    ///
    /// Point 14 Watts.
    pub w14: Option<i16>,
    /// Hz15
    ///
    /// Point 15 Hertz.
    pub hz15: Option<u16>,
    /// W15
    ///
    /// Point 15 Watts.
    pub w15: Option<i16>,
    /// Hz16
    ///
    /// Point 16 Hertz.
    pub hz16: Option<u16>,
    /// W16
    ///
    /// Point 16 Watts.
    pub w16: Option<i16>,
    /// Hz17
    ///
    /// Point 17 Hertz.
    pub hz17: Option<u16>,
    /// W17
    ///
    /// Point 17 Watts.
    pub w17: Option<i16>,
    /// Hz18
    ///
    /// Point 18 Hertz.
    pub hz18: Option<u16>,
    /// W18
    ///
    /// Point 18 Watts.
    pub w18: Option<i16>,
    /// Hz19
    ///
    /// Point 19 Hertz.
    pub hz19: Option<u16>,
    /// W19
    ///
    /// Point 19 Watts.
    pub w19: Option<i16>,
    /// Hz20
    ///
    /// Point 20 Hertz.
    pub hz20: Option<u16>,
    /// W20
    ///
    /// Point 20 Watts.
    pub w20: Option<i16>,
    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    pub crv_nam: Option<String>,
    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_pt1_tms: Option<u16>,
    /// RmpDecTmm
    ///
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    pub rmp_dec_tmm: Option<u16>,
    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    pub rmp_inc_tmm: Option<u16>,
    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    pub rmp_rs_up: Option<u16>,
    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    pub snpt_w: CurveSnptW,
    /// WRef
    ///
    /// Reference active power (default = WMax).
    pub w_ref: Option<u16>,
    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    pub w_ref_str_hz: Option<u16>,
    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    pub w_ref_stop_hz: Option<u16>,
    /// ReadOnly
    ///
    /// Enumerated value indicates if curve is read-only or can be modified.
    pub read_only: CurveReadOnly,
}
#[allow(missing_docs)]
impl Curve {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const HZ1: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const W1: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const HZ2: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const W2: crate::Point<Self, Option<i16>> = crate::Point::new(4, 1, true);
    pub const HZ3: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const W3: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, true);
    pub const HZ4: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const W4: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, true);
    pub const HZ5: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const W5: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, true);
    pub const HZ6: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const W6: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, true);
    pub const HZ7: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const W7: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, true);
    pub const HZ8: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, true);
    pub const W8: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, true);
    pub const HZ9: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const W9: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, true);
    pub const HZ10: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, true);
    pub const W10: crate::Point<Self, Option<i16>> = crate::Point::new(20, 1, true);
    pub const HZ11: crate::Point<Self, Option<u16>> = crate::Point::new(21, 1, true);
    pub const W11: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, true);
    pub const HZ12: crate::Point<Self, Option<u16>> = crate::Point::new(23, 1, true);
    pub const W12: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, true);
    pub const HZ13: crate::Point<Self, Option<u16>> = crate::Point::new(25, 1, true);
    pub const W13: crate::Point<Self, Option<i16>> = crate::Point::new(26, 1, true);
    pub const HZ14: crate::Point<Self, Option<u16>> = crate::Point::new(27, 1, true);
    pub const W14: crate::Point<Self, Option<i16>> = crate::Point::new(28, 1, true);
    pub const HZ15: crate::Point<Self, Option<u16>> = crate::Point::new(29, 1, true);
    pub const W15: crate::Point<Self, Option<i16>> = crate::Point::new(30, 1, true);
    pub const HZ16: crate::Point<Self, Option<u16>> = crate::Point::new(31, 1, true);
    pub const W16: crate::Point<Self, Option<i16>> = crate::Point::new(32, 1, true);
    pub const HZ17: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, true);
    pub const W17: crate::Point<Self, Option<i16>> = crate::Point::new(34, 1, true);
    pub const HZ18: crate::Point<Self, Option<u16>> = crate::Point::new(35, 1, true);
    pub const W18: crate::Point<Self, Option<i16>> = crate::Point::new(36, 1, true);
    pub const HZ19: crate::Point<Self, Option<u16>> = crate::Point::new(37, 1, true);
    pub const W19: crate::Point<Self, Option<i16>> = crate::Point::new(38, 1, true);
    pub const HZ20: crate::Point<Self, Option<u16>> = crate::Point::new(39, 1, true);
    pub const W20: crate::Point<Self, Option<i16>> = crate::Point::new(40, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(41, 8, true);
    pub const RMP_PT1_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(49, 1, true);
    pub const RMP_DEC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(50, 1, true);
    pub const RMP_INC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(51, 1, true);
    pub const RMP_RS_UP: crate::Point<Self, Option<u16>> = crate::Point::new(52, 1, true);
    pub const SNPT_W: crate::Point<Self, CurveSnptW> = crate::Point::new(53, 1, true);
    pub const W_REF: crate::Point<Self, Option<u16>> = crate::Point::new(54, 1, true);
    pub const W_REF_STR_HZ: crate::Point<Self, Option<u16>> = crate::Point::new(55, 1, true);
    pub const W_REF_STOP_HZ: crate::Point<Self, Option<u16>> = crate::Point::new(56, 1, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(57, 1, false);
}
impl crate::Group for Curve {
    const LEN: u16 = 58;
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
                hz1: Self::HZ1.from_data(data)?,
                w1: Self::W1.from_data(data)?,
                hz2: Self::HZ2.from_data(data)?,
                w2: Self::W2.from_data(data)?,
                hz3: Self::HZ3.from_data(data)?,
                w3: Self::W3.from_data(data)?,
                hz4: Self::HZ4.from_data(data)?,
                w4: Self::W4.from_data(data)?,
                hz5: Self::HZ5.from_data(data)?,
                w5: Self::W5.from_data(data)?,
                hz6: Self::HZ6.from_data(data)?,
                w6: Self::W6.from_data(data)?,
                hz7: Self::HZ7.from_data(data)?,
                w7: Self::W7.from_data(data)?,
                hz8: Self::HZ8.from_data(data)?,
                w8: Self::W8.from_data(data)?,
                hz9: Self::HZ9.from_data(data)?,
                w9: Self::W9.from_data(data)?,
                hz10: Self::HZ10.from_data(data)?,
                w10: Self::W10.from_data(data)?,
                hz11: Self::HZ11.from_data(data)?,
                w11: Self::W11.from_data(data)?,
                hz12: Self::HZ12.from_data(data)?,
                w12: Self::W12.from_data(data)?,
                hz13: Self::HZ13.from_data(data)?,
                w13: Self::W13.from_data(data)?,
                hz14: Self::HZ14.from_data(data)?,
                w14: Self::W14.from_data(data)?,
                hz15: Self::HZ15.from_data(data)?,
                w15: Self::W15.from_data(data)?,
                hz16: Self::HZ16.from_data(data)?,
                w16: Self::W16.from_data(data)?,
                hz17: Self::HZ17.from_data(data)?,
                w17: Self::W17.from_data(data)?,
                hz18: Self::HZ18.from_data(data)?,
                w18: Self::W18.from_data(data)?,
                hz19: Self::HZ19.from_data(data)?,
                w19: Self::W19.from_data(data)?,
                hz20: Self::HZ20.from_data(data)?,
                w20: Self::W20.from_data(data)?,
                crv_nam: Self::CRV_NAM.from_data(data)?,
                rmp_pt1_tms: Self::RMP_PT1_TMS.from_data(data)?,
                rmp_dec_tmm: Self::RMP_DEC_TMM.from_data(data)?,
                rmp_inc_tmm: Self::RMP_INC_TMM.from_data(data)?,
                rmp_rs_up: Self::RMP_RS_UP.from_data(data)?,
                snpt_w: Self::SNPT_W.from_data(data)?,
                w_ref: Self::W_REF.from_data(data)?,
                w_ref_str_hz: Self::W_REF_STR_HZ.from_data(data)?,
                w_ref_stop_hz: Self::W_REF_STOP_HZ.from_data(data)?,
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
bitflags::bitflags! {
    #[doc = " SnptW"] #[doc = " "] #[doc = " 1=enable snapshot/capture mode"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct CurveSnptW : u16 {}
}
impl crate::Value for CurveSnptW {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for CurveSnptW {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
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
impl crate::Model for FreqWatt {
    const ID: u16 = 134;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m134
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
