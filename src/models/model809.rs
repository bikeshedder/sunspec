/// Flow Battery Stack Model
#[derive(Debug)]
pub struct Model809 {
    /// Stack Points To Be Determined
    pub stacktbd: u16,
}

#[allow(missing_docs)]

impl Model809 {
    pub const STACKTBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model809 {
    const ID: u16 = 809;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stacktbd: Self::STACKTBD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
