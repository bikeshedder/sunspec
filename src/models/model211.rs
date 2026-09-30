//! single phase (AN or AB) meter
/// Type alias for [`AcMeterAnOrAbFloat`].
pub type Model211 = AcMeterAnOrAbFloat;
/// single phase (AN or AB) meter
///
/// Detail: Float
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AcMeterAnOrAbFloat {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub a_ph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub a_ph_b: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub a_ph_c: Option<f32>,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub ph_v: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_v_ph_a: Option<f32>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_v_ph_b: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_v_ph_c: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub pp_v: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_v_ph_ab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_v_ph_bc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_v_ph_ca: Option<f32>,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub w_ph_a: Option<f32>,
    /// Watts phase B
    pub w_ph_b: Option<f32>,
    /// Watts phase C
    pub w_ph_c: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub va_ph_a: Option<f32>,
    /// VA phase B
    pub va_ph_b: Option<f32>,
    /// VA phase C
    pub va_ph_c: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub var_ph_a: Option<f32>,
    /// VAR phase B
    pub var_ph_b: Option<f32>,
    /// VAR phase C
    pub var_ph_c: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pf_ph_a: Option<f32>,
    /// PF phase B
    pub pf_ph_b: Option<f32>,
    /// PF phase C
    pub pf_ph_c: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub tot_wh_exp: f32,
    /// Total Watt-hours Exported phase A
    pub tot_wh_exp_ph_a: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub tot_wh_exp_ph_b: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub tot_wh_exp_ph_c: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub tot_wh_imp: f32,
    /// Total Watt-hours Imported phase A
    pub tot_wh_imp_ph_a: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub tot_wh_imp_ph_b: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub tot_wh_imp_ph_c: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub tot_vah_exp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub tot_vah_exp_ph_a: Option<f32>,
    /// Total VA-hours Exported phase B
    pub tot_vah_exp_ph_b: Option<f32>,
    /// Total VA-hours Exported phase C
    pub tot_vah_exp_ph_c: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_vah_imp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub tot_vah_imp_ph_a: Option<f32>,
    /// Total VA-hours Imported phase B
    pub tot_vah_imp_ph_b: Option<f32>,
    /// Total VA-hours Imported phase C
    pub tot_vah_imp_ph_c: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_varh_imp_q1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub tot_varh_imp_q1_ph_a: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub tot_varh_imp_q1_ph_b: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub tot_varh_imp_q1_ph_c: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_varh_imp_q2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub tot_varh_imp_q2_ph_a: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub tot_varh_imp_q2_ph_b: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub tot_varh_imp_q2_ph_c: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_varh_exp_q3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub tot_varh_exp_q3_ph_a: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub tot_varh_exp_q3_ph_b: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub tot_varh_exp_q3_ph_c: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_varh_exp_q4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub tot_varh_exp_q4_ph_a: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub tot_varh_exp_q4_ph_b: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub tot_varh_exp_q4_ph_c: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: Evt,
}
#[allow(missing_docs)]
impl AcMeterAnOrAbFloat {
    pub const A: crate::Point<Self, f32> = crate::Point::new(0, 2, false);
    pub const A_PH_A: crate::Point<Self, f32> = crate::Point::new(2, 2, false);
    pub const A_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(4, 2, false);
    pub const A_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(6, 2, false);
    pub const PH_V: crate::Point<Self, Option<f32>> = crate::Point::new(8, 2, false);
    pub const PH_V_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(10, 2, false);
    pub const PH_V_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(12, 2, false);
    pub const PH_V_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(14, 2, false);
    pub const PP_V: crate::Point<Self, Option<f32>> = crate::Point::new(16, 2, false);
    pub const PP_V_PH_AB: crate::Point<Self, Option<f32>> = crate::Point::new(18, 2, false);
    pub const PP_V_PH_BC: crate::Point<Self, Option<f32>> = crate::Point::new(20, 2, false);
    pub const PP_V_PH_CA: crate::Point<Self, Option<f32>> = crate::Point::new(22, 2, false);
    pub const HZ: crate::Point<Self, f32> = crate::Point::new(24, 2, false);
    pub const W: crate::Point<Self, f32> = crate::Point::new(26, 2, false);
    pub const W_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(28, 2, false);
    pub const W_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(30, 2, false);
    pub const W_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(32, 2, false);
    pub const VA: crate::Point<Self, Option<f32>> = crate::Point::new(34, 2, false);
    pub const VA_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(36, 2, false);
    pub const VA_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(38, 2, false);
    pub const VA_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(40, 2, false);
    pub const VAR: crate::Point<Self, Option<f32>> = crate::Point::new(42, 2, false);
    pub const VAR_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(44, 2, false);
    pub const VAR_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(46, 2, false);
    pub const VAR_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(48, 2, false);
    pub const PF: crate::Point<Self, Option<f32>> = crate::Point::new(50, 2, false);
    pub const PF_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(52, 2, false);
    pub const PF_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(54, 2, false);
    pub const PF_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(56, 2, false);
    pub const TOT_WH_EXP: crate::Point<Self, f32> = crate::Point::new(58, 2, false);
    pub const TOT_WH_EXP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(60, 2, false);
    pub const TOT_WH_EXP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(62, 2, false);
    pub const TOT_WH_EXP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(64, 2, false);
    pub const TOT_WH_IMP: crate::Point<Self, f32> = crate::Point::new(66, 2, false);
    pub const TOT_WH_IMP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(68, 2, false);
    pub const TOT_WH_IMP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(70, 2, false);
    pub const TOT_WH_IMP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(72, 2, false);
    pub const TOT_VAH_EXP: crate::Point<Self, Option<f32>> = crate::Point::new(74, 2, false);
    pub const TOT_VAH_EXP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(76, 2, false);
    pub const TOT_VAH_EXP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(78, 2, false);
    pub const TOT_VAH_EXP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(80, 2, false);
    pub const TOT_VAH_IMP: crate::Point<Self, Option<f32>> = crate::Point::new(82, 2, false);
    pub const TOT_VAH_IMP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(84, 2, false);
    pub const TOT_VAH_IMP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(86, 2, false);
    pub const TOT_VAH_IMP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(88, 2, false);
    pub const TOT_VARH_IMP_Q1: crate::Point<Self, Option<f32>> = crate::Point::new(90, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(92, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(94, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(96, 2, false);
    pub const TOT_VARH_IMP_Q2: crate::Point<Self, Option<f32>> = crate::Point::new(98, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(100, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(102, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(104, 2, false);
    pub const TOT_VARH_EXP_Q3: crate::Point<Self, Option<f32>> = crate::Point::new(106, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(108, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(110, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(112, 2, false);
    pub const TOT_VARH_EXP_Q4: crate::Point<Self, Option<f32>> = crate::Point::new(114, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(116, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(118, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(120, 2, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(122, 2, false);
}
impl crate::sealed::Sealed for AcMeterAnOrAbFloat {}
impl crate::Group for AcMeterAnOrAbFloat {
    const LEN: u16 = 124;
    const GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
        name: "ac_meter_an_or_ab_float",
        label: "single phase (AN or AB) meter",
        description: "",
        fields: &[
            crate::FieldInfo {
                name: "a",
                label: "Amps",
                description: "Total AC Current",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "a_ph_a",
                label: "Amps PhaseA",
                description: "Phase A Current",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "a_ph_b",
                label: "Amps PhaseB",
                description: "Phase B Current",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "a_ph_c",
                label: "Amps PhaseC",
                description: "Phase C Current",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "ph_v",
                label: "Voltage LN",
                description: "Line to Neutral AC Voltage (average of active phases)",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "ph_v_ph_a",
                label: "Phase Voltage AN",
                description: "Phase Voltage AN",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "ph_v_ph_b",
                label: "Phase Voltage BN",
                description: "Phase Voltage BN",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "ph_v_ph_c",
                label: "Phase Voltage CN",
                description: "Phase Voltage CN",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pp_v",
                label: "Voltage LL",
                description: "Line to Line AC Voltage (average of active phases)",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pp_v_ph_ab",
                label: "Phase Voltage AB",
                description: "Phase Voltage AB",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pp_v_ph_bc",
                label: "Phase Voltage BC",
                description: "Phase Voltage BC",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pp_v_ph_ca",
                label: "Phase Voltage CA",
                description: "Phase Voltage CA",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "hz",
                label: "Hz",
                description: "Frequency",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "w",
                label: "Watts",
                description: "Total Real Power",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "w_ph_a",
                label: "Watts phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "w_ph_b",
                label: "Watts phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "w_ph_c",
                label: "Watts phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "va",
                label: "VA",
                description: "AC Apparent Power",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "va_ph_a",
                label: "VA phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "va_ph_b",
                label: "VA phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "va_ph_c",
                label: "VA phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var",
                label: "VAR",
                description: "Reactive Power",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var_ph_a",
                label: "VAR phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var_ph_b",
                label: "VAR phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "var_ph_c",
                label: "VAR phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pf",
                label: "PF",
                description: "Power Factor",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pf_ph_a",
                label: "PF phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pf_ph_b",
                label: "PF phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "pf_ph_c",
                label: "PF phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_exp",
                label: "Total Watt-hours Exported",
                description: "Total Real Energy Exported",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_exp_ph_a",
                label: "Total Watt-hours Exported phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_exp_ph_b",
                label: "Total Watt-hours Exported phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_exp_ph_c",
                label: "Total Watt-hours Exported phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_imp",
                label: "Total Watt-hours Imported",
                description: "Total Real Energy Imported",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_imp_ph_a",
                label: "Total Watt-hours Imported phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_imp_ph_b",
                label: "Total Watt-hours Imported phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_wh_imp_ph_c",
                label: "Total Watt-hours Imported phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_exp",
                label: "Total VA-hours Exported",
                description: "Total Apparent Energy Exported",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_exp_ph_a",
                label: "Total VA-hours Exported phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_exp_ph_b",
                label: "Total VA-hours Exported phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_exp_ph_c",
                label: "Total VA-hours Exported phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_imp",
                label: "Total VA-hours Imported",
                description: "Total Apparent Energy Imported",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_imp_ph_a",
                label: "Total VA-hours Imported phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_imp_ph_b",
                label: "Total VA-hours Imported phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_vah_imp_ph_c",
                label: "Total VA-hours Imported phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q1",
                label: "Total VAR-hours Imported Q1",
                description: "Total Reactive Energy Imported Quadrant 1",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q1_ph_a",
                label: "Total VAr-hours Imported Q1 phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q1_ph_b",
                label: "Total VAr-hours Imported Q1 phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q1_ph_c",
                label: "Total VAr-hours Imported Q1 phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q2",
                label: "Total VAr-hours Imported Q2",
                description: "Total Reactive Power Imported Quadrant 2",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q2_ph_a",
                label: "Total VAr-hours Imported Q2 phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q2_ph_b",
                label: "Total VAr-hours Imported Q2 phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_imp_q2_ph_c",
                label: "Total VAr-hours Imported Q2 phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q3",
                label: "Total VAr-hours Exported Q3",
                description: "Total Reactive Power Exported Quadrant 3",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q3_ph_a",
                label: "Total VAr-hours Exported Q3 phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q3_ph_b",
                label: "Total VAr-hours Exported Q3 phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q3_ph_c",
                label: "Total VAr-hours Exported Q3 phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q4",
                label: "Total VAr-hours Exported Q4",
                description: "Total Reactive Power Exported Quadrant 4",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q4_ph_a",
                label: "Total VAr-hours Exported Q4 Imported phase A",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q4_ph_b",
                label: "Total VAr-hours Exported Q4 Imported phase B",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "tot_varh_exp_q4_ph_c",
                label: "Total VAr-hours Exported Q4 Imported phase C",
                description: "",
                kind: crate::FieldKind::Point,
            },
            crate::FieldInfo {
                name: "evt",
                label: "Events",
                description: "Meter Event Flags",
                kind: crate::FieldKind::Point,
            },
        ],
    };
}
impl AcMeterAnOrAbFloat {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                a: Self::A.from_data(data)?,
                a_ph_a: Self::A_PH_A.from_data(data)?,
                a_ph_b: Self::A_PH_B.from_data(data)?,
                a_ph_c: Self::A_PH_C.from_data(data)?,
                ph_v: Self::PH_V.from_data(data)?,
                ph_v_ph_a: Self::PH_V_PH_A.from_data(data)?,
                ph_v_ph_b: Self::PH_V_PH_B.from_data(data)?,
                ph_v_ph_c: Self::PH_V_PH_C.from_data(data)?,
                pp_v: Self::PP_V.from_data(data)?,
                pp_v_ph_ab: Self::PP_V_PH_AB.from_data(data)?,
                pp_v_ph_bc: Self::PP_V_PH_BC.from_data(data)?,
                pp_v_ph_ca: Self::PP_V_PH_CA.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                w: Self::W.from_data(data)?,
                w_ph_a: Self::W_PH_A.from_data(data)?,
                w_ph_b: Self::W_PH_B.from_data(data)?,
                w_ph_c: Self::W_PH_C.from_data(data)?,
                va: Self::VA.from_data(data)?,
                va_ph_a: Self::VA_PH_A.from_data(data)?,
                va_ph_b: Self::VA_PH_B.from_data(data)?,
                va_ph_c: Self::VA_PH_C.from_data(data)?,
                var: Self::VAR.from_data(data)?,
                var_ph_a: Self::VAR_PH_A.from_data(data)?,
                var_ph_b: Self::VAR_PH_B.from_data(data)?,
                var_ph_c: Self::VAR_PH_C.from_data(data)?,
                pf: Self::PF.from_data(data)?,
                pf_ph_a: Self::PF_PH_A.from_data(data)?,
                pf_ph_b: Self::PF_PH_B.from_data(data)?,
                pf_ph_c: Self::PF_PH_C.from_data(data)?,
                tot_wh_exp: Self::TOT_WH_EXP.from_data(data)?,
                tot_wh_exp_ph_a: Self::TOT_WH_EXP_PH_A.from_data(data)?,
                tot_wh_exp_ph_b: Self::TOT_WH_EXP_PH_B.from_data(data)?,
                tot_wh_exp_ph_c: Self::TOT_WH_EXP_PH_C.from_data(data)?,
                tot_wh_imp: Self::TOT_WH_IMP.from_data(data)?,
                tot_wh_imp_ph_a: Self::TOT_WH_IMP_PH_A.from_data(data)?,
                tot_wh_imp_ph_b: Self::TOT_WH_IMP_PH_B.from_data(data)?,
                tot_wh_imp_ph_c: Self::TOT_WH_IMP_PH_C.from_data(data)?,
                tot_vah_exp: Self::TOT_VAH_EXP.from_data(data)?,
                tot_vah_exp_ph_a: Self::TOT_VAH_EXP_PH_A.from_data(data)?,
                tot_vah_exp_ph_b: Self::TOT_VAH_EXP_PH_B.from_data(data)?,
                tot_vah_exp_ph_c: Self::TOT_VAH_EXP_PH_C.from_data(data)?,
                tot_vah_imp: Self::TOT_VAH_IMP.from_data(data)?,
                tot_vah_imp_ph_a: Self::TOT_VAH_IMP_PH_A.from_data(data)?,
                tot_vah_imp_ph_b: Self::TOT_VAH_IMP_PH_B.from_data(data)?,
                tot_vah_imp_ph_c: Self::TOT_VAH_IMP_PH_C.from_data(data)?,
                tot_varh_imp_q1: Self::TOT_VARH_IMP_Q1.from_data(data)?,
                tot_varh_imp_q1_ph_a: Self::TOT_VARH_IMP_Q1_PH_A.from_data(data)?,
                tot_varh_imp_q1_ph_b: Self::TOT_VARH_IMP_Q1_PH_B.from_data(data)?,
                tot_varh_imp_q1_ph_c: Self::TOT_VARH_IMP_Q1_PH_C.from_data(data)?,
                tot_varh_imp_q2: Self::TOT_VARH_IMP_Q2.from_data(data)?,
                tot_varh_imp_q2_ph_a: Self::TOT_VARH_IMP_Q2_PH_A.from_data(data)?,
                tot_varh_imp_q2_ph_b: Self::TOT_VARH_IMP_Q2_PH_B.from_data(data)?,
                tot_varh_imp_q2_ph_c: Self::TOT_VARH_IMP_Q2_PH_C.from_data(data)?,
                tot_varh_exp_q3: Self::TOT_VARH_EXP_Q3.from_data(data)?,
                tot_varh_exp_q3_ph_a: Self::TOT_VARH_EXP_Q3_PH_A.from_data(data)?,
                tot_varh_exp_q3_ph_b: Self::TOT_VARH_EXP_Q3_PH_B.from_data(data)?,
                tot_varh_exp_q3_ph_c: Self::TOT_VARH_EXP_Q3_PH_C.from_data(data)?,
                tot_varh_exp_q4: Self::TOT_VARH_EXP_Q4.from_data(data)?,
                tot_varh_exp_q4_ph_a: Self::TOT_VARH_EXP_Q4_PH_A.from_data(data)?,
                tot_varh_exp_q4_ph_b: Self::TOT_VARH_EXP_Q4_PH_B.from_data(data)?,
                tot_varh_exp_q4_ph_c: Self::TOT_VARH_EXP_Q4_PH_C.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    /// Events
    ///
    /// Meter Event Flags
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Evt: u32 {
        #[allow(missing_docs)]
        const MEventPowerFailure = 4;
        #[allow(missing_docs)]
        const MEventUnderVoltage = 8;
        #[allow(missing_docs)]
        const MEventLowPf = 16;
        #[allow(missing_docs)]
        const MEventOverCurrent = 32;
        #[allow(missing_docs)]
        const MEventOverVoltage = 64;
        #[allow(missing_docs)]
        const MEventMissingSensor = 128;
        #[allow(missing_docs)]
        const MEventReserved1 = 256;
        #[allow(missing_docs)]
        const MEventReserved2 = 512;
        #[allow(missing_docs)]
        const MEventReserved3 = 1024;
        #[allow(missing_docs)]
        const MEventReserved4 = 2048;
        #[allow(missing_docs)]
        const MEventReserved5 = 4096;
        #[allow(missing_docs)]
        const MEventReserved6 = 8192;
        #[allow(missing_docs)]
        const MEventReserved7 = 16384;
        #[allow(missing_docs)]
        const MEventReserved8 = 32768;
        #[allow(missing_docs)]
        const MEventOem01 = 65536;
        #[allow(missing_docs)]
        const MEventOem02 = 131072;
        #[allow(missing_docs)]
        const MEventOem03 = 262144;
        #[allow(missing_docs)]
        const MEventOem04 = 524288;
        #[allow(missing_docs)]
        const MEventOem05 = 1048576;
        #[allow(missing_docs)]
        const MEventOem06 = 2097152;
        #[allow(missing_docs)]
        const MEventOem07 = 4194304;
        #[allow(missing_docs)]
        const MEventOem08 = 8388608;
        #[allow(missing_docs)]
        const MEventOem09 = 16777216;
        #[allow(missing_docs)]
        const MEventOem10 = 33554432;
        #[allow(missing_docs)]
        const MEventOem11 = 67108864;
        #[allow(missing_docs)]
        const MEventOem12 = 134217728;
        #[allow(missing_docs)]
        const MEventOem13 = 268435456;
        #[allow(missing_docs)]
        const MEventOem14 = 536870912;
        #[allow(missing_docs)]
        const MEventOem15 = 1073741824;
    }
}
impl crate::sealed::Sealed for Evt {}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl From<AcMeterAnOrAbFloat> for crate::AnyModel {
    fn from(model: AcMeterAnOrAbFloat) -> Self {
        Self::M211(model)
    }
}
impl crate::Model for AcMeterAnOrAbFloat {
    const ID: u16 = 211;
    const NAME: &'static str = "ac_meter_an_or_ab_float";
    const LABEL: &'static str = "single phase (AN or AB) meter";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m211
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
