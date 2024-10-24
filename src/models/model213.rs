//! wye-connect three phase (abcn) meter
/// wye-connect three phase (abcn) meter
///
/// Notes: Float
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model213 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub aph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aph_b: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aph_c: f32,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub ph_v: f32,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub ph_vph_a: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub ph_vph_b: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub ph_vph_c: f32,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub pp_vph_ab: f32,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub pp_vph_bc: f32,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub pp_vph_ca: f32,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wph_a: Option<f32>,
    /// Watts phase B
    pub wph_b: Option<f32>,
    /// Watts phase C
    pub wph_c: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub v_aph_a: Option<f32>,
    /// VA phase B
    pub v_aph_b: Option<f32>,
    /// VA phase C
    pub v_aph_c: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub va_rph_a: Option<f32>,
    /// VAR phase B
    pub va_rph_b: Option<f32>,
    /// VAR phase C
    pub va_rph_c: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub p_fph_a: Option<f32>,
    /// PF phase B
    pub p_fph_b: Option<f32>,
    /// PF phase C
    pub p_fph_c: Option<f32>,
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
    pub tot_v_ah_exp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub tot_v_ah_exp_ph_a: Option<f32>,
    /// Total VA-hours Exported phase B
    pub tot_v_ah_exp_ph_b: Option<f32>,
    /// Total VA-hours Exported phase C
    pub tot_v_ah_exp_ph_c: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_v_ah_imp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub tot_v_ah_imp_ph_a: Option<f32>,
    /// Total VA-hours Imported phase B
    pub tot_v_ah_imp_ph_b: Option<f32>,
    /// Total VA-hours Imported phase C
    pub tot_v_ah_imp_ph_c: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_v_arh_imp_q1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub tot_v_arh_imp_q1ph_a: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub tot_v_arh_imp_q1ph_b: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub tot_v_arh_imp_q1ph_c: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_v_arh_imp_q2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub tot_v_arh_imp_q2ph_a: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub tot_v_arh_imp_q2ph_b: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub tot_v_arh_imp_q2ph_c: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_v_arh_exp_q3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub tot_v_arh_exp_q3ph_a: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub tot_v_arh_exp_q3ph_b: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub tot_v_arh_exp_q3ph_c: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_v_arh_exp_q4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub tot_v_arh_exp_q4ph_a: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub tot_v_arh_exp_q4ph_b: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub tot_v_arh_exp_q4ph_c: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: Evt,
}
#[allow(missing_docs)]
impl Model213 {
    pub const A: crate::Point<Self, f32> = crate::Point::new(0, 2, false);
    pub const APH_A: crate::Point<Self, f32> = crate::Point::new(2, 2, false);
    pub const APH_B: crate::Point<Self, f32> = crate::Point::new(4, 2, false);
    pub const APH_C: crate::Point<Self, f32> = crate::Point::new(6, 2, false);
    pub const PH_V: crate::Point<Self, f32> = crate::Point::new(8, 2, false);
    pub const PH_VPH_A: crate::Point<Self, f32> = crate::Point::new(10, 2, false);
    pub const PH_VPH_B: crate::Point<Self, f32> = crate::Point::new(12, 2, false);
    pub const PH_VPH_C: crate::Point<Self, f32> = crate::Point::new(14, 2, false);
    pub const PPV: crate::Point<Self, f32> = crate::Point::new(16, 2, false);
    pub const PP_VPH_AB: crate::Point<Self, f32> = crate::Point::new(18, 2, false);
    pub const PP_VPH_BC: crate::Point<Self, f32> = crate::Point::new(20, 2, false);
    pub const PP_VPH_CA: crate::Point<Self, f32> = crate::Point::new(22, 2, false);
    pub const HZ: crate::Point<Self, f32> = crate::Point::new(24, 2, false);
    pub const W: crate::Point<Self, f32> = crate::Point::new(26, 2, false);
    pub const WPH_A: crate::Point<Self, Option<f32>> = crate::Point::new(28, 2, false);
    pub const WPH_B: crate::Point<Self, Option<f32>> = crate::Point::new(30, 2, false);
    pub const WPH_C: crate::Point<Self, Option<f32>> = crate::Point::new(32, 2, false);
    pub const VA: crate::Point<Self, Option<f32>> = crate::Point::new(34, 2, false);
    pub const V_APH_A: crate::Point<Self, Option<f32>> = crate::Point::new(36, 2, false);
    pub const V_APH_B: crate::Point<Self, Option<f32>> = crate::Point::new(38, 2, false);
    pub const V_APH_C: crate::Point<Self, Option<f32>> = crate::Point::new(40, 2, false);
    pub const VAR: crate::Point<Self, Option<f32>> = crate::Point::new(42, 2, false);
    pub const VA_RPH_A: crate::Point<Self, Option<f32>> = crate::Point::new(44, 2, false);
    pub const VA_RPH_B: crate::Point<Self, Option<f32>> = crate::Point::new(46, 2, false);
    pub const VA_RPH_C: crate::Point<Self, Option<f32>> = crate::Point::new(48, 2, false);
    pub const PF: crate::Point<Self, Option<f32>> = crate::Point::new(50, 2, false);
    pub const P_FPH_A: crate::Point<Self, Option<f32>> = crate::Point::new(52, 2, false);
    pub const P_FPH_B: crate::Point<Self, Option<f32>> = crate::Point::new(54, 2, false);
    pub const P_FPH_C: crate::Point<Self, Option<f32>> = crate::Point::new(56, 2, false);
    pub const TOT_WH_EXP: crate::Point<Self, f32> = crate::Point::new(58, 2, false);
    pub const TOT_WH_EXP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(60, 2, false);
    pub const TOT_WH_EXP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(62, 2, false);
    pub const TOT_WH_EXP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(64, 2, false);
    pub const TOT_WH_IMP: crate::Point<Self, f32> = crate::Point::new(66, 2, false);
    pub const TOT_WH_IMP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(68, 2, false);
    pub const TOT_WH_IMP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(70, 2, false);
    pub const TOT_WH_IMP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(72, 2, false);
    pub const TOT_V_AH_EXP: crate::Point<Self, Option<f32>> = crate::Point::new(74, 2, false);
    pub const TOT_V_AH_EXP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(76, 2, false);
    pub const TOT_V_AH_EXP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(78, 2, false);
    pub const TOT_V_AH_EXP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(80, 2, false);
    pub const TOT_V_AH_IMP: crate::Point<Self, Option<f32>> = crate::Point::new(82, 2, false);
    pub const TOT_V_AH_IMP_PH_A: crate::Point<Self, Option<f32>> = crate::Point::new(84, 2, false);
    pub const TOT_V_AH_IMP_PH_B: crate::Point<Self, Option<f32>> = crate::Point::new(86, 2, false);
    pub const TOT_V_AH_IMP_PH_C: crate::Point<Self, Option<f32>> = crate::Point::new(88, 2, false);
    pub const TOT_V_ARH_IMP_Q1: crate::Point<Self, Option<f32>> = crate::Point::new(90, 2, false);
    pub const TOT_V_ARH_IMP_Q1PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(92, 2, false);
    pub const TOT_V_ARH_IMP_Q1PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(94, 2, false);
    pub const TOT_V_ARH_IMP_Q1PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(96, 2, false);
    pub const TOT_V_ARH_IMP_Q2: crate::Point<Self, Option<f32>> = crate::Point::new(98, 2, false);
    pub const TOT_V_ARH_IMP_Q2PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(100, 2, false);
    pub const TOT_V_ARH_IMP_Q2PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(102, 2, false);
    pub const TOT_V_ARH_IMP_Q2PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(104, 2, false);
    pub const TOT_V_ARH_EXP_Q3: crate::Point<Self, Option<f32>> = crate::Point::new(106, 2, false);
    pub const TOT_V_ARH_EXP_Q3PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(108, 2, false);
    pub const TOT_V_ARH_EXP_Q3PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(110, 2, false);
    pub const TOT_V_ARH_EXP_Q3PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(112, 2, false);
    pub const TOT_V_ARH_EXP_Q4: crate::Point<Self, Option<f32>> = crate::Point::new(114, 2, false);
    pub const TOT_V_ARH_EXP_Q4PH_A: crate::Point<Self, Option<f32>> =
        crate::Point::new(116, 2, false);
    pub const TOT_V_ARH_EXP_Q4PH_B: crate::Point<Self, Option<f32>> =
        crate::Point::new(118, 2, false);
    pub const TOT_V_ARH_EXP_Q4PH_C: crate::Point<Self, Option<f32>> =
        crate::Point::new(120, 2, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(122, 2, false);
}
impl crate::Model for Model213 {
    const ID: u16 = 213;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            aph_a: Self::APH_A.from_data(data)?,
            aph_b: Self::APH_B.from_data(data)?,
            aph_c: Self::APH_C.from_data(data)?,
            ph_v: Self::PH_V.from_data(data)?,
            ph_vph_a: Self::PH_VPH_A.from_data(data)?,
            ph_vph_b: Self::PH_VPH_B.from_data(data)?,
            ph_vph_c: Self::PH_VPH_C.from_data(data)?,
            ppv: Self::PPV.from_data(data)?,
            pp_vph_ab: Self::PP_VPH_AB.from_data(data)?,
            pp_vph_bc: Self::PP_VPH_BC.from_data(data)?,
            pp_vph_ca: Self::PP_VPH_CA.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            w: Self::W.from_data(data)?,
            wph_a: Self::WPH_A.from_data(data)?,
            wph_b: Self::WPH_B.from_data(data)?,
            wph_c: Self::WPH_C.from_data(data)?,
            va: Self::VA.from_data(data)?,
            v_aph_a: Self::V_APH_A.from_data(data)?,
            v_aph_b: Self::V_APH_B.from_data(data)?,
            v_aph_c: Self::V_APH_C.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            va_rph_a: Self::VA_RPH_A.from_data(data)?,
            va_rph_b: Self::VA_RPH_B.from_data(data)?,
            va_rph_c: Self::VA_RPH_C.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            p_fph_a: Self::P_FPH_A.from_data(data)?,
            p_fph_b: Self::P_FPH_B.from_data(data)?,
            p_fph_c: Self::P_FPH_C.from_data(data)?,
            tot_wh_exp: Self::TOT_WH_EXP.from_data(data)?,
            tot_wh_exp_ph_a: Self::TOT_WH_EXP_PH_A.from_data(data)?,
            tot_wh_exp_ph_b: Self::TOT_WH_EXP_PH_B.from_data(data)?,
            tot_wh_exp_ph_c: Self::TOT_WH_EXP_PH_C.from_data(data)?,
            tot_wh_imp: Self::TOT_WH_IMP.from_data(data)?,
            tot_wh_imp_ph_a: Self::TOT_WH_IMP_PH_A.from_data(data)?,
            tot_wh_imp_ph_b: Self::TOT_WH_IMP_PH_B.from_data(data)?,
            tot_wh_imp_ph_c: Self::TOT_WH_IMP_PH_C.from_data(data)?,
            tot_v_ah_exp: Self::TOT_V_AH_EXP.from_data(data)?,
            tot_v_ah_exp_ph_a: Self::TOT_V_AH_EXP_PH_A.from_data(data)?,
            tot_v_ah_exp_ph_b: Self::TOT_V_AH_EXP_PH_B.from_data(data)?,
            tot_v_ah_exp_ph_c: Self::TOT_V_AH_EXP_PH_C.from_data(data)?,
            tot_v_ah_imp: Self::TOT_V_AH_IMP.from_data(data)?,
            tot_v_ah_imp_ph_a: Self::TOT_V_AH_IMP_PH_A.from_data(data)?,
            tot_v_ah_imp_ph_b: Self::TOT_V_AH_IMP_PH_B.from_data(data)?,
            tot_v_ah_imp_ph_c: Self::TOT_V_AH_IMP_PH_C.from_data(data)?,
            tot_v_arh_imp_q1: Self::TOT_V_ARH_IMP_Q1.from_data(data)?,
            tot_v_arh_imp_q1ph_a: Self::TOT_V_ARH_IMP_Q1PH_A.from_data(data)?,
            tot_v_arh_imp_q1ph_b: Self::TOT_V_ARH_IMP_Q1PH_B.from_data(data)?,
            tot_v_arh_imp_q1ph_c: Self::TOT_V_ARH_IMP_Q1PH_C.from_data(data)?,
            tot_v_arh_imp_q2: Self::TOT_V_ARH_IMP_Q2.from_data(data)?,
            tot_v_arh_imp_q2ph_a: Self::TOT_V_ARH_IMP_Q2PH_A.from_data(data)?,
            tot_v_arh_imp_q2ph_b: Self::TOT_V_ARH_IMP_Q2PH_B.from_data(data)?,
            tot_v_arh_imp_q2ph_c: Self::TOT_V_ARH_IMP_Q2PH_C.from_data(data)?,
            tot_v_arh_exp_q3: Self::TOT_V_ARH_EXP_Q3.from_data(data)?,
            tot_v_arh_exp_q3ph_a: Self::TOT_V_ARH_EXP_Q3PH_A.from_data(data)?,
            tot_v_arh_exp_q3ph_b: Self::TOT_V_ARH_EXP_Q3PH_B.from_data(data)?,
            tot_v_arh_exp_q3ph_c: Self::TOT_V_ARH_EXP_Q3PH_C.from_data(data)?,
            tot_v_arh_exp_q4: Self::TOT_V_ARH_EXP_Q4.from_data(data)?,
            tot_v_arh_exp_q4ph_a: Self::TOT_V_ARH_EXP_Q4PH_A.from_data(data)?,
            tot_v_arh_exp_q4ph_b: Self::TOT_V_ARH_EXP_Q4PH_B.from_data(data)?,
            tot_v_arh_exp_q4ph_c: Self::TOT_V_ARH_EXP_Q4PH_C.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m213
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
