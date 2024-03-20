//! Flow Battery Stack Model

/// Flow Battery Stack Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model809 {
    /// Stack Points To Be Determined
    pub stack_tbd: u16,
}

#[allow(missing_docs)]

impl Model809 {
    pub const STACK_TBD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
}

impl crate::Model for Model809 {
    const ID: u16 = 809;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stack_tbd: Self::STACK_TBD.from_data(data)?,
        })
    }
}
