//! Volt-Watt
/// Type alias for [`VoltWatt`].
pub type Model132 = VoltWatt;
/// Volt-Watt
///
/// Volt-Watt
///
/// Detail: Ref 3: 8.12.1.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct VoltWatt {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is Volt-Watt control active.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for volt-watt change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
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
    /// Number of points in array (maximum 20).
    pub n_pt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// Scale Factor for % DeptRef
    pub dept_ref_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmp_inc_dec_sf: Option<i16>,
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl VoltWatt {
    pub const ACT_CRV: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const MOD_ENA: crate::Point<Self, ModEna> = crate::Point::new(1, 1, true);
    pub const WIN_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const RVRT_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const N_CRV: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const DEPT_REF_SF: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const RMP_INC_DEC_SF: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::ACT_CRV.is_invalid(&self.act_crv)
            || Self::MOD_ENA.is_invalid(&self.mod_ena)
            || Self::N_CRV.is_invalid(&self.n_crv)
            || Self::N_PT.is_invalid(&self.n_pt)
            || Self::V_SF.is_invalid(&self.v_sf)
            || Self::DEPT_REF_SF.is_invalid(&self.dept_ref_sf)
            || self.curve.iter().any(|group| group.has_invalid_points())
    }
}
impl crate::Group for VoltWatt {
    const LEN: u16 = 10;
}
impl VoltWatt {
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
                v_sf: Self::V_SF.from_data(data)?,
                dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
                rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
                curve,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " ModEna"] #[doc = " "] #[doc = " Is Volt-Watt control active."]
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
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Curve {
    /// ActPt
    ///
    /// Number of active points in array.
    pub act_pt: u16,
    /// DeptRef
    ///
    /// Defines the meaning of the Watts DeptRef.  1=% WMax 2=% WAvail
    pub dept_ref: CurveDeptRef,
    /// V1
    ///
    /// Point 1 Volts.
    pub v1: u16,
    /// W1
    ///
    /// Point 1 Watts.
    pub w1: i16,
    /// V2
    ///
    /// Point 2 Volts.
    pub v2: Option<u16>,
    /// W2
    ///
    /// Point 2 Watts.
    pub w2: Option<i16>,
    /// V3
    ///
    /// Point 3 Volts.
    pub v3: Option<u16>,
    /// W3
    ///
    /// Point 3 Watts.
    pub w3: Option<i16>,
    /// V4
    ///
    /// Point 4 Volts.
    pub v4: Option<u16>,
    /// W4
    ///
    /// Point 4 Watts.
    pub w4: Option<i16>,
    /// V5
    ///
    /// Point 5 Volts.
    pub v5: Option<u16>,
    /// W5
    ///
    /// Point 5 Watts.
    pub w5: Option<i16>,
    /// V6
    ///
    /// Point 6 Volts.
    pub v6: Option<u16>,
    /// W6
    ///
    /// Point 6 Watts.
    pub w6: Option<i16>,
    /// V7
    ///
    /// Point 7 Volts.
    pub v7: Option<u16>,
    /// W7
    ///
    /// Point 7 Watts.
    pub w7: Option<i16>,
    /// V8
    ///
    /// Point 8 Volts.
    pub v8: Option<u16>,
    /// W8
    ///
    /// Point 8 Watts.
    pub w8: Option<i16>,
    /// V9
    ///
    /// Point 9 Volts.
    pub v9: Option<u16>,
    /// W9
    ///
    /// Point 9 Watts.
    pub w9: Option<i16>,
    /// V10
    ///
    /// Point 10 Volts.
    pub v10: Option<u16>,
    /// W10
    ///
    /// Point 10 Watts.
    pub w10: Option<i16>,
    /// V11
    ///
    /// Point 11 Volts.
    pub v11: Option<u16>,
    /// W11
    ///
    /// Point 11 Watts.
    pub w11: Option<i16>,
    /// V12
    ///
    /// Point 12 Volts.
    pub v12: Option<u16>,
    /// W12
    ///
    /// Point 12 Watts.
    pub w12: Option<i16>,
    /// V13
    ///
    /// Point 13 Volts.
    pub v13: Option<u16>,
    /// W13
    ///
    /// Point 13 Watts.
    pub w13: Option<i16>,
    /// V14
    ///
    /// Point 14 Volts.
    pub v14: Option<u16>,
    /// W14
    ///
    /// Point 14 Watts.
    pub w14: Option<i16>,
    /// V15
    ///
    /// Point 15 Volts.
    pub v15: Option<u16>,
    /// W15
    ///
    /// Point 15 Watts.
    pub w15: Option<i16>,
    /// V16
    ///
    /// Point 16 Volts.
    pub v16: Option<u16>,
    /// W16
    ///
    /// Point 16 Watts.
    pub w16: Option<i16>,
    /// V17
    ///
    /// Point 17 Volts.
    pub v17: Option<u16>,
    /// W17
    ///
    /// Point 17 Watts.
    pub w17: Option<i16>,
    /// V18
    ///
    /// Point 18 Volts.
    pub v18: Option<u16>,
    /// W18
    ///
    /// Point 18 Watts.
    pub w18: Option<i16>,
    /// V19
    ///
    /// Point 19 Volts.
    pub v19: Option<u16>,
    /// W19
    ///
    /// Point 19 Watts.
    pub w19: Option<i16>,
    /// V20
    ///
    /// Point 20 Volts.
    pub v20: Option<u16>,
    /// W20
    ///
    /// Point 20 Watts.
    pub w20: Option<i16>,
    /// CrvNam
    ///
    /// Optional description for curve.
    pub crv_nam: Option<String>,
    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_pt1_tms: Option<u16>,
    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    pub rmp_dec_tmm: Option<u16>,
    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    pub rmp_inc_tmm: Option<u16>,
    /// ReadOnly
    ///
    /// Enumerated value indicates if curve is read-only or can be modified.
    pub read_only: CurveReadOnly,
}
#[allow(missing_docs)]
impl Curve {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, CurveDeptRef> = crate::Point::new(1, 1, true);
    pub const V1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const W1: crate::Point<Self, i16> = crate::Point::new(3, 1, true);
    pub const V2: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const W2: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, true);
    pub const V3: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const W3: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, true);
    pub const V4: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const W4: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, true);
    pub const V5: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const W5: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, true);
    pub const V6: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const W6: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, true);
    pub const V7: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const W7: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, true);
    pub const V8: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, true);
    pub const W8: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, true);
    pub const V9: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const W9: crate::Point<Self, Option<i16>> = crate::Point::new(19, 1, true);
    pub const V10: crate::Point<Self, Option<u16>> = crate::Point::new(20, 1, true);
    pub const W10: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, true);
    pub const V11: crate::Point<Self, Option<u16>> = crate::Point::new(22, 1, true);
    pub const W11: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, true);
    pub const V12: crate::Point<Self, Option<u16>> = crate::Point::new(24, 1, true);
    pub const W12: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, true);
    pub const V13: crate::Point<Self, Option<u16>> = crate::Point::new(26, 1, true);
    pub const W13: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, true);
    pub const V14: crate::Point<Self, Option<u16>> = crate::Point::new(28, 1, true);
    pub const W14: crate::Point<Self, Option<i16>> = crate::Point::new(29, 1, true);
    pub const V15: crate::Point<Self, Option<u16>> = crate::Point::new(30, 1, true);
    pub const W15: crate::Point<Self, Option<i16>> = crate::Point::new(31, 1, true);
    pub const V16: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, true);
    pub const W16: crate::Point<Self, Option<i16>> = crate::Point::new(33, 1, true);
    pub const V17: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const W17: crate::Point<Self, Option<i16>> = crate::Point::new(35, 1, true);
    pub const V18: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, true);
    pub const W18: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, true);
    pub const V19: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, true);
    pub const W19: crate::Point<Self, Option<i16>> = crate::Point::new(39, 1, true);
    pub const V20: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, true);
    pub const W20: crate::Point<Self, Option<i16>> = crate::Point::new(41, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(42, 8, true);
    pub const RMP_PT1_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(50, 1, true);
    pub const RMP_DEC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(51, 1, true);
    pub const RMP_INC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(52, 1, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(53, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::ACT_PT.is_invalid(&self.act_pt)
            || Self::DEPT_REF.is_invalid(&self.dept_ref)
            || Self::V1.is_invalid(&self.v1)
            || Self::W1.is_invalid(&self.w1)
            || Self::READ_ONLY.is_invalid(&self.read_only)
    }
}
impl crate::Group for Curve {
    const LEN: u16 = 54;
}
impl Curve {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                act_pt: Self::ACT_PT.from_data(data)?,
                dept_ref: Self::DEPT_REF.from_data(data)?,
                v1: Self::V1.from_data(data)?,
                w1: Self::W1.from_data(data)?,
                v2: Self::V2.from_data(data)?,
                w2: Self::W2.from_data(data)?,
                v3: Self::V3.from_data(data)?,
                w3: Self::W3.from_data(data)?,
                v4: Self::V4.from_data(data)?,
                w4: Self::W4.from_data(data)?,
                v5: Self::V5.from_data(data)?,
                w5: Self::W5.from_data(data)?,
                v6: Self::V6.from_data(data)?,
                w6: Self::W6.from_data(data)?,
                v7: Self::V7.from_data(data)?,
                w7: Self::W7.from_data(data)?,
                v8: Self::V8.from_data(data)?,
                w8: Self::W8.from_data(data)?,
                v9: Self::V9.from_data(data)?,
                w9: Self::W9.from_data(data)?,
                v10: Self::V10.from_data(data)?,
                w10: Self::W10.from_data(data)?,
                v11: Self::V11.from_data(data)?,
                w11: Self::W11.from_data(data)?,
                v12: Self::V12.from_data(data)?,
                w12: Self::W12.from_data(data)?,
                v13: Self::V13.from_data(data)?,
                w13: Self::W13.from_data(data)?,
                v14: Self::V14.from_data(data)?,
                w14: Self::W14.from_data(data)?,
                v15: Self::V15.from_data(data)?,
                w15: Self::W15.from_data(data)?,
                v16: Self::V16.from_data(data)?,
                w16: Self::W16.from_data(data)?,
                v17: Self::V17.from_data(data)?,
                w17: Self::W17.from_data(data)?,
                v18: Self::V18.from_data(data)?,
                w18: Self::W18.from_data(data)?,
                v19: Self::V19.from_data(data)?,
                w19: Self::W19.from_data(data)?,
                v20: Self::V20.from_data(data)?,
                w20: Self::W20.from_data(data)?,
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
/// DeptRef
///
/// Defines the meaning of the Watts DeptRef.  1=% WMax 2=% WAvail
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CurveDeptRef {
    #[allow(missing_docs)]
    WMax,
    #[allow(missing_docs)]
    WAval,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CurveDeptRef {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::WMax,
            2 => Self::WAval,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMax => 1,
            Self::WAval => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CurveDeptRef {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
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
impl crate::Model for VoltWatt {
    const ID: u16 = 132;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m132
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
