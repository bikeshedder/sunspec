//! Basic Settings
/// Basic Settings
///
/// Inverter Controls Basic Settings
///
/// Notes: Ref 3: 8.4.2.1, Ref 4: 17
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model121 {
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    pub w_max: u16,
    /// VRef
    ///
    /// Voltage at the PCC.
    pub v_ref: u16,
    /// VRefOfs
    ///
    /// Offset  from PCC to inverter.
    pub v_ref_ofs: i16,
    /// VMax
    ///
    /// Setpoint for maximum voltage.
    pub v_max: Option<u16>,
    /// VMin
    ///
    /// Setpoint for minimum voltage.
    pub v_min: Option<u16>,
    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    pub va_max: Option<u16>,
    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    pub v_ar_max_q1: Option<i16>,
    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    pub v_ar_max_q2: Option<i16>,
    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    pub v_ar_max_q3: Option<i16>,
    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    pub v_ar_max_q4: Option<i16>,
    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    pub w_gra: Option<u16>,
    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// Notes: EEI sign convention.
    pub pf_min_q1: Option<i16>,
    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// Notes: EEI sign convention.
    pub pf_min_q2: Option<i16>,
    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// Notes: EEI sign convention.
    pub pf_min_q3: Option<i16>,
    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// Notes: EEI sign convention.
    pub pf_min_q4: Option<i16>,
    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    pub v_ar_act: Option<VArAct>,
    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    pub clc_tot_va: Option<ClcTotVa>,
    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    pub max_rmp_rte: Option<u16>,
    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    pub ecp_nom_hz: Option<u16>,
    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    pub conn_ph: Option<ConnPh>,
    /// WMax_SF
    ///
    /// Scale factor for real power.
    pub w_max_sf: i16,
    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    pub v_ref_sf: i16,
    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    pub v_ref_ofs_sf: i16,
    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    pub v_min_max_sf: Option<i16>,
    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    pub va_max_sf: Option<i16>,
    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    pub v_ar_max_sf: Option<i16>,
    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    pub w_gra_sf: Option<i16>,
    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    pub pf_min_sf: Option<i16>,
    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    pub max_rmp_rte_sf: Option<i16>,
    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    pub ecp_nom_hz_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model121 {
    pub const W_MAX: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const V_REF: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const V_REF_OFS: crate::Point<Self, i16> = crate::Point::new(2, 1, true);
    pub const V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const VA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const V_AR_MAX_Q1: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, true);
    pub const V_AR_MAX_Q2: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, true);
    pub const V_AR_MAX_Q3: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, true);
    pub const V_AR_MAX_Q4: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, true);
    pub const W_GRA: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const PF_MIN_Q1: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, true);
    pub const PF_MIN_Q2: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, true);
    pub const PF_MIN_Q3: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, true);
    pub const PF_MIN_Q4: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, true);
    pub const V_AR_ACT: crate::Point<Self, Option<VArAct>> = crate::Point::new(15, 1, true);
    pub const CLC_TOT_VA: crate::Point<Self, Option<ClcTotVa>> = crate::Point::new(16, 1, true);
    pub const MAX_RMP_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const ECP_NOM_HZ: crate::Point<Self, Option<u16>> = crate::Point::new(18, 1, true);
    pub const CONN_PH: crate::Point<Self, Option<ConnPh>> = crate::Point::new(19, 1, true);
    pub const W_MAX_SF: crate::Point<Self, i16> = crate::Point::new(20, 1, false);
    pub const V_REF_SF: crate::Point<Self, i16> = crate::Point::new(21, 1, false);
    pub const V_REF_OFS_SF: crate::Point<Self, i16> = crate::Point::new(22, 1, false);
    pub const V_MIN_MAX_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const VA_MAX_SF: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, false);
    pub const V_AR_MAX_SF: crate::Point<Self, Option<i16>> = crate::Point::new(25, 1, false);
    pub const W_GRA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(26, 1, false);
    pub const PF_MIN_SF: crate::Point<Self, Option<i16>> = crate::Point::new(27, 1, false);
    pub const MAX_RMP_RTE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(28, 1, false);
    pub const ECP_NOM_HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(29, 1, false);
}
impl crate::Model for Model121 {
    const ID: u16 = 121;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            w_max: Self::W_MAX.from_data(data)?,
            v_ref: Self::V_REF.from_data(data)?,
            v_ref_ofs: Self::V_REF_OFS.from_data(data)?,
            v_max: Self::V_MAX.from_data(data)?,
            v_min: Self::V_MIN.from_data(data)?,
            va_max: Self::VA_MAX.from_data(data)?,
            v_ar_max_q1: Self::V_AR_MAX_Q1.from_data(data)?,
            v_ar_max_q2: Self::V_AR_MAX_Q2.from_data(data)?,
            v_ar_max_q3: Self::V_AR_MAX_Q3.from_data(data)?,
            v_ar_max_q4: Self::V_AR_MAX_Q4.from_data(data)?,
            w_gra: Self::W_GRA.from_data(data)?,
            pf_min_q1: Self::PF_MIN_Q1.from_data(data)?,
            pf_min_q2: Self::PF_MIN_Q2.from_data(data)?,
            pf_min_q3: Self::PF_MIN_Q3.from_data(data)?,
            pf_min_q4: Self::PF_MIN_Q4.from_data(data)?,
            v_ar_act: Self::V_AR_ACT.from_data(data)?,
            clc_tot_va: Self::CLC_TOT_VA.from_data(data)?,
            max_rmp_rte: Self::MAX_RMP_RTE.from_data(data)?,
            ecp_nom_hz: Self::ECP_NOM_HZ.from_data(data)?,
            conn_ph: Self::CONN_PH.from_data(data)?,
            w_max_sf: Self::W_MAX_SF.from_data(data)?,
            v_ref_sf: Self::V_REF_SF.from_data(data)?,
            v_ref_ofs_sf: Self::V_REF_OFS_SF.from_data(data)?,
            v_min_max_sf: Self::V_MIN_MAX_SF.from_data(data)?,
            va_max_sf: Self::VA_MAX_SF.from_data(data)?,
            v_ar_max_sf: Self::V_AR_MAX_SF.from_data(data)?,
            w_gra_sf: Self::W_GRA_SF.from_data(data)?,
            pf_min_sf: Self::PF_MIN_SF.from_data(data)?,
            max_rmp_rte_sf: Self::MAX_RMP_RTE_SF.from_data(data)?,
            ecp_nom_hz_sf: Self::ECP_NOM_HZ_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m121
    }
}
/// VArAct
///
/// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum VArAct {
    #[allow(missing_docs)]
    Switch = 1,
    #[allow(missing_docs)]
    Maintain = 2,
}
impl crate::Value for VArAct {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<VArAct> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                VArAct::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// ClcTotVA
///
/// Calculation method for total apparent power. 1=vector 2=arithmetic.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ClcTotVa {
    #[allow(missing_docs)]
    Vector = 1,
    #[allow(missing_docs)]
    Arithmetic = 2,
}
impl crate::Value for ClcTotVa {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ClcTotVa> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ClcTotVa::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// ConnPh
///
/// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ConnPh {
    #[allow(missing_docs)]
    A = 1,
    #[allow(missing_docs)]
    B = 2,
    #[allow(missing_docs)]
    C = 3,
}
impl crate::Value for ConnPh {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ConnPh> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ConnPh::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
