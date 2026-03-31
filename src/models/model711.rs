//! DER Frequency Droop
/// Type alias for [`DerFreqDroop`].
pub type Model711 = DerFreqDroop;
struct Counts {
    n_ctl: u16,
}
/// DER Frequency Droop
///
/// DER Frequency Droop model.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerFreqDroop {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    pub ena: Ena,
    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    pub adpt_ctl_req: u16,
    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    pub adpt_ctl_rslt: AdptCtlRslt,
    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    pub n_ctl: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds.  0 = No reversion time.
    pub rvrt_tms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    pub rvrt_rem: Option<u32>,
    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    pub rvrt_ctl: Option<u16>,
    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    pub db_sf: i16,
    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    pub k_sf: i16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    pub rsp_tms_sf: i16,
    /// Stored Controls
    ///
    /// Stored control sets.
    ///
    /// Comments: Stored control sets - Number of control sets contained in NCtl - The first set is read-only and indicates the current settings.
    pub ctl: Vec<Ctl>,
}
#[allow(missing_docs)]
impl DerFreqDroop {
    pub const ENA: crate::Point<Self, Ena> = crate::Point::new(0, 1, true);
    pub const ADPT_CTL_REQ: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const ADPT_CTL_RSLT: crate::Point<Self, AdptCtlRslt> = crate::Point::new(2, 1, false);
    pub const N_CTL: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const RVRT_TMS: crate::Point<Self, Option<u32>> = crate::Point::new(4, 2, true);
    pub const RVRT_REM: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2, false);
    pub const RVRT_CTL: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const DB_SF: crate::Point<Self, i16> = crate::Point::new(9, 1, false);
    pub const K_SF: crate::Point<Self, i16> = crate::Point::new(10, 1, false);
    pub const RSP_TMS_SF: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
}
impl crate::Group for DerFreqDroop {
    const LEN: u16 = 12;
}
impl DerFreqDroop {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let counts = Counts {
            n_ctl: Self::N_CTL.from_data(data)?,
        };
        let (nested_data, ctl) = Ctl::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                ena: Self::ENA.from_data(data)?,
                adpt_ctl_req: Self::ADPT_CTL_REQ.from_data(data)?,
                adpt_ctl_rslt: Self::ADPT_CTL_RSLT.from_data(data)?,
                n_ctl: Self::N_CTL.from_data(data)?,
                rvrt_tms: Self::RVRT_TMS.from_data(data)?,
                rvrt_rem: Self::RVRT_REM.from_data(data)?,
                rvrt_ctl: Self::RVRT_CTL.from_data(data)?,
                db_sf: Self::DB_SF.from_data(data)?,
                k_sf: Self::K_SF.from_data(data)?,
                rsp_tms_sf: Self::RSP_TMS_SF.from_data(data)?,
                ctl,
            },
        ))
    }
}
/// DER Frequency Droop Module Enable
///
/// DER Frequency-Watt (Frequency-Droop) control enable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Ena {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Ena {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Ena {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Active Control Result
///
/// Result of last set active control operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum AdptCtlRslt {
    /// Update In Progress
    ///
    /// Control update in progress.
    InProgress,
    /// Update Complete
    ///
    /// Control update completed successfully.
    Completed,
    /// Update Failed
    ///
    /// Control update failed.
    Failed,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for AdptCtlRslt {
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
impl crate::FixedSize for AdptCtlRslt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Stored Controls
///
/// Stored control sets.
///
/// Comments: Stored control sets - Number of control sets contained in NCtl - The first set is read-only and indicates the current settings.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Ctl {
    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    pub db_of: u32,
    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    pub db_uf: u32,
    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    pub k_of: u16,
    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    pub k_uf: u16,
    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    pub rsp_tms: u32,
    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    pub p_min: Option<i16>,
    /// Control Access
    ///
    /// Control read-write access.
    pub read_only: CtlReadOnly,
}
#[allow(missing_docs)]
impl Ctl {
    pub const DB_OF: crate::Point<Self, u32> = crate::Point::new(0, 2, true);
    pub const DB_UF: crate::Point<Self, u32> = crate::Point::new(2, 2, true);
    pub const K_OF: crate::Point<Self, u16> = crate::Point::new(4, 1, true);
    pub const K_UF: crate::Point<Self, u16> = crate::Point::new(5, 1, true);
    pub const RSP_TMS: crate::Point<Self, u32> = crate::Point::new(6, 2, true);
    pub const P_MIN: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, true);
    pub const READ_ONLY: crate::Point<Self, CtlReadOnly> = crate::Point::new(9, 1, false);
}
impl crate::Group for Ctl {
    const LEN: u16 = 10;
}
impl Ctl {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                db_of: Self::DB_OF.from_data(data)?,
                db_uf: Self::DB_UF.from_data(data)?,
                k_of: Self::K_OF.from_data(data)?,
                k_uf: Self::K_UF.from_data(data)?,
                rsp_tms: Self::RSP_TMS.from_data(data)?,
                p_min: Self::P_MIN.from_data(data)?,
                read_only: Self::READ_ONLY.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) =
            (0..counts.n_ctl).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Ctl::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Control Access
///
/// Control read-write access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CtlReadOnly {
    /// Read-Write Access
    ///
    /// Control has read-write access.
    Rw,
    /// Read-Only Access
    ///
    /// Control has read-only access.
    R,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CtlReadOnly {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Rw,
            1 => Self::R,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Rw => 0,
            Self::R => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CtlReadOnly {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for DerFreqDroop {
    const ID: u16 = 711;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m711
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
