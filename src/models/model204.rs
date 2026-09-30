//! delta-connect three phase (abc) meter
/// Type alias for [`AcMeterAbc`].
pub type Model204 = AcMeterAbc;
/// delta-connect three phase (abc) meter
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AcMeterAbc {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub a_ph_a: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub a_ph_b: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub a_ph_c: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub ph_v: Option<i16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_v_ph_a: Option<i16>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_v_ph_b: Option<i16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_v_ph_c: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub pp_v: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ph_v_ph_ab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ph_v_ph_bc: i16,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ph_v_ph_ca: i16,
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
    pub w_ph_a: Option<i16>,
    /// Watts phase B
    pub w_ph_b: Option<i16>,
    /// Watts phase C
    pub w_ph_c: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub va_ph_a: Option<i16>,
    /// VA phase B
    pub va_ph_b: Option<i16>,
    /// VA phase C
    pub va_ph_c: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub var_ph_a: Option<i16>,
    /// VAR phase B
    pub var_ph_b: Option<i16>,
    /// VAR phase C
    pub var_ph_c: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pf_ph_a: Option<i16>,
    /// PF phase B
    pub pf_ph_b: Option<i16>,
    /// PF phase C
    pub pf_ph_c: Option<i16>,
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
    pub tot_vah_exp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub tot_vah_exp_ph_a: Option<u32>,
    /// Total VA-hours Exported phase B
    pub tot_vah_exp_ph_b: Option<u32>,
    /// Total VA-hours Exported phase C
    pub tot_vah_exp_ph_c: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_vah_imp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub tot_vah_imp_ph_a: Option<u32>,
    /// Total VA-hours Imported phase B
    pub tot_vah_imp_ph_b: Option<u32>,
    /// Total VA-hours Imported phase C
    pub tot_vah_imp_ph_c: Option<u32>,
    /// Apparent Energy scale factor
    pub tot_vah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_varh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub tot_varh_imp_q1_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub tot_varh_imp_q1_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub tot_varh_imp_q1_ph_c: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_varh_imp_q2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub tot_varh_imp_q2_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub tot_varh_imp_q2_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub tot_varh_imp_q2_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_varh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub tot_varh_exp_q3_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub tot_varh_exp_q3_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub tot_varh_exp_q3_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_varh_exp_q4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub tot_varh_exp_q4_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub tot_varh_exp_q4_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub tot_varh_exp_q4_ph_c: Option<u32>,
    /// Reactive Energy scale factor
    pub tot_varh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: Evt,
}
#[allow(missing_docs)]
impl AcMeterAbc {
    pub const A: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const A_PH_A: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const A_PH_B: crate::Point<Self, i16> = crate::Point::new(2, 1, false);
    pub const A_PH_C: crate::Point<Self, i16> = crate::Point::new(3, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const PH_V: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, false);
    pub const PH_V_PH_A: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, false);
    pub const PH_V_PH_B: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, false);
    pub const PH_V_PH_C: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const PP_V: crate::Point<Self, i16> = crate::Point::new(9, 1, false);
    pub const PH_V_PH_AB: crate::Point<Self, i16> = crate::Point::new(10, 1, false);
    pub const PH_V_PH_BC: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const PH_V_PH_CA: crate::Point<Self, i16> = crate::Point::new(12, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(13, 1, false);
    pub const HZ: crate::Point<Self, i16> = crate::Point::new(14, 1, false);
    pub const HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const W: crate::Point<Self, i16> = crate::Point::new(16, 1, false);
    pub const W_PH_A: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, false);
    pub const W_PH_B: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, false);
    pub const W_PH_C: crate::Point<Self, Option<i16>> = crate::Point::new(19, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(20, 1, false);
    pub const VA: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, false);
    pub const VA_PH_A: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, false);
    pub const VA_PH_B: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const VA_PH_C: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, false);
    pub const VA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, false);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(26, 1, false);
    pub const VAR_PH_A: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, false);
    pub const VAR_PH_B: crate::Point<Self, Option<i16>> = crate::Point::new(28, 1, false);
    pub const VAR_PH_C: crate::Point<Self, Option<i16>> = crate::Point::new(29, 1, false);
    pub const VAR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(30, 1, false);
    pub const PF: crate::Point<Self, Option<i16>> = crate::Point::new(31, 1, false);
    pub const PF_PH_A: crate::Point<Self, Option<i16>> = crate::Point::new(32, 1, false);
    pub const PF_PH_B: crate::Point<Self, Option<i16>> = crate::Point::new(33, 1, false);
    pub const PF_PH_C: crate::Point<Self, Option<i16>> = crate::Point::new(34, 1, false);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(35, 1, false);
    pub const TOT_WH_EXP: crate::Point<Self, u32> = crate::Point::new(36, 2, false);
    pub const TOT_WH_EXP_PH_A: crate::Point<Self, Option<u32>> = crate::Point::new(38, 2, false);
    pub const TOT_WH_EXP_PH_B: crate::Point<Self, Option<u32>> = crate::Point::new(40, 2, false);
    pub const TOT_WH_EXP_PH_C: crate::Point<Self, Option<u32>> = crate::Point::new(42, 2, false);
    pub const TOT_WH_IMP: crate::Point<Self, u32> = crate::Point::new(44, 2, false);
    pub const TOT_WH_IMP_PH_A: crate::Point<Self, Option<u32>> = crate::Point::new(46, 2, false);
    pub const TOT_WH_IMP_PH_B: crate::Point<Self, Option<u32>> = crate::Point::new(48, 2, false);
    pub const TOT_WH_IMP_PH_C: crate::Point<Self, Option<u32>> = crate::Point::new(50, 2, false);
    pub const TOT_WH_SF: crate::Point<Self, i16> = crate::Point::new(52, 1, false);
    pub const TOT_VAH_EXP: crate::Point<Self, Option<u32>> = crate::Point::new(53, 2, false);
    pub const TOT_VAH_EXP_PH_A: crate::Point<Self, Option<u32>> = crate::Point::new(55, 2, false);
    pub const TOT_VAH_EXP_PH_B: crate::Point<Self, Option<u32>> = crate::Point::new(57, 2, false);
    pub const TOT_VAH_EXP_PH_C: crate::Point<Self, Option<u32>> = crate::Point::new(59, 2, false);
    pub const TOT_VAH_IMP: crate::Point<Self, Option<u32>> = crate::Point::new(61, 2, false);
    pub const TOT_VAH_IMP_PH_A: crate::Point<Self, Option<u32>> = crate::Point::new(63, 2, false);
    pub const TOT_VAH_IMP_PH_B: crate::Point<Self, Option<u32>> = crate::Point::new(65, 2, false);
    pub const TOT_VAH_IMP_PH_C: crate::Point<Self, Option<u32>> = crate::Point::new(67, 2, false);
    pub const TOT_VAH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(69, 1, false);
    pub const TOT_VARH_IMP_Q1: crate::Point<Self, Option<u32>> = crate::Point::new(70, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_A: crate::Point<Self, Option<u32>> =
        crate::Point::new(72, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_B: crate::Point<Self, Option<u32>> =
        crate::Point::new(74, 2, false);
    pub const TOT_VARH_IMP_Q1_PH_C: crate::Point<Self, Option<u32>> =
        crate::Point::new(76, 2, false);
    pub const TOT_VARH_IMP_Q2: crate::Point<Self, Option<u32>> = crate::Point::new(78, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_A: crate::Point<Self, Option<u32>> =
        crate::Point::new(80, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_B: crate::Point<Self, Option<u32>> =
        crate::Point::new(82, 2, false);
    pub const TOT_VARH_IMP_Q2_PH_C: crate::Point<Self, Option<u32>> =
        crate::Point::new(84, 2, false);
    pub const TOT_VARH_EXP_Q3: crate::Point<Self, Option<u32>> = crate::Point::new(86, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_A: crate::Point<Self, Option<u32>> =
        crate::Point::new(88, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_B: crate::Point<Self, Option<u32>> =
        crate::Point::new(90, 2, false);
    pub const TOT_VARH_EXP_Q3_PH_C: crate::Point<Self, Option<u32>> =
        crate::Point::new(92, 2, false);
    pub const TOT_VARH_EXP_Q4: crate::Point<Self, Option<u32>> = crate::Point::new(94, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_A: crate::Point<Self, Option<u32>> =
        crate::Point::new(96, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_B: crate::Point<Self, Option<u32>> =
        crate::Point::new(98, 2, false);
    pub const TOT_VARH_EXP_Q4_PH_C: crate::Point<Self, Option<u32>> =
        crate::Point::new(100, 2, false);
    pub const TOT_VARH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(102, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(103, 2, false);
}
impl crate::sealed::Sealed for AcMeterAbc {}
impl crate::Group for AcMeterAbc {
    const LEN: u16 = 105;
}
impl AcMeterAbc {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(105..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                a: Self::A.from_data(data)?,
                a_ph_a: Self::A_PH_A.from_data(data)?,
                a_ph_b: Self::A_PH_B.from_data(data)?,
                a_ph_c: Self::A_PH_C.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                ph_v: Self::PH_V.from_data(data)?,
                ph_v_ph_a: Self::PH_V_PH_A.from_data(data)?,
                ph_v_ph_b: Self::PH_V_PH_B.from_data(data)?,
                ph_v_ph_c: Self::PH_V_PH_C.from_data(data)?,
                pp_v: Self::PP_V.from_data(data)?,
                ph_v_ph_ab: Self::PH_V_PH_AB.from_data(data)?,
                ph_v_ph_bc: Self::PH_V_PH_BC.from_data(data)?,
                ph_v_ph_ca: Self::PH_V_PH_CA.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
                w: Self::W.from_data(data)?,
                w_ph_a: Self::W_PH_A.from_data(data)?,
                w_ph_b: Self::W_PH_B.from_data(data)?,
                w_ph_c: Self::W_PH_C.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
                va: Self::VA.from_data(data)?,
                va_ph_a: Self::VA_PH_A.from_data(data)?,
                va_ph_b: Self::VA_PH_B.from_data(data)?,
                va_ph_c: Self::VA_PH_C.from_data(data)?,
                va_sf: Self::VA_SF.from_data(data)?,
                var: Self::VAR.from_data(data)?,
                var_ph_a: Self::VAR_PH_A.from_data(data)?,
                var_ph_b: Self::VAR_PH_B.from_data(data)?,
                var_ph_c: Self::VAR_PH_C.from_data(data)?,
                var_sf: Self::VAR_SF.from_data(data)?,
                pf: Self::PF.from_data(data)?,
                pf_ph_a: Self::PF_PH_A.from_data(data)?,
                pf_ph_b: Self::PF_PH_B.from_data(data)?,
                pf_ph_c: Self::PF_PH_C.from_data(data)?,
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
                tot_vah_exp: Self::TOT_VAH_EXP.from_data(data)?,
                tot_vah_exp_ph_a: Self::TOT_VAH_EXP_PH_A.from_data(data)?,
                tot_vah_exp_ph_b: Self::TOT_VAH_EXP_PH_B.from_data(data)?,
                tot_vah_exp_ph_c: Self::TOT_VAH_EXP_PH_C.from_data(data)?,
                tot_vah_imp: Self::TOT_VAH_IMP.from_data(data)?,
                tot_vah_imp_ph_a: Self::TOT_VAH_IMP_PH_A.from_data(data)?,
                tot_vah_imp_ph_b: Self::TOT_VAH_IMP_PH_B.from_data(data)?,
                tot_vah_imp_ph_c: Self::TOT_VAH_IMP_PH_C.from_data(data)?,
                tot_vah_sf: Self::TOT_VAH_SF.from_data(data)?,
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
                tot_varh_sf: Self::TOT_VARH_SF.from_data(data)?,
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
impl From<AcMeterAbc> for crate::AnyModel {
    fn from(model: AcMeterAbc) -> Self {
        Self::M204(model)
    }
}
impl crate::Model for AcMeterAbc {
    const ID: u16 = 204;
    const NAME: &'static str = "ac_meter_abc";
    const LABEL: &'static str = "delta-connect three phase (abc) meter";
    fn addr(models: &crate::Models) -> Option<crate::ModelAddr<Self>> {
        models.m204
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
