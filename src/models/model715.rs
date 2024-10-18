//! DERCtl
/// DERCtl
///
/// DER Control
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model715 {
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
impl Model715 {
    pub const LOC_REM_CTL: crate::PointDef<Self, Option<LocRemCtl>> =
        crate::PointDef::new(0, 1, false);
    pub const DER_HB: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(1, 2, false);
    pub const CONTROLLER_HB: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(3, 2, true);
    pub const ALARM_RESET: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const OP_CTL: crate::PointDef<Self, Option<OpCtl>> = crate::PointDef::new(6, 1, true);
}
impl crate::Model for Model715 {
    const ID: u16 = 715;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            loc_rem_ctl: Self::LOC_REM_CTL.from_data(data)?,
            der_hb: Self::DER_HB.from_data(data)?,
            controller_hb: Self::CONTROLLER_HB.from_data(data)?,
            alarm_reset: Self::ALARM_RESET.from_data(data)?,
            op_ctl: Self::OP_CTL.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m715
    }
}
/// Control Mode
///
/// DER control mode. Enumeration.
///
/// Comments: DER Controls
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum LocRemCtl {
    /// Remote Control
    Remote = 0,
    /// Local Control
    ///
    /// Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
    Local = 1,
}
impl crate::Value for LocRemCtl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<LocRemCtl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                LocRemCtl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Set Operation
///
/// Commands to PCS. Enumerated value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum OpCtl {
    /// Stop the DER
    Stop = 0,
    /// Start the DER
    Start = 1,
    /// Enter Standby Mode
    EnterStandby = 2,
    /// Exit Standby Mode
    ExitStandby = 3,
}
impl crate::Value for OpCtl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<OpCtl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                OpCtl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
