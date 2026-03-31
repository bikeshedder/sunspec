//! DER AC Measurement
/// Type alias for [`DerMeasureAc`].
pub type Model701 = DerMeasureAc;
/// DER AC Measurement
///
/// DER AC measurement model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerMeasureAc {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    ///
    /// Comments: Wiring Type
    pub ac_type: AcType,
    /// Operating State
    ///
    /// Operating state of the DER.
    ///
    /// Comments: Operating State
    pub st: Option<St>,
    /// Inverter State
    ///
    /// Enumerated value.  Inverter state.
    ///
    /// Comments: Inverter State
    pub inv_st: Option<InvSt>,
    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    ///
    /// Comments: Grid Connection State
    pub conn_st: Option<ConnSt>,
    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    ///
    /// Comments: Alarms
    pub alrm: Option<Alrm>,
    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    pub der_mode: Option<DerMode>,
    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    ///
    /// Comments: Scale Factors
    pub w: Option<i16>,
    /// Apparent Power
    ///
    /// Total apparent power.
    pub va: Option<i16>,
    /// Reactive Power
    ///
    /// Total reactive power.
    pub var: Option<i16>,
    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    pub pf: Option<i16>,
    /// Total AC Current
    ///
    /// Total AC current.
    pub a: Option<i16>,
    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    pub llv: Option<u16>,
    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    pub lnv: Option<u16>,
    /// Frequency
    ///
    /// AC frequency.
    pub hz: Option<u32>,
    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    pub tot_wh_inj: Option<u64>,
    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    pub tot_wh_abs: Option<u64>,
    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    pub tot_varh_inj: Option<u64>,
    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    pub tot_varh_abs: Option<u64>,
    /// Ambient Temperature
    ///
    /// Ambient temperature.
    ///
    /// Comments: Temperatures
    pub tmp_amb: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    pub tmp_cab: Option<i16>,
    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    pub tmp_snk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer temperature.
    pub tmp_trns: Option<i16>,
    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    pub tmp_sw: Option<i16>,
    /// Other Temperature
    ///
    /// Other temperature.
    pub tmp_ot: Option<i16>,
    /// Watts L1
    ///
    /// Active power L1.
    ///
    /// Comments: L1
    pub wl1: Option<i16>,
    /// VA L1
    ///
    /// Apparent power L1.
    pub val1: Option<i16>,
    /// Var L1
    ///
    /// Reactive power L1.
    pub var_l1: Option<i16>,
    /// PF L1
    ///
    /// Power factor phase L1.
    pub pfl1: Option<i16>,
    /// Amps L1
    ///
    /// Current phase L1.
    pub al1: Option<i16>,
    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    pub vl1l2: Option<u16>,
    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    pub vl1: Option<u16>,
    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    pub tot_wh_inj_l1: Option<u64>,
    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    pub tot_wh_abs_l1: Option<u64>,
    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    pub tot_varh_inj_l1: Option<u64>,
    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    pub tot_varh_abs_l1: Option<u64>,
    /// Watts L2
    ///
    /// Active power L2.
    ///
    /// Comments: L2
    pub wl2: Option<i16>,
    /// VA L2
    ///
    /// Apparent power L2.
    pub val2: Option<i16>,
    /// Var L2
    ///
    /// Reactive power L2.
    pub var_l2: Option<i16>,
    /// PF L2
    ///
    /// Power factor L2.
    pub pfl2: Option<i16>,
    /// Amps L2
    ///
    /// Current L2.
    pub al2: Option<i16>,
    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    pub vl2l3: Option<u16>,
    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    pub vl2: Option<u16>,
    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    pub tot_wh_inj_l2: Option<u64>,
    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    pub tot_wh_abs_l2: Option<u64>,
    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    pub tot_varh_inj_l2: Option<u64>,
    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    pub tot_varh_abs_l2: Option<u64>,
    /// Watts L3
    ///
    /// Active power L3.
    ///
    /// Comments: L3
    pub wl3: Option<i16>,
    /// VA L3
    ///
    /// Apparent power L3.
    pub val3: Option<i16>,
    /// Var L3
    ///
    /// Reactive power L3.
    pub var_l3: Option<i16>,
    /// PF L3
    ///
    /// Power factor L3.
    pub pfl3: Option<i16>,
    /// Amps L3
    ///
    /// Current L3.
    pub al3: Option<i16>,
    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    pub vl3l1: Option<u16>,
    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    pub vl3: Option<u16>,
    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    pub tot_wh_inj_l3: Option<u64>,
    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    pub tot_wh_abs_l3: Option<u64>,
    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    pub tot_varh_inj_l3: Option<u64>,
    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    pub tot_varh_abs_l3: Option<u64>,
    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    ///
    /// Comments: Active Power Throttling
    pub throt_pct: Option<u16>,
    /// Throttle Source Information
    ///
    /// Active throttling source.
    pub throt_src: Option<ThrotSrc>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    ///
    /// Comments: Scale Factors
    pub a_sf: Option<i16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    pub v_sf: Option<i16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    pub hz_sf: Option<i16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
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
    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    pub tot_wh_sf: Option<i16>,
    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    pub tot_varh_sf: Option<i16>,
    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    pub tmp_sf: Option<i16>,
    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    ///
    /// Comments: Manufacturer Alarm Information
    pub mn_alrm_info: Option<String>,
}
#[allow(missing_docs)]
impl DerMeasureAc {
    pub const AC_TYPE: crate::Point<Self, AcType> = crate::Point::new(0, 1, false);
    pub const ST: crate::Point<Self, Option<St>> = crate::Point::new(1, 1, false);
    pub const INV_ST: crate::Point<Self, Option<InvSt>> = crate::Point::new(2, 1, false);
    pub const CONN_ST: crate::Point<Self, Option<ConnSt>> = crate::Point::new(3, 1, false);
    pub const ALRM: crate::Point<Self, Option<Alrm>> = crate::Point::new(4, 2, false);
    pub const DER_MODE: crate::Point<Self, Option<DerMode>> = crate::Point::new(6, 2, false);
    pub const W: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const VA: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, false);
    pub const PF: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, false);
    pub const A: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, false);
    pub const LLV: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const LNV: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const HZ: crate::Point<Self, Option<u32>> = crate::Point::new(15, 2, false);
    pub const TOT_WH_INJ: crate::Point<Self, Option<u64>> = crate::Point::new(17, 4, false);
    pub const TOT_WH_ABS: crate::Point<Self, Option<u64>> = crate::Point::new(21, 4, false);
    pub const TOT_VARH_INJ: crate::Point<Self, Option<u64>> = crate::Point::new(25, 4, false);
    pub const TOT_VARH_ABS: crate::Point<Self, Option<u64>> = crate::Point::new(29, 4, false);
    pub const TMP_AMB: crate::Point<Self, Option<i16>> = crate::Point::new(33, 1, false);
    pub const TMP_CAB: crate::Point<Self, Option<i16>> = crate::Point::new(34, 1, false);
    pub const TMP_SNK: crate::Point<Self, Option<i16>> = crate::Point::new(35, 1, false);
    pub const TMP_TRNS: crate::Point<Self, Option<i16>> = crate::Point::new(36, 1, false);
    pub const TMP_SW: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, false);
    pub const TMP_OT: crate::Point<Self, Option<i16>> = crate::Point::new(38, 1, false);
    pub const WL1: crate::Point<Self, Option<i16>> = crate::Point::new(39, 1, false);
    pub const VAL1: crate::Point<Self, Option<i16>> = crate::Point::new(40, 1, false);
    pub const VAR_L1: crate::Point<Self, Option<i16>> = crate::Point::new(41, 1, false);
    pub const PFL1: crate::Point<Self, Option<i16>> = crate::Point::new(42, 1, false);
    pub const AL1: crate::Point<Self, Option<i16>> = crate::Point::new(43, 1, false);
    pub const VL1L2: crate::Point<Self, Option<u16>> = crate::Point::new(44, 1, false);
    pub const VL1: crate::Point<Self, Option<u16>> = crate::Point::new(45, 1, false);
    pub const TOT_WH_INJ_L1: crate::Point<Self, Option<u64>> = crate::Point::new(46, 4, false);
    pub const TOT_WH_ABS_L1: crate::Point<Self, Option<u64>> = crate::Point::new(50, 4, false);
    pub const TOT_VARH_INJ_L1: crate::Point<Self, Option<u64>> = crate::Point::new(54, 4, false);
    pub const TOT_VARH_ABS_L1: crate::Point<Self, Option<u64>> = crate::Point::new(58, 4, false);
    pub const WL2: crate::Point<Self, Option<i16>> = crate::Point::new(62, 1, false);
    pub const VAL2: crate::Point<Self, Option<i16>> = crate::Point::new(63, 1, false);
    pub const VAR_L2: crate::Point<Self, Option<i16>> = crate::Point::new(64, 1, false);
    pub const PFL2: crate::Point<Self, Option<i16>> = crate::Point::new(65, 1, false);
    pub const AL2: crate::Point<Self, Option<i16>> = crate::Point::new(66, 1, false);
    pub const VL2L3: crate::Point<Self, Option<u16>> = crate::Point::new(67, 1, false);
    pub const VL2: crate::Point<Self, Option<u16>> = crate::Point::new(68, 1, false);
    pub const TOT_WH_INJ_L2: crate::Point<Self, Option<u64>> = crate::Point::new(69, 4, false);
    pub const TOT_WH_ABS_L2: crate::Point<Self, Option<u64>> = crate::Point::new(73, 4, false);
    pub const TOT_VARH_INJ_L2: crate::Point<Self, Option<u64>> = crate::Point::new(77, 4, false);
    pub const TOT_VARH_ABS_L2: crate::Point<Self, Option<u64>> = crate::Point::new(81, 4, false);
    pub const WL3: crate::Point<Self, Option<i16>> = crate::Point::new(85, 1, false);
    pub const VAL3: crate::Point<Self, Option<i16>> = crate::Point::new(86, 1, false);
    pub const VAR_L3: crate::Point<Self, Option<i16>> = crate::Point::new(87, 1, false);
    pub const PFL3: crate::Point<Self, Option<i16>> = crate::Point::new(88, 1, false);
    pub const AL3: crate::Point<Self, Option<i16>> = crate::Point::new(89, 1, false);
    pub const VL3L1: crate::Point<Self, Option<u16>> = crate::Point::new(90, 1, false);
    pub const VL3: crate::Point<Self, Option<u16>> = crate::Point::new(91, 1, false);
    pub const TOT_WH_INJ_L3: crate::Point<Self, Option<u64>> = crate::Point::new(92, 4, false);
    pub const TOT_WH_ABS_L3: crate::Point<Self, Option<u64>> = crate::Point::new(96, 4, false);
    pub const TOT_VARH_INJ_L3: crate::Point<Self, Option<u64>> = crate::Point::new(100, 4, false);
    pub const TOT_VARH_ABS_L3: crate::Point<Self, Option<u64>> = crate::Point::new(104, 4, false);
    pub const THROT_PCT: crate::Point<Self, Option<u16>> = crate::Point::new(108, 1, false);
    pub const THROT_SRC: crate::Point<Self, Option<ThrotSrc>> = crate::Point::new(109, 2, false);
    pub const A_SF: crate::Point<Self, Option<i16>> = crate::Point::new(111, 1, false);
    pub const V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(112, 1, false);
    pub const HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(113, 1, false);
    pub const W_SF: crate::Point<Self, Option<i16>> = crate::Point::new(114, 1, false);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(115, 1, false);
    pub const VA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(116, 1, false);
    pub const VAR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(117, 1, false);
    pub const TOT_WH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(118, 1, false);
    pub const TOT_VARH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(119, 1, false);
    pub const TMP_SF: crate::Point<Self, Option<i16>> = crate::Point::new(120, 1, false);
    pub const MN_ALRM_INFO: crate::Point<Self, Option<String>> = crate::Point::new(121, 32, false);
}
static DER_MEASURE_AC_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "ac_type",
        label: "AC Wiring Type",
        description: "AC wiring type.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "st",
        label: "Operating State",
        description: "Operating state of the DER.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "inv_st",
        label: "Inverter State",
        description: "Enumerated value.  Inverter state.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "conn_st",
        label: "Grid Connection State",
        description: "Grid connection state of the DER.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alrm",
        label: "Alarm Bitfield",
        description: "Active alarms for the DER.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "der_mode",
        label: "DER Operational Characteristics",
        description: "Current operational characteristics of the DER.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w",
        label: "Active Power",
        description: "Total active power. Active power is positive for DER generation and negative for absorption.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "va",
        label: "Apparent Power",
        description: "Total apparent power.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var",
        label: "Reactive Power",
        description: "Total reactive power.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pf",
        label: "Power Factor",
        description: "Power factor. The sign of power factor should be the sign of active power.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a",
        label: "Total AC Current",
        description: "Total AC current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "llv",
        label: "Voltage LL",
        description: "Line to line AC voltage as an average of active phases.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "lnv",
        label: "Voltage LN",
        description: "Line to neutral AC voltage as an average of active phases.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz",
        label: "Frequency",
        description: "AC frequency.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_inj",
        label: "Total Energy Injected",
        description: "Total active energy injected (Quadrants 1 & 4).",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_abs",
        label: "Total Energy Absorbed",
        description: "Total active energy absorbed (Quadrants 2 & 3).",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_inj",
        label: "Total Reactive Energy Inj",
        description: "Total reactive energy injected (Quadrants 1 & 2).",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_abs",
        label: "Total Reactive Energy Abs",
        description: "Total reactive energy absorbed (Quadrants 3 & 4).",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_amb",
        label: "Ambient Temperature",
        description: "Ambient temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_cab",
        label: "Cabinet Temperature",
        description: "Cabinet temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_snk",
        label: "Heat Sink Temperature",
        description: "Heat sink temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_trns",
        label: "Transformer Temperature",
        description: "Transformer temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_sw",
        label: "IGBT/MOSFET Temperature",
        description: "IGBT/MOSFET temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_ot",
        label: "Other Temperature",
        description: "Other temperature.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wl1",
        label: "Watts L1",
        description: "Active power L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val1",
        label: "VA L1",
        description: "Apparent power L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var_l1",
        label: "Var L1",
        description: "Reactive power L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pfl1",
        label: "PF L1",
        description: "Power factor phase L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "al1",
        label: "Amps L1",
        description: "Current phase L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl1l2",
        label: "Phase Voltage L1-L2",
        description: "Phase voltage L1-L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl1",
        label: "Phase Voltage L1-N",
        description: "Phase voltage L1-N.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_inj_l1",
        label: "Total Watt-Hours Inj L1",
        description: "Total active energy injected L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_abs_l1",
        label: "Total Watt-Hours Abs L1",
        description: "Total active energy absorbed L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_inj_l1",
        label: "Total Var-Hours Inj L1",
        description: "Total reactive energy injected L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_abs_l1",
        label: "Total Var-Hours Abs L1",
        description: "Total reactive energy absorbed L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wl2",
        label: "Watts L2",
        description: "Active power L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val2",
        label: "VA L2",
        description: "Apparent power L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var_l2",
        label: "Var L2",
        description: "Reactive power L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pfl2",
        label: "PF L2",
        description: "Power factor L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "al2",
        label: "Amps L2",
        description: "Current L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl2l3",
        label: "Phase Voltage L2-L3",
        description: "Phase voltage L2-L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl2",
        label: "Phase Voltage L2-N",
        description: "Phase voltage L2-N.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_inj_l2",
        label: "Total Watt-Hours Inj L2",
        description: "Total active energy injected L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_abs_l2",
        label: "Total Watt-Hours Abs L2",
        description: "Total active energy absorbed L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_inj_l2",
        label: "Total Var-Hours Inj L2",
        description: "Total reactive energy injected L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_abs_l2",
        label: "Total Var-Hours Abs L2",
        description: "Total reactive energy absorbed L2.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wl3",
        label: "Watts L3",
        description: "Active power L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "val3",
        label: "VA L3",
        description: "Apparent power L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var_l3",
        label: "Var L3",
        description: "Reactive power L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pfl3",
        label: "PF L3",
        description: "Power factor L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "al3",
        label: "Amps L3",
        description: "Current L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl3l1",
        label: "Phase Voltage L3-L1",
        description: "Phase voltage L3-L1.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "vl3",
        label: "Phase Voltage L3-N",
        description: "Phase voltage L3-N.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_inj_l3",
        label: "Total Watt-Hours Inj L3",
        description: "Total active energy injected L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_abs_l3",
        label: "Total Watt-Hours Abs L3",
        description: "Total active energy absorbed L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_inj_l3",
        label: "Total Var-Hours Inj L3",
        description: "Total reactive energy injected L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_abs_l3",
        label: "Total Var-Hours Abs L3",
        description: "Total reactive energy absorbed L3.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "throt_pct",
        label: "Throttling In Pct",
        description: "Throttling in pct of maximum active power.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "throt_src",
        label: "Throttle Source Information",
        description: "Active throttling source.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "Current Scale Factor",
        description: "Current scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "Voltage Scale Factor",
        description: "Voltage scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hz_sf",
        label: "Frequency Scale Factor",
        description: "Frequency scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_sf",
        label: "Active Power Scale Factor",
        description: "Active power scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pf_sf",
        label: "Power Factor Scale Factor",
        description: "Power factor scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "va_sf",
        label: "Apparent Power Scale Factor",
        description: "Apparent power scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "var_sf",
        label: "Reactive Power Scale Factor",
        description: "Reactive power scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_wh_sf",
        label: "Active Energy Scale Factor",
        description: "Active energy scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tot_varh_sf",
        label: "Reactive Energy Scale Factor",
        description: "Reactive energy scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_sf",
        label: "Temperature Scale Factor",
        description: "Temperature scale factor.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "mn_alrm_info",
        label: "Manufacturer Alarm Info",
        description: "Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.",
        kind: crate::FieldKind::Point,
    },
];
static DER_MEASURE_AC_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DERMeasureAC",
    label: "DER AC Measurement",
    description: "DER AC measurement model.",
    fields: DER_MEASURE_AC_FIELDS,
};
impl crate::GroupMeta for DerMeasureAc {
    fn group_info() -> &'static crate::GroupInfo {
        &DER_MEASURE_AC_GROUP_INFO
    }
}
impl crate::Group for DerMeasureAc {
    const LEN: u16 = 153;
}
impl DerMeasureAc {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ac_type: Self::AC_TYPE.from_data(data)?,
                st: Self::ST.from_data(data)?,
                inv_st: Self::INV_ST.from_data(data)?,
                conn_st: Self::CONN_ST.from_data(data)?,
                alrm: Self::ALRM.from_data(data)?,
                der_mode: Self::DER_MODE.from_data(data)?,
                w: Self::W.from_data(data)?,
                va: Self::VA.from_data(data)?,
                var: Self::VAR.from_data(data)?,
                pf: Self::PF.from_data(data)?,
                a: Self::A.from_data(data)?,
                llv: Self::LLV.from_data(data)?,
                lnv: Self::LNV.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                tot_wh_inj: Self::TOT_WH_INJ.from_data(data)?,
                tot_wh_abs: Self::TOT_WH_ABS.from_data(data)?,
                tot_varh_inj: Self::TOT_VARH_INJ.from_data(data)?,
                tot_varh_abs: Self::TOT_VARH_ABS.from_data(data)?,
                tmp_amb: Self::TMP_AMB.from_data(data)?,
                tmp_cab: Self::TMP_CAB.from_data(data)?,
                tmp_snk: Self::TMP_SNK.from_data(data)?,
                tmp_trns: Self::TMP_TRNS.from_data(data)?,
                tmp_sw: Self::TMP_SW.from_data(data)?,
                tmp_ot: Self::TMP_OT.from_data(data)?,
                wl1: Self::WL1.from_data(data)?,
                val1: Self::VAL1.from_data(data)?,
                var_l1: Self::VAR_L1.from_data(data)?,
                pfl1: Self::PFL1.from_data(data)?,
                al1: Self::AL1.from_data(data)?,
                vl1l2: Self::VL1L2.from_data(data)?,
                vl1: Self::VL1.from_data(data)?,
                tot_wh_inj_l1: Self::TOT_WH_INJ_L1.from_data(data)?,
                tot_wh_abs_l1: Self::TOT_WH_ABS_L1.from_data(data)?,
                tot_varh_inj_l1: Self::TOT_VARH_INJ_L1.from_data(data)?,
                tot_varh_abs_l1: Self::TOT_VARH_ABS_L1.from_data(data)?,
                wl2: Self::WL2.from_data(data)?,
                val2: Self::VAL2.from_data(data)?,
                var_l2: Self::VAR_L2.from_data(data)?,
                pfl2: Self::PFL2.from_data(data)?,
                al2: Self::AL2.from_data(data)?,
                vl2l3: Self::VL2L3.from_data(data)?,
                vl2: Self::VL2.from_data(data)?,
                tot_wh_inj_l2: Self::TOT_WH_INJ_L2.from_data(data)?,
                tot_wh_abs_l2: Self::TOT_WH_ABS_L2.from_data(data)?,
                tot_varh_inj_l2: Self::TOT_VARH_INJ_L2.from_data(data)?,
                tot_varh_abs_l2: Self::TOT_VARH_ABS_L2.from_data(data)?,
                wl3: Self::WL3.from_data(data)?,
                val3: Self::VAL3.from_data(data)?,
                var_l3: Self::VAR_L3.from_data(data)?,
                pfl3: Self::PFL3.from_data(data)?,
                al3: Self::AL3.from_data(data)?,
                vl3l1: Self::VL3L1.from_data(data)?,
                vl3: Self::VL3.from_data(data)?,
                tot_wh_inj_l3: Self::TOT_WH_INJ_L3.from_data(data)?,
                tot_wh_abs_l3: Self::TOT_WH_ABS_L3.from_data(data)?,
                tot_varh_inj_l3: Self::TOT_VARH_INJ_L3.from_data(data)?,
                tot_varh_abs_l3: Self::TOT_VARH_ABS_L3.from_data(data)?,
                throt_pct: Self::THROT_PCT.from_data(data)?,
                throt_src: Self::THROT_SRC.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                pf_sf: Self::PF_SF.from_data(data)?,
                va_sf: Self::VA_SF.from_data(data)?,
                var_sf: Self::VAR_SF.from_data(data)?,
                tot_wh_sf: Self::TOT_WH_SF.from_data(data)?,
                tot_varh_sf: Self::TOT_VARH_SF.from_data(data)?,
                tmp_sf: Self::TMP_SF.from_data(data)?,
                mn_alrm_info: Self::MN_ALRM_INFO.from_data(data)?,
            },
        ))
    }
}
/// AC Wiring Type
///
/// AC wiring type.
///
/// Comments: Wiring Type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AcType {
    /// Single Phase
    SinglePhase,
    /// Split Phase
    SplitPhase,
    /// Three Phase
    ThreePhase,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for AcType {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::SinglePhase,
            1 => Self::SplitPhase,
            2 => Self::ThreePhase,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::SinglePhase => 0,
            Self::SplitPhase => 1,
            Self::ThreePhase => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for AcType {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Operating State
///
/// Operating state of the DER.
///
/// Comments: Operating State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum St {
    /// Off
    Off,
    /// On
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for St {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for St {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Inverter State
///
/// Enumerated value.  Inverter state.
///
/// Comments: Inverter State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum InvSt {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Sleeping,
    #[allow(missing_docs)]
    Starting,
    #[allow(missing_docs)]
    Running,
    #[allow(missing_docs)]
    Throttled,
    #[allow(missing_docs)]
    ShuttingDown,
    #[allow(missing_docs)]
    Fault,
    #[allow(missing_docs)]
    Standby,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for InvSt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Sleeping,
            2 => Self::Starting,
            3 => Self::Running,
            4 => Self::Throttled,
            5 => Self::ShuttingDown,
            6 => Self::Fault,
            7 => Self::Standby,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::Sleeping => 1,
            Self::Starting => 2,
            Self::Running => 3,
            Self::Throttled => 4,
            Self::ShuttingDown => 5,
            Self::Fault => 6,
            Self::Standby => 7,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for InvSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Grid Connection State
///
/// Grid connection state of the DER.
///
/// Comments: Grid Connection State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ConnSt {
    /// Disconnected
    ///
    /// Disconnected from the grid.
    Disconnected,
    /// Connected
    ///
    /// Connected to the grid.
    Connected,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ConnSt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disconnected,
            1 => Self::Connected,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disconnected => 0,
            Self::Connected => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ConnSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Alarm Bitfield"] #[doc = " "] #[doc = " Active alarms for the DER."] #[doc
    = " "] #[doc = " Comments: Alarms"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Alrm : u32 { #[doc = " Ground Fault"] const GroundFault = 1; #[doc =
    " DC Over Voltage"] const DcOverVolt = 2; #[doc = " AC Disconnect Open"] const
    AcDisconnect = 4; #[doc = " DC Disconnect Open"] const DcDisconnect = 8; #[doc =
    " Grid Disconnect"] const GridDisconnect = 16; #[doc = " Cabinet Open"] const
    CabinetOpen = 32; #[doc = " Manual Shutdown"] const ManualShutdown = 64; #[doc =
    " Over Temperature"] const OverTemp = 128; #[doc = " Frequency Above Limit"] const
    OverFrequency = 256; #[doc = " Frequency Under Limit"] const UnderFrequency = 512;
    #[doc = " AC Voltage Above Limit"] const AcOverVolt = 1024; #[doc =
    " AC Voltage Under Limit"] const AcUnderVolt = 2048; #[doc =
    " Blown String Fuse On Input"] const BlownStringFuse = 4096; #[doc =
    " Under Temperature"] const UnderTemp = 8192; #[doc =
    " Generic Memory Or Communication Error (Internal)"] const MemoryLoss = 16384; #[doc
    = " Hardware Test Failure"] const HwTestFailure = 32768; #[doc =
    " Manufacturer Alarm"] #[doc = " "] #[doc =
    " Manufacturer alarm, see ManAlrmInfo field for more information."] const
    ManufacturerAlrm = 65536; }
}
impl crate::Value for Alrm {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Alrm {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " DER Operational Characteristics"] #[doc = " "] #[doc =
    " Current operational characteristics of the DER."] #[derive(Copy, Clone, Debug, Eq,
    PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct DerMode : u32 { #[doc = " Grid Following"] #[doc =
    " "] #[doc = " The DER is operating as part of a larger grid."] const GridFollowing =
    1; #[doc = " Grid Forming"] #[doc = " "] #[doc = " The DER is providing the grid."]
    const GridForming = 2; #[doc = " PV Output Clipped"] #[doc = " "] #[doc =
    " The PV output is clipped."] const PvClipped = 4; }
}
impl crate::Value for DerMode {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for DerMode {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Throttle Source Information"] #[doc = " "] #[doc =
    " Active throttling source."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct ThrotSrc : u32 { #[allow(missing_docs)] const MaxW = 1; #[allow(missing_docs)]
    const FixedW = 2; #[allow(missing_docs)] const FixedVar = 4; #[allow(missing_docs)]
    const FixedPf = 8; #[allow(missing_docs)] const VoltVar = 16; #[allow(missing_docs)]
    const FreqWatt = 32; #[allow(missing_docs)] const DynReactCurr = 64;
    #[allow(missing_docs)] const Lvrt = 128; #[allow(missing_docs)] const Hvrt = 256;
    #[allow(missing_docs)] const WattVar = 512; #[allow(missing_docs)] const VoltWatt =
    1024; #[allow(missing_docs)] const Scheduled = 2048; #[allow(missing_docs)] const
    Lfrt = 4096; #[allow(missing_docs)] const Hfrt = 8192; #[allow(missing_docs)] const
    Derated = 16384; }
}
impl crate::Value for ThrotSrc {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for ThrotSrc {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for DerMeasureAc {
    const ID: u16 = 701;
    const NAME: &'static str = "DERMeasureAC";
    const LABEL: &'static str = "DER AC Measurement";
    const DESCRIPTION: &'static str = "DER AC measurement model.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m701
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
