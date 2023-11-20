/// Flow Battery Model
#[derive(Debug)]
pub struct Model806 {
    /// Battery Points To Be Determined
    pub battbd: u16,
}

#[allow(missing_docs)]

impl Model806 {
    pub const BATTBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model806 {
    const ID: u16 = 806;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            battbd: Self::BATTBD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
