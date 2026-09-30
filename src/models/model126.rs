//! Static Volt-VAR
/// Type alias for [`VoltVar`].
pub type Model126 = VoltVar;
/// Static Volt-VAR
///
/// Static Volt-VAR Arrays
///
/// Detail: Ref 3: 8.8.1.2
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct VoltVar {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub act_crv: u16,
    /// ModEna
    ///
    /// Is Volt-VAR control active.
    pub mod_ena: ModEna,
    /// WinTms
    ///
    /// Time window for volt-VAR change.
    pub win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    pub rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    pub n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    pub n_pt: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    pub v_sf: i16,
    /// DeptRef_SF
    ///
    /// Scale factor for dependent variable.
    pub dept_ref_sf: i16,
    #[allow(missing_docs)]
    pub rmp_inc_dec_sf: Option<i16>,
    #[allow(missing_docs)]
    pub curve: Vec<Curve>,
}
#[allow(missing_docs)]
impl VoltVar {
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
}
impl crate::sealed::Sealed for VoltVar {}
impl crate::Group for VoltVar {
    const LEN: u16 = 10;
    const GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
        name: "volt_var",
        label: "Static Volt-VAR",
        description: "Static Volt-VAR Arrays ",
        fields: &[
            crate::FieldInfo {
                name: "act_crv",
                label: "ActCrv",
                description: "Index of active curve. 0=no active curve.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "mod_ena",
                label: "ModEna",
                description: "Is Volt-VAR control active.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "win_tms",
                label: "WinTms",
                description: "Time window for volt-VAR change.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rvrt_tms",
                label: "RvrtTms",
                description: "Timeout period for volt-VAR curve selection.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rmp_tms",
                label: "RmpTms",
                description: "The time of the PT1 in seconds (time to accomplish a change of 95%).",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "n_crv",
                label: "NCrv",
                description: "Number of curves supported (recommend 4).",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "n_pt",
                label: "NPt",
                description: "Number of curve points supported (maximum of 20).",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v_sf",
                label: "V_SF",
                description: "Scale factor for percent VRef.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "dept_ref_sf",
                label: "DeptRef_SF",
                description: "Scale factor for dependent variable.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rmp_inc_dec_sf",
                label: "RmpIncDec_SF",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "curve",
                label: "curve",
                description: "",
                kind: crate::FieldKind::RepeatingGroup(&<Curve as crate::Group>::GROUP_INFO),
            },
        ],
    };
}
impl VoltVar {
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
                v_sf: Self::V_SF.from_data(data)?,
                dept_ref_sf: Self::DEPT_REF_SF.from_data(data)?,
                rmp_inc_dec_sf: Self::RMP_INC_DEC_SF.from_data(data)?,
                curve,
            },
        ))
    }
}
bitflags::bitflags! {
    /// ModEna
    ///
    /// Is Volt-VAR control active.
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct ModEna: u16 {
        #[allow(missing_docs)]
        const Enabled = 1;
    }
}
impl crate::sealed::Sealed for ModEna {}
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Curve {
    /// ActPt
    ///
    /// Number of active points in array.
    pub act_pt: u16,
    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    pub dept_ref: CurveDeptRef,
    /// V1
    ///
    /// Point 1 Volts.
    pub v1: u16,
    /// VAr1
    ///
    /// Point 1 VARs.
    pub var1: i16,
    /// V2
    ///
    /// Point 2 Volts.
    pub v2: Option<u16>,
    /// VAr2
    ///
    /// Point 2 VARs.
    pub var2: Option<i16>,
    /// V3
    ///
    /// Point 3 Volts.
    pub v3: Option<u16>,
    /// VAr3
    ///
    /// Point 3 VARs.
    pub var3: Option<i16>,
    /// V4
    ///
    /// Point 4 Volts.
    pub v4: Option<u16>,
    /// VAr4
    ///
    /// Point 4 VARs.
    pub var4: Option<i16>,
    /// V5
    ///
    /// Point 5 Volts.
    pub v5: Option<u16>,
    /// VAr5
    ///
    /// Point 5 VARs.
    pub var5: Option<i16>,
    /// V6
    ///
    /// Point 6 Volts.
    pub v6: Option<u16>,
    /// VAr6
    ///
    /// Point 6 VARs.
    pub var6: Option<i16>,
    /// V7
    ///
    /// Point 7 Volts.
    pub v7: Option<u16>,
    /// VAr7
    ///
    /// Point 7 VARs.
    pub var7: Option<i16>,
    /// V8
    ///
    /// Point 8 Volts.
    pub v8: Option<u16>,
    /// VAr8
    ///
    /// Point 8 VARs.
    pub var8: Option<i16>,
    /// V9
    ///
    /// Point 9 Volts.
    pub v9: Option<u16>,
    /// VAr9
    ///
    /// Point 9 VARs.
    pub var9: Option<i16>,
    /// V10
    ///
    /// Point 10 Volts.
    pub v10: Option<u16>,
    /// VAr10
    ///
    /// Point 10 VARs.
    pub var10: Option<i16>,
    /// V11
    ///
    /// Point 11 Volts.
    pub v11: Option<u16>,
    /// VAr11
    ///
    /// Point 11 VARs.
    pub var11: Option<i16>,
    /// V12
    ///
    /// Point 12 Volts.
    pub v12: Option<u16>,
    /// VAr12
    ///
    /// Point 12 VARs.
    pub var12: Option<i16>,
    /// V13
    ///
    /// Point 13 Volts.
    pub v13: Option<u16>,
    /// VAr13
    ///
    /// Point 13 VARs.
    pub var13: Option<i16>,
    /// V14
    ///
    /// Point 14 Volts.
    pub v14: Option<u16>,
    /// VAr14
    ///
    /// Point 14 VARs.
    pub var14: Option<i16>,
    /// V15
    ///
    /// Point 15 Volts.
    pub v15: Option<u16>,
    /// VAr15
    ///
    /// Point 15 VARs.
    pub var15: Option<i16>,
    /// V16
    ///
    /// Point 16 Volts.
    pub v16: Option<u16>,
    /// VAr16
    ///
    /// Point 16 VARs.
    pub var16: Option<i16>,
    /// V17
    ///
    /// Point 17 Volts.
    pub v17: Option<u16>,
    /// VAr17
    ///
    /// Point 17 VARs.
    pub var17: Option<i16>,
    /// V18
    ///
    /// Point 18 Volts.
    pub v18: Option<u16>,
    /// VAr18
    ///
    /// Point 18 VARs.
    pub var18: Option<i16>,
    /// V19
    ///
    /// Point 19 Volts.
    pub v19: Option<u16>,
    /// VAr19
    ///
    /// Point 19 VARs.
    pub var19: Option<i16>,
    /// V20
    ///
    /// Point 20 Volts.
    pub v20: Option<u16>,
    /// VAr20
    ///
    /// Point 20 VARs.
    pub var20: Option<i16>,
    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    pub crv_nam: Option<String>,
    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    pub rmp_tms: Option<u16>,
    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    pub rmp_dec_tmm: Option<u16>,
    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    pub rmp_inc_tmm: Option<u16>,
    /// ReadOnly
    ///
    /// Boolean flag indicates if curve is read-only or can be modified.
    pub read_only: CurveReadOnly,
}
#[allow(missing_docs)]
impl Curve {
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const DEPT_REF: crate::Point<Self, CurveDeptRef> = crate::Point::new(1, 1, true);
    pub const V1: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const VAR1: crate::Point<Self, i16> = crate::Point::new(3, 1, true);
    pub const V2: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const VAR2: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, true);
    pub const V3: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const VAR3: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, true);
    pub const V4: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const VAR4: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, true);
    pub const V5: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const VAR5: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, true);
    pub const V6: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const VAR6: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, true);
    pub const V7: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const VAR7: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, true);
    pub const V8: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, true);
    pub const VAR8: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, true);
    pub const V9: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const VAR9: crate::Point<Self, Option<i16>> = crate::Point::new(19, 1, true);
    pub const V10: crate::Point<Self, Option<u16>> = crate::Point::new(20, 1, true);
    pub const VAR10: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, true);
    pub const V11: crate::Point<Self, Option<u16>> = crate::Point::new(22, 1, true);
    pub const VAR11: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, true);
    pub const V12: crate::Point<Self, Option<u16>> = crate::Point::new(24, 1, true);
    pub const VAR12: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, true);
    pub const V13: crate::Point<Self, Option<u16>> = crate::Point::new(26, 1, true);
    pub const VAR13: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, true);
    pub const V14: crate::Point<Self, Option<u16>> = crate::Point::new(28, 1, true);
    pub const VAR14: crate::Point<Self, Option<i16>> = crate::Point::new(29, 1, true);
    pub const V15: crate::Point<Self, Option<u16>> = crate::Point::new(30, 1, true);
    pub const VAR15: crate::Point<Self, Option<i16>> = crate::Point::new(31, 1, true);
    pub const V16: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, true);
    pub const VAR16: crate::Point<Self, Option<i16>> = crate::Point::new(33, 1, true);
    pub const V17: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const VAR17: crate::Point<Self, Option<i16>> = crate::Point::new(35, 1, true);
    pub const V18: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, true);
    pub const VAR18: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, true);
    pub const V19: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, true);
    pub const VAR19: crate::Point<Self, Option<i16>> = crate::Point::new(39, 1, true);
    pub const V20: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, true);
    pub const VAR20: crate::Point<Self, Option<i16>> = crate::Point::new(41, 1, true);
    pub const CRV_NAM: crate::Point<Self, Option<String>> = crate::Point::new(42, 8, true);
    pub const RMP_TMS: crate::Point<Self, Option<u16>> = crate::Point::new(50, 1, true);
    pub const RMP_DEC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(51, 1, true);
    pub const RMP_INC_TMM: crate::Point<Self, Option<u16>> = crate::Point::new(52, 1, true);
    pub const READ_ONLY: crate::Point<Self, CurveReadOnly> = crate::Point::new(53, 1, false);
}
impl crate::sealed::Sealed for Curve {}
impl crate::Group for Curve {
    const LEN: u16 = 54;
    const GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
        name: "curve",
        label: "curve",
        description: "",
        fields: &[
            crate::FieldInfo {
                name: "act_pt",
                label: "ActPt",
                description: "Number of active points in array.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "dept_ref",
                label: "DeptRef",
                description: "Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v1",
                label: "V1",
                description: "Point 1 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var1",
                label: "VAr1",
                description: "Point 1 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v2",
                label: "V2",
                description: "Point 2 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var2",
                label: "VAr2",
                description: "Point 2 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v3",
                label: "V3",
                description: "Point 3 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var3",
                label: "VAr3",
                description: "Point 3 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v4",
                label: "V4",
                description: "Point 4 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var4",
                label: "VAr4",
                description: "Point 4 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v5",
                label: "V5",
                description: "Point 5 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var5",
                label: "VAr5",
                description: "Point 5 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v6",
                label: "V6",
                description: "Point 6 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var6",
                label: "VAr6",
                description: "Point 6 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v7",
                label: "V7",
                description: "Point 7 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var7",
                label: "VAr7",
                description: "Point 7 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v8",
                label: "V8",
                description: "Point 8 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var8",
                label: "VAr8",
                description: "Point 8 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v9",
                label: "V9",
                description: "Point 9 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var9",
                label: "VAr9",
                description: "Point 9 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v10",
                label: "V10",
                description: "Point 10 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var10",
                label: "VAr10",
                description: "Point 10 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v11",
                label: "V11",
                description: "Point 11 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var11",
                label: "VAr11",
                description: "Point 11 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v12",
                label: "V12",
                description: "Point 12 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var12",
                label: "VAr12",
                description: "Point 12 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v13",
                label: "V13",
                description: "Point 13 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var13",
                label: "VAr13",
                description: "Point 13 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v14",
                label: "V14",
                description: "Point 14 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var14",
                label: "VAr14",
                description: "Point 14 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v15",
                label: "V15",
                description: "Point 15 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var15",
                label: "VAr15",
                description: "Point 15 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v16",
                label: "V16",
                description: "Point 16 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var16",
                label: "VAr16",
                description: "Point 16 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v17",
                label: "V17",
                description: "Point 17 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var17",
                label: "VAr17",
                description: "Point 17 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v18",
                label: "V18",
                description: "Point 18 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var18",
                label: "VAr18",
                description: "Point 18 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v19",
                label: "V19",
                description: "Point 19 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var19",
                label: "VAr19",
                description: "Point 19 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "v20",
                label: "V20",
                description: "Point 20 Volts.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var20",
                label: "VAr20",
                description: "Point 20 VARs.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "crv_nam",
                label: "CrvNam",
                description: "Optional description for curve. (Max 16 chars)",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rmp_tms",
                label: "RmpTms",
                description: "The time of the PT1 in seconds (time to accomplish a change of 95%).",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rmp_dec_tmm",
                label: "RmpDecTmm",
                description: "The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "rmp_inc_tmm",
                label: "RmpIncTmm",
                description: "The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "read_only",
                label: "ReadOnly",
                description: "Boolean flag indicates if curve is read-only or can be modified.",
                kind: crate::FieldKind::Point,
            },
        ],
    };
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
                dept_ref: Self::DEPT_REF.from_data(data)?,
                v1: Self::V1.from_data(data)?,
                var1: Self::VAR1.from_data(data)?,
                v2: Self::V2.from_data(data)?,
                var2: Self::VAR2.from_data(data)?,
                v3: Self::V3.from_data(data)?,
                var3: Self::VAR3.from_data(data)?,
                v4: Self::V4.from_data(data)?,
                var4: Self::VAR4.from_data(data)?,
                v5: Self::V5.from_data(data)?,
                var5: Self::VAR5.from_data(data)?,
                v6: Self::V6.from_data(data)?,
                var6: Self::VAR6.from_data(data)?,
                v7: Self::V7.from_data(data)?,
                var7: Self::VAR7.from_data(data)?,
                v8: Self::V8.from_data(data)?,
                var8: Self::VAR8.from_data(data)?,
                v9: Self::V9.from_data(data)?,
                var9: Self::VAR9.from_data(data)?,
                v10: Self::V10.from_data(data)?,
                var10: Self::VAR10.from_data(data)?,
                v11: Self::V11.from_data(data)?,
                var11: Self::VAR11.from_data(data)?,
                v12: Self::V12.from_data(data)?,
                var12: Self::VAR12.from_data(data)?,
                v13: Self::V13.from_data(data)?,
                var13: Self::VAR13.from_data(data)?,
                v14: Self::V14.from_data(data)?,
                var14: Self::VAR14.from_data(data)?,
                v15: Self::V15.from_data(data)?,
                var15: Self::VAR15.from_data(data)?,
                v16: Self::V16.from_data(data)?,
                var16: Self::VAR16.from_data(data)?,
                v17: Self::V17.from_data(data)?,
                var17: Self::VAR17.from_data(data)?,
                v18: Self::V18.from_data(data)?,
                var18: Self::VAR18.from_data(data)?,
                v19: Self::V19.from_data(data)?,
                var19: Self::VAR19.from_data(data)?,
                v20: Self::V20.from_data(data)?,
                var20: Self::VAR20.from_data(data)?,
                crv_nam: Self::CRV_NAM.from_data(data)?,
                rmp_tms: Self::RMP_TMS.from_data(data)?,
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
/// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CurveDeptRef {
    #[allow(missing_docs)]
    WMax,
    #[allow(missing_docs)]
    VarMax,
    #[allow(missing_docs)]
    VarAval,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::sealed::Sealed for CurveDeptRef {}
impl crate::EnumValue for CurveDeptRef {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::WMax,
            2 => Self::VarMax,
            3 => Self::VarAval,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::WMax => 1,
            Self::VarMax => 2,
            Self::VarAval => 3,
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
/// Boolean flag indicates if curve is read-only or can be modified.
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
impl crate::sealed::Sealed for CurveReadOnly {}
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
impl From<VoltVar> for crate::AnyModel {
    fn from(model: VoltVar) -> Self {
        Self::M126(model)
    }
}
impl crate::Model for VoltVar {
    const ID: u16 = 126;
    const NAME: &'static str = "volt_var";
    const LABEL: &'static str = "Static Volt-VAR";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m126
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
