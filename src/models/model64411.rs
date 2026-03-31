//! AC Simulator Control Interface
/// Type alias for [`AcSimInterface`].
pub type Model64411 = AcSimInterface;
struct Counts {
    n_prof: u16,
    n_pt: u16,
}
/// AC Simulator Control Interface
///
/// A generic AC simulator/power supply control interface for DER electrical testing.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct AcSimInterface {
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    pub phases: Option<u16>,
    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    pub phase_angle: Option<u16>,
    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    pub v_nom: Option<u16>,
    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    pub v_max: Option<u16>,
    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    pub i_max: Option<u16>,
    /// Frequency
    ///
    /// Frequency Setpoint
    pub freq: Option<u16>,
    /// Output State
    ///
    /// AC Output State
    pub output: Option<Output>,
    /// Relay State
    ///
    /// AC Relay State
    pub relay: Option<Relay>,
    /// Regeneration State
    ///
    /// Regeneration State
    pub regen: Option<Regen>,
    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    pub v_set: Option<u16>,
    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    pub v_set_a: Option<u16>,
    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    pub v_set_b: Option<u16>,
    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    pub v_set_c: Option<u16>,
    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    pub freq_slew: Option<u16>,
    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    pub v_slew: Option<u16>,
    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    pub va: Option<i32>,
    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    pub vb: Option<i32>,
    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    pub vc: Option<i32>,
    /// Measured Frequency
    ///
    /// Measured Frequency
    pub hz: Option<i32>,
    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    pub ia: Option<i32>,
    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    pub ib: Option<i32>,
    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    pub ic: Option<i32>,
    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    pub v_har_a: Option<String>,
    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    pub v_har_b: Option<String>,
    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    pub v_har_c: Option<String>,
    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    pub i_har_a: Option<String>,
    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    pub i_har_b: Option<String>,
    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    pub i_har_c: Option<String>,
    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    pub i_int_har_a: Option<String>,
    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    pub i_int_har_b: Option<String>,
    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    pub i_int_har_c: Option<String>,
    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    pub v_thd_a: Option<u16>,
    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    pub v_thd_b: Option<u16>,
    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    pub v_thd_c: Option<u16>,
    /// Current THD Phase A
    ///
    /// Current THD Phase A
    pub i_thd_a: Option<u16>,
    /// Current THD Phase B
    ///
    /// Current THD Phase B
    pub i_thd_b: Option<u16>,
    /// Current THD Phase C
    ///
    /// Current THD Phase C
    pub i_thd_c: Option<u16>,
    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    pub ena_prof: Option<EnaProf>,
    /// Profile Result
    ///
    /// Result of last profile operation.
    pub prof_rslt: ProfRslt,
    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    pub n_prof: u16,
    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    pub n_pt: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    pub v_sf: i16,
    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    pub a_sf: i16,
    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    pub tms_sf: i16,
    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    pub hz_sf: i16,
    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    pub hz_slew_sf: i16,
    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    pub v_slew_sf: i16,
    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    pub thd_sf: i16,
    /// Stored AC Profiles
    ///
    /// Stored AC profile sets.
    ///
    /// Comments: Stored AC Profile Sets - Number of profile sets = NProf
    pub prof: Vec<Prof>,
}
#[allow(missing_docs)]
impl AcSimInterface {
    pub const PHASES: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const PHASE_ANGLE: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const V_NOM: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const I_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const FREQ: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const OUTPUT: crate::Point<Self, Option<Output>> = crate::Point::new(6, 1, true);
    pub const RELAY: crate::Point<Self, Option<Relay>> = crate::Point::new(7, 1, true);
    pub const REGEN: crate::Point<Self, Option<Regen>> = crate::Point::new(8, 1, true);
    pub const V_SET: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, true);
    pub const V_SET_A: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, true);
    pub const V_SET_B: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, true);
    pub const V_SET_C: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, true);
    pub const FREQ_SLEW: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, true);
    pub const V_SLEW: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, true);
    pub const VA: crate::Point<Self, Option<i32>> = crate::Point::new(15, 2, false);
    pub const VB: crate::Point<Self, Option<i32>> = crate::Point::new(17, 2, false);
    pub const VC: crate::Point<Self, Option<i32>> = crate::Point::new(19, 2, false);
    pub const HZ: crate::Point<Self, Option<i32>> = crate::Point::new(21, 2, false);
    pub const IA: crate::Point<Self, Option<i32>> = crate::Point::new(23, 2, false);
    pub const IB: crate::Point<Self, Option<i32>> = crate::Point::new(25, 2, false);
    pub const IC: crate::Point<Self, Option<i32>> = crate::Point::new(27, 2, false);
    pub const V_HAR_A: crate::Point<Self, Option<String>> = crate::Point::new(29, 150, false);
    pub const V_HAR_B: crate::Point<Self, Option<String>> = crate::Point::new(179, 150, false);
    pub const V_HAR_C: crate::Point<Self, Option<String>> = crate::Point::new(329, 150, false);
    pub const I_HAR_A: crate::Point<Self, Option<String>> = crate::Point::new(479, 150, false);
    pub const I_HAR_B: crate::Point<Self, Option<String>> = crate::Point::new(629, 150, false);
    pub const I_HAR_C: crate::Point<Self, Option<String>> = crate::Point::new(779, 150, false);
    pub const I_INT_HAR_A: crate::Point<Self, Option<String>> = crate::Point::new(929, 150, false);
    pub const I_INT_HAR_B: crate::Point<Self, Option<String>> = crate::Point::new(1079, 150, false);
    pub const I_INT_HAR_C: crate::Point<Self, Option<String>> = crate::Point::new(1229, 150, false);
    pub const V_THD_A: crate::Point<Self, Option<u16>> = crate::Point::new(1379, 1, false);
    pub const V_THD_B: crate::Point<Self, Option<u16>> = crate::Point::new(1380, 1, false);
    pub const V_THD_C: crate::Point<Self, Option<u16>> = crate::Point::new(1381, 1, false);
    pub const I_THD_A: crate::Point<Self, Option<u16>> = crate::Point::new(1382, 1, false);
    pub const I_THD_B: crate::Point<Self, Option<u16>> = crate::Point::new(1383, 1, false);
    pub const I_THD_C: crate::Point<Self, Option<u16>> = crate::Point::new(1384, 1, false);
    pub const ENA_PROF: crate::Point<Self, Option<EnaProf>> = crate::Point::new(1385, 1, true);
    pub const PROF_RSLT: crate::Point<Self, ProfRslt> = crate::Point::new(1386, 1, false);
    pub const N_PROF: crate::Point<Self, u16> = crate::Point::new(1387, 1, false);
    pub const N_PT: crate::Point<Self, u16> = crate::Point::new(1388, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(1389, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(1390, 1, false);
    pub const TMS_SF: crate::Point<Self, i16> = crate::Point::new(1391, 1, false);
    pub const HZ_SF: crate::Point<Self, i16> = crate::Point::new(1392, 1, false);
    pub const HZ_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(1393, 1, false);
    pub const V_SLEW_SF: crate::Point<Self, i16> = crate::Point::new(1394, 1, false);
    pub const THD_SF: crate::Point<Self, i16> = crate::Point::new(1395, 1, false);
}
impl crate::Group for AcSimInterface {
    const LEN: u16 = 1396;
}
impl AcSimInterface {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let counts = Counts {
            n_prof: Self::N_PROF.from_data(data)?,
            n_pt: Self::N_PT.from_data(data)?,
        };
        let (nested_data, prof) = Prof::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                phases: Self::PHASES.from_data(data)?,
                phase_angle: Self::PHASE_ANGLE.from_data(data)?,
                v_nom: Self::V_NOM.from_data(data)?,
                v_max: Self::V_MAX.from_data(data)?,
                i_max: Self::I_MAX.from_data(data)?,
                freq: Self::FREQ.from_data(data)?,
                output: Self::OUTPUT.from_data(data)?,
                relay: Self::RELAY.from_data(data)?,
                regen: Self::REGEN.from_data(data)?,
                v_set: Self::V_SET.from_data(data)?,
                v_set_a: Self::V_SET_A.from_data(data)?,
                v_set_b: Self::V_SET_B.from_data(data)?,
                v_set_c: Self::V_SET_C.from_data(data)?,
                freq_slew: Self::FREQ_SLEW.from_data(data)?,
                v_slew: Self::V_SLEW.from_data(data)?,
                va: Self::VA.from_data(data)?,
                vb: Self::VB.from_data(data)?,
                vc: Self::VC.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                ia: Self::IA.from_data(data)?,
                ib: Self::IB.from_data(data)?,
                ic: Self::IC.from_data(data)?,
                v_har_a: Self::V_HAR_A.from_data(data)?,
                v_har_b: Self::V_HAR_B.from_data(data)?,
                v_har_c: Self::V_HAR_C.from_data(data)?,
                i_har_a: Self::I_HAR_A.from_data(data)?,
                i_har_b: Self::I_HAR_B.from_data(data)?,
                i_har_c: Self::I_HAR_C.from_data(data)?,
                i_int_har_a: Self::I_INT_HAR_A.from_data(data)?,
                i_int_har_b: Self::I_INT_HAR_B.from_data(data)?,
                i_int_har_c: Self::I_INT_HAR_C.from_data(data)?,
                v_thd_a: Self::V_THD_A.from_data(data)?,
                v_thd_b: Self::V_THD_B.from_data(data)?,
                v_thd_c: Self::V_THD_C.from_data(data)?,
                i_thd_a: Self::I_THD_A.from_data(data)?,
                i_thd_b: Self::I_THD_B.from_data(data)?,
                i_thd_c: Self::I_THD_C.from_data(data)?,
                ena_prof: Self::ENA_PROF.from_data(data)?,
                prof_rslt: Self::PROF_RSLT.from_data(data)?,
                n_prof: Self::N_PROF.from_data(data)?,
                n_pt: Self::N_PT.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                tms_sf: Self::TMS_SF.from_data(data)?,
                hz_sf: Self::HZ_SF.from_data(data)?,
                hz_slew_sf: Self::HZ_SLEW_SF.from_data(data)?,
                v_slew_sf: Self::V_SLEW_SF.from_data(data)?,
                thd_sf: Self::THD_SF.from_data(data)?,
                prof,
            },
        ))
    }
}
/// Output State
///
/// AC Output State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Output {
    /// Output Off
    Off,
    /// Output On
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Output {
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
impl crate::FixedSize for Output {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Relay State
///
/// AC Relay State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Relay {
    /// Relay Open
    Open,
    /// Relay Closed
    Closed,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Relay {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Open,
            1 => Self::Closed,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Open => 0,
            Self::Closed => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Relay {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Regeneration State
///
/// Regeneration State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Regen {
    /// Regen Off
    Off,
    /// Regen On
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Regen {
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
impl crate::FixedSize for Regen {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Enable Profile
///
/// Start/Stop the AC Profile
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum EnaProf {
    /// Stop Profile
    Stop,
    /// Start Profile Immediately
    Start,
    /// Start Profile via External Trigger Signal
    Trigger,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for EnaProf {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Stop,
            1 => Self::Start,
            2 => Self::Trigger,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Stop => 0,
            Self::Start => 1,
            Self::Trigger => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for EnaProf {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Profile Result
///
/// Result of last profile operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ProfRslt {
    /// Profile update in progress.
    InProgress,
    /// Profile update completed successfully.
    Completed,
    /// Profile update failed.
    Failed,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ProfRslt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::InProgress,
            1 => Self::Completed,
            2 => Self::Failed,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::InProgress => 0,
            Self::Completed => 1,
            Self::Failed => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ProfRslt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Stored AC Profiles
///
/// Stored AC profile sets.
///
/// Comments: Stored AC Profile Sets - Number of profile sets = NProf
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Prof {
    /// Profile Name
    ///
    /// Profile name.
    pub name: Option<String>,
    /// Active Points
    ///
    /// Number of active points.
    pub act_pt: u16,
    /// Stored AC Profile Points
    ///
    /// Stored AC profile points.
    ///
    /// Comments: Stored AC Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
    pub pt: Vec<Pt>,
}
#[allow(missing_docs)]
impl Prof {
    pub const NAME: crate::Point<Self, Option<String>> = crate::Point::new(0, 32, true);
    pub const ACT_PT: crate::Point<Self, u16> = crate::Point::new(32, 1, true);
}
impl crate::Group for Prof {
    const LEN: u16 = 33;
}
impl Prof {
    fn parse_group<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, pt) = Pt::parse_multiple(nested_data, counts)?;
        Ok((
            nested_data,
            Self {
                name: Self::NAME.from_data(data)?,
                act_pt: Self::ACT_PT.from_data(data)?,
                pt,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_prof).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Prof::parse_group(data, counts)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Stored AC Profile Points
///
/// Stored AC profile points.
///
/// Comments: Stored AC Profile Sets - Profile points for each stored profile - Number of profile points contained in NPt
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pt {
    /// Profile Time
    ///
    /// Profile time.
    pub tms: Option<u16>,
    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    pub va: Option<u16>,
    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    pub vb: Option<u16>,
    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    pub vc: Option<u16>,
    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    pub hz: Option<u16>,
    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    pub phase_angle_a: Option<u16>,
    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    pub phase_angle_b: Option<u16>,
    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    pub phase_angle_c: Option<u16>,
}
#[allow(missing_docs)]
impl Pt {
    pub const TMS: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const VA: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const VB: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const VC: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const HZ: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const PHASE_ANGLE_A: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const PHASE_ANGLE_B: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const PHASE_ANGLE_C: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
}
impl crate::Group for Pt {
    const LEN: u16 = 8;
}
impl Pt {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                tms: Self::TMS.from_data(data)?,
                va: Self::VA.from_data(data)?,
                vb: Self::VB.from_data(data)?,
                vc: Self::VC.from_data(data)?,
                hz: Self::HZ.from_data(data)?,
                phase_angle_a: Self::PHASE_ANGLE_A.from_data(data)?,
                phase_angle_b: Self::PHASE_ANGLE_B.from_data(data)?,
                phase_angle_c: Self::PHASE_ANGLE_C.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_pt).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Pt::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for AcSimInterface {
    const ID: u16 = 64411;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64411
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
