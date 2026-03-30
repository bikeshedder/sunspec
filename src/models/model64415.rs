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
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum LogEventEna {
    /// Disabled
    ///
    /// LogEvent Mode Disabled
    Disabled = 0,
    /// Enabled
    ///
    /// LogEvent Mode Enabled
    Enabled = 1,
}
impl crate::Value for LogEventEna {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<LogEventEna> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                LogEventEna::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// HTTP Message Mode Enable
///
/// Enable or disable the HTTP Message mode
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum HttpMsg {
    /// Disabled
    ///
    /// HTTP Message Mode Disabled
    Disabled = 0,
    /// Enabled
    ///
    /// HTTP Message Mode Enabled
    Enabled = 1,
}
impl crate::Value for HttpMsg {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<HttpMsg> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                HttpMsg::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// COMM-004 Certificate
///
/// Select COMM-004 certificate type
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Comm004Cert {
    /// DEFAULT
    ///
    /// Default Certificate
    DefaultCertificate = 0,
    /// COMM-004A
    ///
    /// Chain Length Two Certificate
    Comm004a = 1,
    /// COMM-004B
    ///
    /// Chain Length Three Certificate
    Comm004b = 2,
    /// COMM-004C
    ///
    /// Chain Length Four Certificate
    Comm004c = 3,
    /// COMM-004D
    ///
    /// Invalid MICA Extended Key Critical Value
    Comm004d = 4,
    /// COMM-004E
    ///
    /// Invalid MICA Name Non-Critical Value
    Comm004e = 5,
    /// COMM-004F
    ///
    /// Invalid MICA Policy Mapping Non-Critical Value
    Comm004f = 6,
    /// COMM-004G
    ///
    /// Self-signed device certificate
    Comm004g = 7,
}
impl crate::Value for Comm004Cert {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Comm004Cert> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Comm004Cert::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
impl crate::Model for CsipControl {
    const ID: u16 = 64415;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64415
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
