//! CSIP Client Control
/// Type alias for [`CsipControl`].
pub type Model64415 = CsipControl;
/// CSIP Client Control
///
/// CSIP Client Control for Alarms and Error tests
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct CsipControl {
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    pub log_event_ena: Option<LogEventEna>,
    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    pub http_msg: Option<HttpMsg>,
    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    pub comm004_cert: Option<Comm004Cert>,
}
#[allow(missing_docs)]
impl CsipControl {
    pub const LOG_EVENT_ENA: crate::Point<Self, Option<LogEventEna>> =
        crate::Point::new(0, 1, true);
    pub const HTTP_MSG: crate::Point<Self, Option<HttpMsg>> = crate::Point::new(1, 1, true);
    pub const COMM004_CERT: crate::Point<Self, Option<Comm004Cert>> = crate::Point::new(2, 1, true);
}
impl crate::Group for CsipControl {
    const LEN: u16 = 3;
}
impl CsipControl {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                log_event_ena: Self::LOG_EVENT_ENA.from_data(data)?,
                http_msg: Self::HTTP_MSG.from_data(data)?,
                comm004_cert: Self::COMM004_CERT.from_data(data)?,
            },
        ))
    }
}
/// LogEvent Mode Enable
///
/// Enable or disable the LogEvent mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum LogEventEna {
    /// Disabled
    ///
    /// LogEvent Mode Disabled
    Disabled,
    /// Enabled
    ///
    /// LogEvent Mode Enabled
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for LogEventEna {
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
impl crate::FixedSize for LogEventEna {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// HTTP Message Mode Enable
///
/// Enable or disable the HTTP Message mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum HttpMsg {
    /// Disabled
    ///
    /// HTTP Message Mode Disabled
    Disabled,
    /// Enabled
    ///
    /// HTTP Message Mode Enabled
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for HttpMsg {
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
impl crate::FixedSize for HttpMsg {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// COMM-004 Certificate
///
/// Select COMM-004 certificate type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Comm004Cert {
    /// DEFAULT
    ///
    /// Default Certificate
    DefaultCertificate,
    /// COMM-004A
    ///
    /// Chain Length Two Certificate
    Comm004a,
    /// COMM-004B
    ///
    /// Chain Length Three Certificate
    Comm004b,
    /// COMM-004C
    ///
    /// Chain Length Four Certificate
    Comm004c,
    /// COMM-004D
    ///
    /// Invalid MICA Extended Key Critical Value
    Comm004d,
    /// COMM-004E
    ///
    /// Invalid MICA Name Non-Critical Value
    Comm004e,
    /// COMM-004F
    ///
    /// Invalid MICA Policy Mapping Non-Critical Value
    Comm004f,
    /// COMM-004G
    ///
    /// Self-signed device certificate
    Comm004g,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Comm004Cert {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::DefaultCertificate,
            1 => Self::Comm004a,
            2 => Self::Comm004b,
            3 => Self::Comm004c,
            4 => Self::Comm004d,
            5 => Self::Comm004e,
            6 => Self::Comm004f,
            7 => Self::Comm004g,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::DefaultCertificate => 0,
            Self::Comm004a => 1,
            Self::Comm004b => 2,
            Self::Comm004c => 3,
            Self::Comm004d => 4,
            Self::Comm004e => 5,
            Self::Comm004f => 6,
            Self::Comm004g => 7,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Comm004Cert {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for CsipControl {
    const ID: u16 = 64415;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64415
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
