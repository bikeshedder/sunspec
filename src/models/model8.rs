//! Get Device Security Certificate
/// Get Device Security Certificate
///
/// Security model for PKI
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model8 {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    pub fmt: Fmt,
    /// N
    ///
    /// Number of registers to follow for the certificate
    pub n: u16,
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Model8 {
    pub const FMT: crate::Point<Self, Fmt> = crate::Point::new(0, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::FMT.is_invalid(&self.fmt)
            || Self::N.is_invalid(&self.n)
            || self
                .repeating
                .iter()
                .any(|group| group.has_invalid_points())
    }
}
impl crate::Group for Model8 {
    const LEN: u16 = 2;
}
impl Model8 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                fmt: Self::FMT.from_data(data)?,
                n: Self::N.from_data(data)?,
                repeating,
            },
        ))
    }
}
/// Format
///
/// X.509 format of the certificate. DER or PEM.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Fmt {
    #[allow(missing_docs)]
    None,
    #[allow(missing_docs)]
    X509Pem,
    #[allow(missing_docs)]
    X509Der,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Fmt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::None,
            1 => Self::X509Pem,
            2 => Self::X509Der,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::None => 0,
            Self::X509Pem => 1,
            Self::X509Der => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Fmt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    /// Cert
    ///
    /// X.509 Certificate of the device
    pub cert: u16,
}
#[allow(missing_docs)]
impl Repeating {
    pub const CERT: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::CERT.is_invalid(&self.cert)
    }
}
impl crate::Group for Repeating {
    const LEN: u16 = 1;
}
impl Repeating {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                cert: Self::CERT.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Model8 {
    const ID: u16 = 8;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m8
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        if model.has_invalid_points() {
            Err(crate::ParseError::InvalidPointData(
                crate::InvalidPointData { model },
            ))
        } else {
            Ok(model)
        }
    }
}
