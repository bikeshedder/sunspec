/// Energy Storage Base Model (DEPRECATED)
///
/// This model has been deprecated.
#[derive(Debug)]
pub struct Model801 {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    pub deprecated: u16,
}

#[allow(missing_docs)]

impl Model801 {
    pub const DEPRECATED: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model801 {
    const ID: u16 = 801;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            deprecated: Self::DEPRECATED
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
