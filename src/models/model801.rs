//! Energy Storage Base Model (DEPRECATED)
/// Type alias for [`Storage`].
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
static STORAGE_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "deprecated",
    label: "Deprecated Model",
    description: "This model has been deprecated.",
    kind: crate::FieldKind::Point,
}];
static STORAGE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "storage",
    label: "Energy Storage Base Model (DEPRECATED)",
    description: "This model has been deprecated.",
    fields: STORAGE_FIELDS,
};
impl crate::GroupMeta for Storage {
    fn group_info() -> &'static crate::GroupInfo {
        &STORAGE_GROUP_INFO
    }
}
impl crate::Group for Storage {
    const LEN: u16 = 1;
}
impl Storage {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                deprecated: Self::DEPRECATED.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for Storage {
    const ID: u16 = 801;
    const NAME: &'static str = "storage";
    const LABEL: &'static str = "Energy Storage Base Model (DEPRECATED)";
    const DESCRIPTION: &'static str = "This model has been deprecated.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m801
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
