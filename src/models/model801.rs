//! Energy Storage Base Model (DEPRECATED)
pub type Model801 = Storage;
/// Energy Storage Base Model (DEPRECATED)
///
/// This model has been deprecated.
#[derive(Debug)]
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
impl crate::Group for Storage {
    const LEN: u16 = 1;
}
impl Storage {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                deprecated: Self::DEPRECATED.from_data(data)?,
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
}
impl crate::Model for Storage {
    const ID: u16 = 801;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m801
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
