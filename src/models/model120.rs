//! Nameplate
/// Nameplate
///
/// Inverter Controls Nameplate Ratings
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model120 {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    pub der_typ: DerTyp,
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
    pub const DER_TYP: crate::Point<Self, DerTyp> = crate::Point::new(0, 1, false);
    pub const W_RTG: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const W_RTG_SF: crate::Point<Self, i16> = crate::Point::new(2, 1, false);
    pub const VA_RTG: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const VA_RTG_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const V_AR_RTG_Q1: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const V_AR_RTG_Q2: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
    pub const V_AR_RTG_Q3: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const V_AR_RTG_Q4: crate::Point<Self, i16> = crate::Point::new(8, 1, false);
    pub const V_AR_RTG_SF: crate::Point<Self, i16> = crate::Point::new(9, 1, false);
    pub const A_RTG: crate::Point<Self, u16> = crate::Point::new(10, 1, false);
    pub const A_RTG_SF: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const PF_RTG_Q1: crate::Point<Self, i16> = crate::Point::new(12, 1, false);
    pub const PF_RTG_Q2: crate::Point<Self, i16> = crate::Point::new(13, 1, false);
    pub const PF_RTG_Q3: crate::Point<Self, i16> = crate::Point::new(14, 1, false);
    pub const PF_RTG_Q4: crate::Point<Self, i16> = crate::Point::new(15, 1, false);
    pub const PF_RTG_SF: crate::Point<Self, i16> = crate::Point::new(16, 1, false);
    pub const WH_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const WH_RTG_SF: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, false);
    pub const AHR_RTG: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, false);
    pub const AHR_RTG_SF: crate::Point<Self, Option<i16>> = crate::Point::new(20, 1, false);
    pub const MAX_CHA_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(21, 1, false);
    pub const MAX_CHA_RTE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, false);
    pub const MAX_DIS_CHA_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(23, 1, false);
    pub const MAX_DIS_CHA_RTE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, false);
}
impl crate::Model for Model120 {
    const ID: u16 = 120;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
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
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m120
    }
}
/// DERTyp
///
/// Type of DER device. Default value is 4 to indicate PV device.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum DerTyp {
    #[allow(missing_docs)]
    Pv = 4,
    #[allow(missing_docs)]
    PvStor = 82,
}
impl crate::Value for DerTyp {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<DerTyp> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                DerTyp::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
