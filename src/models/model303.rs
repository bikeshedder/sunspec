//! Back of Module Temperature Model

/// Back of Module Temperature Model
///
/// Include to support variable number of  back of module temperature measurements
#[derive(Debug)]
pub struct Model303;

#[allow(missing_docs)]

impl Model303 {}

impl crate::Model for Model303 {
    const ID: u16 = 303;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}
