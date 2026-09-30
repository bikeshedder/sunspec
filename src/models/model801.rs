//! Energy Storage Base Model (DEPRECATED)
/// Type alias for [`Storage`].
pub type Model801 = Storage;
/// Energy Storage Base Model (DEPRECATED)
///
/// This model has been deprecated.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Storage {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    pub deprecated: u16,
}
#[allow(missing_docs)]
impl Storage {
    pub const DEPRECATED: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::sealed::Sealed for Storage {}
impl crate::Group for Storage {
    const LEN: u16 = 1;
}
impl Storage {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data.get(1..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                deprecated: Self::DEPRECATED.from_data(data)?,
            },
        ))
    }
}
impl From<Storage> for crate::AnyModel {
    fn from(model: Storage) -> Self {
        Self::M801(model)
    }
}
impl crate::Model for Storage {
    const ID: u16 = 801;
    const NAME: &'static str = "storage";
    const LABEL: &'static str = "Energy Storage Base Model (DEPRECATED)";
    fn addr(models: &crate::Models) -> Option<crate::ModelAddr<Self>> {
        models.m801
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
