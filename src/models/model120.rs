/// Nameplate
///
/// Inverter Controls Nameplate Ratings
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model120 {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    pub der_typ: u16,
    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    pub w_rtg: u16,
    /// WRtg_SF
    ///
    /// Scale factor
    pub w_rtg_sf: i16,
    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    pub va_rtg: u16,
    /// VARtg_SF
    ///
    /// Scale factor
    pub va_rtg_sf: i16,
    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    pub v_ar_rtg_q1: i16,
    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    pub v_ar_rtg_q2: i16,
    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    pub v_ar_rtg_q3: i16,
    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    pub v_ar_rtg_q4: i16,
    /// VArRtg_SF
    ///
    /// Scale factor
    pub v_ar_rtg_sf: i16,
    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Notes: Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    pub a_rtg: u16,
    /// ARtg_SF
    ///
    /// Scale factor
    pub a_rtg_sf: i16,
    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// Notes: EEI sign convention.
    pub pf_rtg_q1: i16,
    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// Notes: EEI sign convention.
    pub pf_rtg_q2: i16,
    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// Notes: EEI sign convention.
    pub pf_rtg_q3: i16,
    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// Notes: EEI sign convention.
    pub pf_rtg_q4: i16,
    /// PFRtg_SF
    ///
    /// Scale factor
    pub pf_rtg_sf: i16,
    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    pub wh_rtg: Option<u16>,
    /// WHRtg_SF
    ///
    /// Scale factor
    pub wh_rtg_sf: Option<i16>,
    /// AhrRtg
    ///
    /// The usable capacity of the battery.  Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    pub ahr_rtg: Option<u16>,
    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    pub ahr_rtg_sf: Option<i16>,
    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    pub max_cha_rte: Option<u16>,
    /// MaxChaRte_SF
    ///
    /// Scale factor
    pub max_cha_rte_sf: Option<i16>,
    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    pub max_dis_cha_rte: Option<u16>,
    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    pub max_dis_cha_rte_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model120 {
    pub const DER_TYP: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const W_RTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const W_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const VA_RTG: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const VA_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const V_AR_RTG_Q1: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const V_AR_RTG_Q2: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const V_AR_RTG_Q3: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const V_AR_RTG_Q4: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const V_AR_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const A_RTG: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const A_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PF_RTG_Q1: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const PF_RTG_Q2: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const PF_RTG_Q3: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const PF_RTG_Q4: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const PF_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WH_RTG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, false);
    pub const WH_RTG_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const AHR_RTG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(19, 1, false);
    pub const AHR_RTG_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(20, 1, false);
    pub const MAX_CHA_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(21, 1, false);
    pub const MAX_CHA_RTE_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(22, 1, false);
    pub const MAX_DIS_CHA_RTE: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(23, 1, false);
    pub const MAX_DIS_CHA_RTE_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(24, 1, false);
}

impl crate::Model for Model120 {
    const ID: u16 = 120;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            der_typ: Self::DER_TYP.from_data(data)?,
            w_rtg: Self::W_RTG.from_data(data)?,
            w_rtg_sf: Self::W_RTG_SF.from_data(data)?,
            va_rtg: Self::VA_RTG.from_data(data)?,
            va_rtg_sf: Self::VA_RTG_SF.from_data(data)?,
            v_ar_rtg_q1: Self::V_AR_RTG_Q1.from_data(data)?,
            v_ar_rtg_q2: Self::V_AR_RTG_Q2.from_data(data)?,
            v_ar_rtg_q3: Self::V_AR_RTG_Q3.from_data(data)?,
            v_ar_rtg_q4: Self::V_AR_RTG_Q4.from_data(data)?,
            v_ar_rtg_sf: Self::V_AR_RTG_SF.from_data(data)?,
            a_rtg: Self::A_RTG.from_data(data)?,
            a_rtg_sf: Self::A_RTG_SF.from_data(data)?,
            pf_rtg_q1: Self::PF_RTG_Q1.from_data(data)?,
            pf_rtg_q2: Self::PF_RTG_Q2.from_data(data)?,
            pf_rtg_q3: Self::PF_RTG_Q3.from_data(data)?,
            pf_rtg_q4: Self::PF_RTG_Q4.from_data(data)?,
            pf_rtg_sf: Self::PF_RTG_SF.from_data(data)?,
            wh_rtg: Self::WH_RTG.from_data(data)?,
            wh_rtg_sf: Self::WH_RTG_SF.from_data(data)?,
            ahr_rtg: Self::AHR_RTG.from_data(data)?,
            ahr_rtg_sf: Self::AHR_RTG_SF.from_data(data)?,
            max_cha_rte: Self::MAX_CHA_RTE.from_data(data)?,
            max_cha_rte_sf: Self::MAX_CHA_RTE_SF.from_data(data)?,
            max_dis_cha_rte: Self::MAX_DIS_CHA_RTE.from_data(data)?,
            max_dis_cha_rte_sf: Self::MAX_DIS_CHA_RTE_SF.from_data(data)?,
        })
    }
}
