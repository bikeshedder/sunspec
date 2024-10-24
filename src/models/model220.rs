//! Secure AC Meter Selected Readings
/// Secure AC Meter Selected Readings
///
/// Include this model for secure metering
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
    pub evt: Evt,
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
    pub alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}
#[allow(missing_docs)]
impl Model220 {
    pub const A: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const PH_V: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(3, 1, false);
    pub const HZ: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const HZ_SF: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, false);
    pub const W: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
    pub const W_SF: crate::Point<Self, i16> = crate::Point::new(7, 1, false);
    pub const VA: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const VA_SF: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const VAR: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, false);
    pub const VAR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, false);
    pub const PF: crate::Point<Self, Option<i16>> = crate::Point::new(12, 1, false);
    pub const PF_SF: crate::Point<Self, Option<i16>> = crate::Point::new(13, 1, false);
    pub const TOT_WH_EXP: crate::Point<Self, u32> = crate::Point::new(14, 2, false);
    pub const TOT_WH_IMP: crate::Point<Self, u32> = crate::Point::new(16, 2, false);
    pub const TOT_WH_SF: crate::Point<Self, i16> = crate::Point::new(18, 1, false);
    pub const TOT_V_AH_EXP: crate::Point<Self, Option<u32>> = crate::Point::new(19, 2, false);
    pub const TOT_V_AH_IMP: crate::Point<Self, Option<u32>> = crate::Point::new(21, 2, false);
    pub const TOT_V_AH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const TOT_V_ARH_IMP_Q1: crate::Point<Self, Option<u32>> = crate::Point::new(24, 2, false);
    pub const TOT_V_ARH_IMP_Q2: crate::Point<Self, Option<u32>> = crate::Point::new(26, 2, false);
    pub const TOT_V_ARH_EXP_Q3: crate::Point<Self, Option<u32>> = crate::Point::new(28, 2, false);
    pub const TOT_V_ARH_EXP_Q4: crate::Point<Self, Option<u32>> = crate::Point::new(30, 2, false);
    pub const TOT_V_ARH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(32, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(33, 2, false);
    pub const TS: crate::Point<Self, u32> = crate::Point::new(36, 2, false);
    pub const MS: crate::Point<Self, u16> = crate::Point::new(38, 1, false);
    pub const SEQ: crate::Point<Self, u16> = crate::Point::new(39, 1, false);
    pub const ALG: crate::Point<Self, Alg> = crate::Point::new(40, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(41, 1, false);
}
impl crate::Model for Model220 {
    const ID: u16 = 220;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
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
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m220
    }
}
bitflags::bitflags! {
    #[doc = " Events"] #[doc = " "] #[doc = " Meter Event Flags"] #[derive(Copy, Clone,
    Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde", derive(::serde::Serialize,
    ::serde::Deserialize))] pub struct Evt : u32 { #[allow(missing_docs)] const
    PowerFailure = 4; #[allow(missing_docs)] const UnderVoltage = 8;
    #[allow(missing_docs)] const LowPf = 16; #[allow(missing_docs)] const OverCurrent =
    32; #[allow(missing_docs)] const OverVoltage = 64; #[allow(missing_docs)] const
    MissingSensor = 128; #[allow(missing_docs)] const Oem01 = 65536;
    #[allow(missing_docs)] const Oem02 = 131072; #[allow(missing_docs)] const Oem03 =
    262144; #[allow(missing_docs)] const Oem04 = 524288; #[allow(missing_docs)] const
    Oem05 = 1048576; #[allow(missing_docs)] const Oem06 = 2097152; #[allow(missing_docs)]
    const Oem07 = 4194304; #[allow(missing_docs)] const Oem08 = 8388608;
    #[allow(missing_docs)] const Oem09 = 16777216; #[allow(missing_docs)] const Oem10 =
    33554432; #[allow(missing_docs)] const Oem11 = 67108864; #[allow(missing_docs)] const
    Oem12 = 134217728; #[allow(missing_docs)] const Oem13 = 268435456;
    #[allow(missing_docs)] const Oem14 = 536870912; #[allow(missing_docs)] const Oem15 =
    1073741824; }
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
/// Algorithm
///
/// Algorithm used to compute the digital signature
///
/// Notes: For future proof
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Alg {
    /// Notes: For test purposes only
    None = 0,
    #[allow(missing_docs)]
    AesGmac64 = 1,
    #[allow(missing_docs)]
    Ecc256 = 2,
}
impl crate::Value for Alg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Alg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Alg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
