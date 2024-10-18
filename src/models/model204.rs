//! delta-connect three phase (abc) meter
/// delta-connect three phase (abc) meter
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model204 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub aph_a: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aph_b: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aph_c: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub ph_v: Option<i16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_vph_a: Option<i16>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_vph_b: Option<i16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_vph_c: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ph_vph_ab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ph_vph_bc: i16,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ph_vph_ca: i16,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Watts phase A
    pub wph_a: Option<i16>,
    /// Watts phase B
    pub wph_b: Option<i16>,
    /// Watts phase C
    pub wph_c: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub v_aph_a: Option<i16>,
    /// VA phase B
    pub v_aph_b: Option<i16>,
    /// VA phase C
    pub v_aph_c: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub va_rph_a: Option<i16>,
    /// VAR phase B
    pub va_rph_b: Option<i16>,
    /// VAR phase C
    pub va_rph_c: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub p_fph_a: Option<i16>,
    /// PF phase B
    pub p_fph_b: Option<i16>,
    /// PF phase C
    pub p_fph_c: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub tot_wh_exp: u32,
    /// Total Watt-hours Exported phase A
    pub tot_wh_exp_ph_a: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub tot_wh_exp_ph_b: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub tot_wh_exp_ph_c: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub tot_wh_imp: u32,
    /// Total Watt-hours Imported phase A
    pub tot_wh_imp_ph_a: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub tot_wh_imp_ph_b: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub tot_wh_imp_ph_c: Option<u32>,
    /// Real Energy scale factor
    pub tot_wh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub tot_v_ah_exp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub tot_v_ah_exp_ph_a: Option<u32>,
    /// Total VA-hours Exported phase B
    pub tot_v_ah_exp_ph_b: Option<u32>,
    /// Total VA-hours Exported phase C
    pub tot_v_ah_exp_ph_c: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_v_ah_imp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub tot_v_ah_imp_ph_a: Option<u32>,
    /// Total VA-hours Imported phase B
    pub tot_v_ah_imp_ph_b: Option<u32>,
    /// Total VA-hours Imported phase C
    pub tot_v_ah_imp_ph_c: Option<u32>,
    /// Apparent Energy scale factor
    pub tot_v_ah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_v_arh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub tot_v_arh_imp_q1_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub tot_v_arh_imp_q1_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub tot_v_arh_imp_q1_ph_c: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_v_arh_imp_q2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub tot_v_arh_imp_q2_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub tot_v_arh_imp_q2_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub tot_v_arh_imp_q2_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_v_arh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub tot_v_arh_exp_q3_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub tot_v_arh_exp_q3_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub tot_v_arh_exp_q3_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_v_arh_exp_q4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub tot_v_arh_exp_q4_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub tot_v_arh_exp_q4_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub tot_v_arh_exp_q4_ph_c: Option<u32>,
    /// Reactive Energy scale factor
    pub tot_v_arh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: Evt,
}
#[allow(missing_docs)]
impl Model204 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APH_A: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APH_B: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APH_C: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PH_V: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(5, 1, false);
    pub const PH_VPH_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(6, 1, false);
    pub const PH_VPH_B: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, false);
    pub const PH_VPH_C: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PH_VPH_AB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PH_VPH_BC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PH_VPH_CA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPH_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
    pub const WPH_B: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const WPH_C: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const V_APH_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const V_APH_B: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const V_APH_C: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(26, 1, false);
    pub const VA_RPH_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, false);
    pub const VA_RPH_B: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(28, 1, false);
    pub const VA_RPH_C: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(31, 1, false);
    pub const P_FPH_A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(32, 1, false);
    pub const P_FPH_B: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(33, 1, false);
    pub const P_FPH_C: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(35, 1, false);
    pub const TOT_WH_EXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOT_WH_EXP_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(38, 2, false);
    pub const TOT_WH_EXP_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(40, 2, false);
    pub const TOT_WH_EXP_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(42, 2, false);
    pub const TOT_WH_IMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOT_WH_IMP_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(46, 2, false);
    pub const TOT_WH_IMP_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(48, 2, false);
    pub const TOT_WH_IMP_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(50, 2, false);
    pub const TOT_WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOT_V_AH_EXP: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(53, 2, false);
    pub const TOT_V_AH_EXP_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(55, 2, false);
    pub const TOT_V_AH_EXP_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(57, 2, false);
    pub const TOT_V_AH_EXP_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(59, 2, false);
    pub const TOT_V_AH_IMP: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(61, 2, false);
    pub const TOT_V_AH_IMP_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(63, 2, false);
    pub const TOT_V_AH_IMP_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(65, 2, false);
    pub const TOT_V_AH_IMP_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(67, 2, false);
    pub const TOT_V_AH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(69, 1, false);
    pub const TOT_V_ARH_IMP_Q1: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(70, 2, false);
    pub const TOT_V_ARH_IMP_Q1_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(72, 2, false);
    pub const TOT_V_ARH_IMP_Q1_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(74, 2, false);
    pub const TOT_V_ARH_IMP_Q1_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(76, 2, false);
    pub const TOT_V_ARH_IMP_Q2: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(78, 2, false);
    pub const TOT_V_ARH_IMP_Q2_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(80, 2, false);
    pub const TOT_V_ARH_IMP_Q2_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(82, 2, false);
    pub const TOT_V_ARH_IMP_Q2_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(84, 2, false);
    pub const TOT_V_ARH_EXP_Q3: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(86, 2, false);
    pub const TOT_V_ARH_EXP_Q3_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(88, 2, false);
    pub const TOT_V_ARH_EXP_Q3_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(90, 2, false);
    pub const TOT_V_ARH_EXP_Q3_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(92, 2, false);
    pub const TOT_V_ARH_EXP_Q4: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(94, 2, false);
    pub const TOT_V_ARH_EXP_Q4_PH_A: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(96, 2, false);
    pub const TOT_V_ARH_EXP_Q4_PH_B: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(98, 2, false);
    pub const TOT_V_ARH_EXP_Q4_PH_C: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(100, 2, false);
    pub const TOT_V_ARH_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, Evt> = crate::PointDef::new(103, 2, false);
}
impl crate::Model for Model204 {
    const ID: u16 = 204;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            aph_a: Self::APH_A.from_data(data)?,
            aph_b: Self::APH_B.from_data(data)?,
            aph_c: Self::APH_C.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            ph_v: Self::PH_V.from_data(data)?,
            ph_vph_a: Self::PH_VPH_A.from_data(data)?,
            ph_vph_b: Self::PH_VPH_B.from_data(data)?,
            ph_vph_c: Self::PH_VPH_C.from_data(data)?,
            ppv: Self::PPV.from_data(data)?,
            ph_vph_ab: Self::PH_VPH_AB.from_data(data)?,
            ph_vph_bc: Self::PH_VPH_BC.from_data(data)?,
            ph_vph_ca: Self::PH_VPH_CA.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W.from_data(data)?,
            wph_a: Self::WPH_A.from_data(data)?,
            wph_b: Self::WPH_B.from_data(data)?,
            wph_c: Self::WPH_C.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            va: Self::VA.from_data(data)?,
            v_aph_a: Self::V_APH_A.from_data(data)?,
            v_aph_b: Self::V_APH_B.from_data(data)?,
            v_aph_c: Self::V_APH_C.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            va_rph_a: Self::VA_RPH_A.from_data(data)?,
            va_rph_b: Self::VA_RPH_B.from_data(data)?,
            va_rph_c: Self::VA_RPH_C.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            p_fph_a: Self::P_FPH_A.from_data(data)?,
            p_fph_b: Self::P_FPH_B.from_data(data)?,
            p_fph_c: Self::P_FPH_C.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            tot_wh_exp: Self::TOT_WH_EXP.from_data(data)?,
            tot_wh_exp_ph_a: Self::TOT_WH_EXP_PH_A.from_data(data)?,
            tot_wh_exp_ph_b: Self::TOT_WH_EXP_PH_B.from_data(data)?,
            tot_wh_exp_ph_c: Self::TOT_WH_EXP_PH_C.from_data(data)?,
            tot_wh_imp: Self::TOT_WH_IMP.from_data(data)?,
            tot_wh_imp_ph_a: Self::TOT_WH_IMP_PH_A.from_data(data)?,
            tot_wh_imp_ph_b: Self::TOT_WH_IMP_PH_B.from_data(data)?,
            tot_wh_imp_ph_c: Self::TOT_WH_IMP_PH_C.from_data(data)?,
            tot_wh_sf: Self::TOT_WH_SF.from_data(data)?,
            tot_v_ah_exp: Self::TOT_V_AH_EXP.from_data(data)?,
            tot_v_ah_exp_ph_a: Self::TOT_V_AH_EXP_PH_A.from_data(data)?,
            tot_v_ah_exp_ph_b: Self::TOT_V_AH_EXP_PH_B.from_data(data)?,
            tot_v_ah_exp_ph_c: Self::TOT_V_AH_EXP_PH_C.from_data(data)?,
            tot_v_ah_imp: Self::TOT_V_AH_IMP.from_data(data)?,
            tot_v_ah_imp_ph_a: Self::TOT_V_AH_IMP_PH_A.from_data(data)?,
            tot_v_ah_imp_ph_b: Self::TOT_V_AH_IMP_PH_B.from_data(data)?,
            tot_v_ah_imp_ph_c: Self::TOT_V_AH_IMP_PH_C.from_data(data)?,
            tot_v_ah_sf: Self::TOT_V_AH_SF.from_data(data)?,
            tot_v_arh_imp_q1: Self::TOT_V_ARH_IMP_Q1.from_data(data)?,
            tot_v_arh_imp_q1_ph_a: Self::TOT_V_ARH_IMP_Q1_PH_A.from_data(data)?,
            tot_v_arh_imp_q1_ph_b: Self::TOT_V_ARH_IMP_Q1_PH_B.from_data(data)?,
            tot_v_arh_imp_q1_ph_c: Self::TOT_V_ARH_IMP_Q1_PH_C.from_data(data)?,
            tot_v_arh_imp_q2: Self::TOT_V_ARH_IMP_Q2.from_data(data)?,
            tot_v_arh_imp_q2_ph_a: Self::TOT_V_ARH_IMP_Q2_PH_A.from_data(data)?,
            tot_v_arh_imp_q2_ph_b: Self::TOT_V_ARH_IMP_Q2_PH_B.from_data(data)?,
            tot_v_arh_imp_q2_ph_c: Self::TOT_V_ARH_IMP_Q2_PH_C.from_data(data)?,
            tot_v_arh_exp_q3: Self::TOT_V_ARH_EXP_Q3.from_data(data)?,
            tot_v_arh_exp_q3_ph_a: Self::TOT_V_ARH_EXP_Q3_PH_A.from_data(data)?,
            tot_v_arh_exp_q3_ph_b: Self::TOT_V_ARH_EXP_Q3_PH_B.from_data(data)?,
            tot_v_arh_exp_q3_ph_c: Self::TOT_V_ARH_EXP_Q3_PH_C.from_data(data)?,
            tot_v_arh_exp_q4: Self::TOT_V_ARH_EXP_Q4.from_data(data)?,
            tot_v_arh_exp_q4_ph_a: Self::TOT_V_ARH_EXP_Q4_PH_A.from_data(data)?,
            tot_v_arh_exp_q4_ph_b: Self::TOT_V_ARH_EXP_Q4_PH_B.from_data(data)?,
            tot_v_arh_exp_q4_ph_c: Self::TOT_V_ARH_EXP_Q4_PH_C.from_data(data)?,
            tot_v_arh_sf: Self::TOT_V_ARH_SF.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m204
    }
}
bitflags::bitflags! {
    #[doc = " Events"] #[doc = " "] #[doc = " Meter Event Flags"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Evt : u32 { #[allow(missing_docs)] const
    MEventPowerFailure = 4; #[allow(missing_docs)] const MEventUnderVoltage = 8;
    #[allow(missing_docs)] const MEventLowPf = 16; #[allow(missing_docs)] const
    MEventOverCurrent = 32; #[allow(missing_docs)] const MEventOverVoltage = 64;
    #[allow(missing_docs)] const MEventMissingSensor = 128; #[allow(missing_docs)] const
    MEventReserved1 = 256; #[allow(missing_docs)] const MEventReserved2 = 512;
    #[allow(missing_docs)] const MEventReserved3 = 1024; #[allow(missing_docs)] const
    MEventReserved4 = 2048; #[allow(missing_docs)] const MEventReserved5 = 4096;
    #[allow(missing_docs)] const MEventReserved6 = 8192; #[allow(missing_docs)] const
    MEventReserved7 = 16384; #[allow(missing_docs)] const MEventReserved8 = 32768;
    #[allow(missing_docs)] const MEventOem01 = 65536; #[allow(missing_docs)] const
    MEventOem02 = 131072; #[allow(missing_docs)] const MEventOem03 = 262144;
    #[allow(missing_docs)] const MEventOem04 = 524288; #[allow(missing_docs)] const
    MEventOem05 = 1048576; #[allow(missing_docs)] const MEventOem06 = 2097152;
    #[allow(missing_docs)] const MEventOem07 = 4194304; #[allow(missing_docs)] const
    MEventOem08 = 8388608; #[allow(missing_docs)] const MEventOem09 = 16777216;
    #[allow(missing_docs)] const MEventOem10 = 33554432; #[allow(missing_docs)] const
    MEventOem11 = 67108864; #[allow(missing_docs)] const MEventOem12 = 134217728;
    #[allow(missing_docs)] const MEventOem13 = 268435456; #[allow(missing_docs)] const
    MEventOem14 = 536870912; #[allow(missing_docs)] const MEventOem15 = 1073741824; }
}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Evt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        if value != 4294967295u32 {
            Ok(Some(Evt::from_bits_retain(value)))
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
