/// Secure AC Meter Selected Readings
///
/// Include this model for secure metering
#[derive(Debug)]
pub struct Model220 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage
    ///
    /// Average phase or line voltage
    pub ph_v: Option<i16>,
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
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub tot_wh_exp: u32,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub tot_wh_imp: u32,
    /// Real Energy scale factor
    pub tot_wh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub tot_v_ah_exp: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub tot_v_ah_imp: Option<u32>,
    /// Apparent Energy scale factor
    pub tot_v_ah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub tot_v_arh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub tot_v_arh_imp_q2: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub tot_v_arh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub tot_v_arh_exp_q4: Option<u32>,
    /// Reactive Energy scale factor
    pub tot_v_arh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Notes: Shall be advanced for each request
    pub seq: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Notes: For future proof
    pub alg: u16,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}

#[allow(missing_docs)]

impl Model220 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const PH_V: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const HZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(5, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const VA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, false);
    pub const VA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(9, 1, false);
    pub const VAR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(10, 1, false);
    pub const VAR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(11, 1, false);
    pub const PF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(12, 1, false);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(13, 1, false);
    pub const TOT_WH_EXP: crate::PointDef<Self, u32> = crate::PointDef::new(14, 2, false);
    pub const TOT_WH_IMP: crate::PointDef<Self, u32> = crate::PointDef::new(16, 2, false);
    pub const TOT_WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const TOT_V_AH_EXP: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(19, 2, false);
    pub const TOT_V_AH_IMP: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(21, 2, false);
    pub const TOT_V_AH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const TOT_V_ARH_IMP_Q1: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(24, 2, false);
    pub const TOT_V_ARH_IMP_Q2: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(26, 2, false);
    pub const TOT_V_ARH_EXP_Q3: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(28, 2, false);
    pub const TOT_V_ARH_EXP_Q4: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(30, 2, false);
    pub const TOT_V_ARH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(32, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(33, 2, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, false);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
}

impl crate::Model for Model220 {
    const ID: u16 = 220;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            ph_v: Self::PH_V.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            tot_wh_exp: Self::TOT_WH_EXP.from_data(data)?,
            tot_wh_imp: Self::TOT_WH_IMP.from_data(data)?,
            tot_wh_sf: Self::TOT_WH_SF.from_data(data)?,
            tot_v_ah_exp: Self::TOT_V_AH_EXP.from_data(data)?,
            tot_v_ah_imp: Self::TOT_V_AH_IMP.from_data(data)?,
            tot_v_ah_sf: Self::TOT_V_AH_SF.from_data(data)?,
            tot_v_arh_imp_q1: Self::TOT_V_ARH_IMP_Q1.from_data(data)?,
            tot_v_arh_imp_q2: Self::TOT_V_ARH_IMP_Q2.from_data(data)?,
            tot_v_arh_exp_q3: Self::TOT_V_ARH_EXP_Q3.from_data(data)?,
            tot_v_arh_exp_q4: Self::TOT_V_ARH_EXP_Q4.from_data(data)?,
            tot_v_arh_sf: Self::TOT_V_ARH_SF.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            ts: Self::TS.from_data(data)?,
            ms: Self::MS.from_data(data)?,
            seq: Self::SEQ.from_data(data)?,
            alg: Self::ALG.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
}
