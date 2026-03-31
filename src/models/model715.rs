//! DERCtl
/// Type alias for [`DerCtl`].
pub type Model715 = DerCtl;
/// DERCtl
///
/// DER Control
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerCtl {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    ///
    /// Comments: DER Controls
    pub loc_rem_ctl: Option<LocRemCtl>,
    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    pub der_hb: Option<u32>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    pub controller_hb: Option<u32>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    pub alarm_reset: Option<u16>,
    /// Set Operation
    ///
    /// Commands to PCS. Enumerated value.
    pub op_ctl: Option<OpCtl>,
}
#[allow(missing_docs)]
impl DerCtl {
    pub const LOC_REM_CTL: crate::Point<Self, Option<LocRemCtl>> = crate::Point::new(0, 1, false);
    pub const DER_HB: crate::Point<Self, Option<u32>> = crate::Point::new(1, 2, false);
    pub const CONTROLLER_HB: crate::Point<Self, Option<u32>> = crate::Point::new(3, 2, true);
    pub const ALARM_RESET: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const OP_CTL: crate::Point<Self, Option<OpCtl>> = crate::Point::new(6, 1, true);
}
impl crate::Group for DerCtl {
    const LEN: u16 = 7;
}
impl DerCtl {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                loc_rem_ctl: Self::LOC_REM_CTL.from_data(data)?,
                der_hb: Self::DER_HB.from_data(data)?,
                controller_hb: Self::CONTROLLER_HB.from_data(data)?,
                alarm_reset: Self::ALARM_RESET.from_data(data)?,
                op_ctl: Self::OP_CTL.from_data(data)?,
            },
        ))
    }
}
/// Control Mode
///
/// DER control mode. Enumeration.
///
/// Comments: DER Controls
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum LocRemCtl {
    /// Remote Control
    Remote,
    /// Local Control
    ///
    /// Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
    Local,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for LocRemCtl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Remote,
            1 => Self::Local,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Remote => 0,
            Self::Local => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for LocRemCtl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Operation
///
/// Commands to PCS. Enumerated value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum OpCtl {
    /// Stop the DER
    Stop,
    /// Start the DER
    Start,
    /// Enter Standby Mode
    EnterStandby,
    /// Exit Standby Mode
    ExitStandby,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for OpCtl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Stop,
            1 => Self::Start,
            2 => Self::EnterStandby,
            3 => Self::ExitStandby,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Stop => 0,
            Self::Start => 1,
            Self::EnterStandby => 2,
            Self::ExitStandby => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for OpCtl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for DerCtl {
    const ID: u16 = 715;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m715
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
