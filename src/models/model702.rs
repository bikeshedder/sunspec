//! DER Capacity
/// DER Capacity
///
/// DER capacity model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model702 {
    /// Active Power Max Rating
    ///
    /// Maximum active power rating at unity power factor in watts.
    ///
    /// Comments: Nameplate Ratings - Specifies capacity ratings
    pub w_max_rtg: Option<u16>,
    /// Active Power (Over-Excited) Rating
    ///
    /// Active power rating at specified over-excited power factor in watts.
    pub w_ovr_ext_rtg: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    pub w_ovr_ext_rtg_pf: Option<u16>,
    /// Active Power (Under-Excited) Rating
    ///
    /// Active power rating at specified under-excited power factor in watts.
    pub w_und_ext_rtg: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    pub w_und_ext_rtg_pf: Option<u16>,
    /// Apparent Power Max Rating
    ///
    /// Maximum apparent power rating in voltamperes.
    pub va_max_rtg: Option<u16>,
    /// Reactive Power Injected Rating
    ///
    /// Maximum injected reactive power rating in vars.
    pub var_max_inj_rtg: Option<u16>,
    /// Reactive Power Absorbed Rating
    ///
    /// Maximum absorbed reactive power rating in vars.
    pub var_max_abs_rtg: Option<u16>,
    /// Charge Rate Max Rating
    ///
    /// Maximum active power charge rate in watts.
    pub w_cha_rte_max_rtg: Option<u16>,
    /// Discharge Rate Max Rating
    ///
    /// Maximum active power discharge rate in watts.
    pub w_dis_cha_rte_max_rtg: Option<u16>,
    /// Charge Rate Max VA Rating
    ///
    /// Maximum apparent power charge rate in voltamperes.
    pub va_cha_rte_max_rtg: Option<u16>,
    /// Discharge Rate Max VA Rating
    ///
    /// Maximum apparent power discharge rate in voltamperes.
    pub va_dis_cha_rte_max_rtg: Option<u16>,
    /// AC Voltage Nominal Rating
    ///
    /// AC voltage nominal rating.
    pub v_nom_rtg: Option<u16>,
    /// AC Voltage Max Rating
    ///
    /// AC voltage maximum rating.
    pub v_max_rtg: Option<u16>,
    /// AC Voltage Min Rating
    ///
    /// AC voltage minimum rating.
    pub v_min_rtg: Option<u16>,
    /// AC Current Max Rating
    ///
    /// AC current maximum rating in amps.
    pub a_max_rtg: Option<u16>,
    /// PF Over-Excited Rating
    ///
    /// Power factor over-excited rating.
    pub pf_ovr_ext_rtg: Option<u16>,
    /// PF Under-Excited Rating
    ///
    /// Power factor under-excited rating.
    pub pf_und_ext_rtg: Option<u16>,
    /// Reactive Susceptance
    ///
    /// Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state.
    pub react_suscept_rtg: Option<u16>,
    /// Normal Operating Category
    ///
    /// Normal operating performance category as specified in IEEE 1547-2018.
    pub nor_op_cat_rtg: Option<NorOpCatRtg>,
    /// Abnormal Operating Category
    ///
    /// Abnormal operating performance category as specified in IEEE 1547-2018.
    pub abn_op_cat_rtg: Option<AbnOpCatRtg>,
    /// Supported Control Modes
    ///
    /// Supported control mode functions.
    pub ctrl_modes: Option<CtrlModes>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    pub int_island_cat_rtg: Option<IntIslandCatRtg>,
    /// Active Power Max Setting
    ///
    /// Maximum active power setting used to adjust maximum active power setting.
    ///
    /// Comments: Settings - Used to adjust nameplate ratings
    pub w_max: Option<u16>,
    /// Active Power (Over-Excited) Setting
    ///
    /// Active power setting at specified over-excited power factor in watts.
    pub w_max_ovr_ext: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    pub w_ovr_ext_pf: Option<u16>,
    /// Active Power (Under-Excited) Setting
    ///
    /// Active power setting at specified under-excited power factor in watts.
    pub w_max_und_ext: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    pub w_und_ext_pf: Option<u16>,
    /// Apparent Power Max Setting
    ///
    /// Maximum apparent power setting used to adjust maximum apparent power rating.
    pub va_max: Option<u16>,
    /// Reactive Power Injected Setting
    ///
    /// Maximum injected reactive power setting used to adjust maximum injected reactive power rating.
    pub var_max_inj: Option<u16>,
    /// Reactive Power Absorbed Setting
    ///
    /// Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating.
    pub var_max_abs: Option<u16>,
    /// Charge Rate Max Setting
    ///
    /// Maximum active power charge rate setting used to adjust maximum active power charge rate rating.
    pub w_cha_rte_max: Option<u16>,
    /// Discharge Rate Max Setting
    ///
    /// Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating.
    pub w_dis_cha_rte_max: Option<u16>,
    /// Charge Rate Max VA Setting
    ///
    /// Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating.
    pub va_cha_rte_max: Option<u16>,
    /// Discharge Rate Max VA Setting
    ///
    /// Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating.
    pub va_dis_cha_rte_max: Option<u16>,
    /// Nominal AC Voltage Setting
    ///
    /// Nominal AC voltage setting.
    pub v_nom: Option<u16>,
    /// AC Voltage Max Setting
    ///
    /// AC voltage maximum setting used to adjust AC voltage maximum rating.
    pub v_max: Option<u16>,
    /// AC Voltage Min Setting
    ///
    /// AC voltage minimum setting used to adjust AC voltage minimum rating.
    pub v_min: Option<u16>,
    /// AC Current Max Setting
    ///
    /// Maximum AC current setting used to adjust maximum AC current rating.
    pub a_max: Option<u16>,
    /// PF Over-Excited Setting
    ///
    /// Power factor over-excited setting.
    pub pf_ovr_ext: Option<u16>,
    /// PF Under-Excited Setting
    ///
    /// Power factor under-excited setting.
    pub pf_und_ext: Option<u16>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    pub int_island_cat: Option<IntIslandCat>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    ///
    /// Comments: Scale Factors
    pub w_sf: Option<i16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pub pf_sf: Option<i16>,
    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    pub va_sf: Option<i16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    pub var_sf: Option<i16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    pub v_sf: Option<i16>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    pub a_sf: Option<i16>,
    /// Susceptance Scale Factor
    ///
    /// Susceptance scale factor.
    pub s_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model702 {
    pub const W_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const W_OVR_EXT_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const W_OVR_EXT_RTG_PF: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const W_UND_EXT_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const W_UND_EXT_RTG_PF: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const VA_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const VAR_MAX_INJ_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const VAR_MAX_ABS_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    pub const W_CHA_RTE_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const W_DIS_CHA_RTE_MAX_RTG: crate::Point<Self, Option<u16>> =
        crate::Point::new(9, 1, false);
    pub const VA_CHA_RTE_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const VA_DIS_CHA_RTE_MAX_RTG: crate::Point<Self, Option<u16>> =
        crate::Point::new(11, 1, false);
    pub const V_NOM_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const V_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const V_MIN_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const A_MAX_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, false);
    pub const PF_OVR_EXT_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, false);
    pub const PF_UND_EXT_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const REACT_SUSCEPT_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, false);
    pub const NOR_OP_CAT_RTG: crate::Point<Self, Option<NorOpCatRtg>> =
        crate::Point::new(19, 1, false);
    pub const ABN_OP_CAT_RTG: crate::Point<Self, Option<AbnOpCatRtg>> =
        crate::Point::new(20, 1, false);
    pub const CTRL_MODES: crate::Point<Self, Option<CtrlModes>> = crate::Point::new(21, 2, false);
    pub const INT_ISLAND_CAT_RTG: crate::Point<Self, Option<IntIslandCatRtg>> =
        crate::Point::new(23, 1, false);
    pub const W_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(24, 1, true);
    pub const W_MAX_OVR_EXT: crate::Point<Self, Option<u16>> = crate::Point::new(25, 1, true);
    pub const W_OVR_EXT_PF: crate::Point<Self, Option<u16>> = crate::Point::new(26, 1, true);
    pub const W_MAX_UND_EXT: crate::Point<Self, Option<u16>> = crate::Point::new(27, 1, true);
    pub const W_UND_EXT_PF: crate::Point<Self, Option<u16>> = crate::Point::new(28, 1, true);
    pub const VA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(29, 1, true);
    pub const VAR_MAX_INJ: crate::Point<Self, Option<u16>> = crate::Point::new(30, 1, true);
    pub const VAR_MAX_ABS: crate::Point<Self, Option<u16>> = crate::Point::new(31, 1, true);
    pub const W_CHA_RTE_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, true);
    pub const W_DIS_CHA_RTE_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, true);
    pub const VA_CHA_RTE_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, true);
    pub const VA_DIS_CHA_RTE_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(35, 1, true);
    pub const V_NOM: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, true);
    pub const V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(37, 1, true);
    pub const V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, true);
    pub const A_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(39, 1, true);
    pub const PF_OVR_EXT: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, true);
    pub const PF_UND_EXT: crate::Point<Self, Option<u16>> = crate::Point::new(41, 1, true);
    pub const INT_ISLAND_CAT: crate::Point<Self, Option<IntIslandCat>> =
        crate::Point::new(42, 1, true);
    pub const W_SF: crate::Point<Self, Option<i16>> = crate::Point::new(43, 1, false);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(44, 1, false);
    pub const VA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(45, 1, false);
    pub const VAR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(46, 1, false);
    pub const V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(47, 1, false);
    pub const A_SF: crate::Point<Self, Option<i16>> = crate::Point::new(48, 1, false);
    pub const S_SF: crate::Point<Self, Option<i16>> = crate::Point::new(49, 1, false);
}
impl crate::Model for Model702 {
    const ID: u16 = 702;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            w_max_rtg: Self::W_MAX_RTG.from_data(data)?,
            w_ovr_ext_rtg: Self::W_OVR_EXT_RTG.from_data(data)?,
            w_ovr_ext_rtg_pf: Self::W_OVR_EXT_RTG_PF.from_data(data)?,
            w_und_ext_rtg: Self::W_UND_EXT_RTG.from_data(data)?,
            w_und_ext_rtg_pf: Self::W_UND_EXT_RTG_PF.from_data(data)?,
            va_max_rtg: Self::VA_MAX_RTG.from_data(data)?,
            var_max_inj_rtg: Self::VAR_MAX_INJ_RTG.from_data(data)?,
            var_max_abs_rtg: Self::VAR_MAX_ABS_RTG.from_data(data)?,
            w_cha_rte_max_rtg: Self::W_CHA_RTE_MAX_RTG.from_data(data)?,
            w_dis_cha_rte_max_rtg: Self::W_DIS_CHA_RTE_MAX_RTG.from_data(data)?,
            va_cha_rte_max_rtg: Self::VA_CHA_RTE_MAX_RTG.from_data(data)?,
            va_dis_cha_rte_max_rtg: Self::VA_DIS_CHA_RTE_MAX_RTG.from_data(data)?,
            v_nom_rtg: Self::V_NOM_RTG.from_data(data)?,
            v_max_rtg: Self::V_MAX_RTG.from_data(data)?,
            v_min_rtg: Self::V_MIN_RTG.from_data(data)?,
            a_max_rtg: Self::A_MAX_RTG.from_data(data)?,
            pf_ovr_ext_rtg: Self::PF_OVR_EXT_RTG.from_data(data)?,
            pf_und_ext_rtg: Self::PF_UND_EXT_RTG.from_data(data)?,
            react_suscept_rtg: Self::REACT_SUSCEPT_RTG.from_data(data)?,
            nor_op_cat_rtg: Self::NOR_OP_CAT_RTG.from_data(data)?,
            abn_op_cat_rtg: Self::ABN_OP_CAT_RTG.from_data(data)?,
            ctrl_modes: Self::CTRL_MODES.from_data(data)?,
            int_island_cat_rtg: Self::INT_ISLAND_CAT_RTG.from_data(data)?,
            w_max: Self::W_MAX.from_data(data)?,
            w_max_ovr_ext: Self::W_MAX_OVR_EXT.from_data(data)?,
            w_ovr_ext_pf: Self::W_OVR_EXT_PF.from_data(data)?,
            w_max_und_ext: Self::W_MAX_UND_EXT.from_data(data)?,
            w_und_ext_pf: Self::W_UND_EXT_PF.from_data(data)?,
            va_max: Self::VA_MAX.from_data(data)?,
            var_max_inj: Self::VAR_MAX_INJ.from_data(data)?,
            var_max_abs: Self::VAR_MAX_ABS.from_data(data)?,
            w_cha_rte_max: Self::W_CHA_RTE_MAX.from_data(data)?,
            w_dis_cha_rte_max: Self::W_DIS_CHA_RTE_MAX.from_data(data)?,
            va_cha_rte_max: Self::VA_CHA_RTE_MAX.from_data(data)?,
            va_dis_cha_rte_max: Self::VA_DIS_CHA_RTE_MAX.from_data(data)?,
            v_nom: Self::V_NOM.from_data(data)?,
            v_max: Self::V_MAX.from_data(data)?,
            v_min: Self::V_MIN.from_data(data)?,
            a_max: Self::A_MAX.from_data(data)?,
            pf_ovr_ext: Self::PF_OVR_EXT.from_data(data)?,
            pf_und_ext: Self::PF_UND_EXT.from_data(data)?,
            int_island_cat: Self::INT_ISLAND_CAT.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            s_sf: Self::S_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m702
    }
}
/// Normal Operating Category
///
/// Normal operating performance category as specified in IEEE 1547-2018.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NorOpCatRtg {
    /// Category A
    CatA = 0,
    /// Category B
    CatB = 1,
}
impl crate::Value for NorOpCatRtg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NorOpCatRtg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NorOpCatRtg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Abnormal Operating Category
///
/// Abnormal operating performance category as specified in IEEE 1547-2018.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum AbnOpCatRtg {
    /// Category I
    Cat1 = 0,
    /// Category II
    Cat2 = 1,
    /// Category III
    Cat3 = 2,
}
impl crate::Value for AbnOpCatRtg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<AbnOpCatRtg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                AbnOpCatRtg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
bitflags::bitflags! {
    #[doc = " Supported Control Modes"] #[doc = " "] #[doc =
    " Supported control mode functions."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct CtrlModes : u32 { #[doc = " Limit Maximum Active Power"] const MaxW = 1; #[doc
    = " Fixed Active Power"] const FixedW = 2; #[doc = " Fixed Reactive Power"] const
    FixedVar = 4; #[doc = " Fixed Power Factor"] const FixedPf = 8; #[doc =
    " Volt-Var Function"] const VoltVar = 16; #[doc = " Freq-Watt Function"] const
    FreqWatt = 32; #[doc = " Dynamic Reactive Current Function"] const DynReactCurr = 64;
    #[doc = " Low-Voltage Trip"] const LvTrip = 128; #[doc = " High-Voltage Trip"] const
    HvTrip = 256; #[doc = " Watt-Var Function"] const WattVar = 512; #[doc =
    " Volt-Watt Function"] const VoltWatt = 1024; #[doc = " Scheduling"] const Scheduled
    = 2048; #[doc = " Low-Frequency Trip"] const LfTrip = 4096; #[doc =
    " High-Frequency Trip"] const HfTrip = 8192; }
}
impl crate::Value for CtrlModes {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<CtrlModes> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(CtrlModes::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            4294967295u32.encode()
        }
    }
}
bitflags::bitflags! {
    #[doc = " Intentional Island Categories"] #[doc = " "] #[doc =
    " Intentional island categories."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct IntIslandCatRtg : u16 { #[doc = " Uncategorized"] const Uncategorized = 1;
    #[doc = " Intentional Island-Capable"] const IntIslCapable = 2; #[doc =
    " Black Start-Capable"] const BlackStartCapable = 4; #[doc = " Isochronous-Capable"]
    const IsochCapable = 8; }
}
impl crate::Value for IntIslandCatRtg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<IntIslandCatRtg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(IntIslandCatRtg::from_bits_retain(value)))
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
bitflags::bitflags! {
    #[doc = " Intentional Island Categories"] #[doc = " "] #[doc =
    " Intentional island categories."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct IntIslandCat : u16 { #[doc = " Uncategorized"] const Uncategorized = 1; #[doc
    = " Intentional Island-Capable"] const IntIslCapable = 2; #[doc =
    " Black Start-Capable"] const BlackStartCapable = 4; #[doc = " Isochronous-Capable"]
    const IsochCapable = 8; }
}
impl crate::Value for IntIslandCat {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<IntIslandCat> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(IntIslandCat::from_bits_retain(value)))
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
