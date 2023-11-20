/// Inclinometer Model
///
/// Include to support orientation measurements
#[derive(Debug)]
pub struct Model304;

#[allow(missing_docs)]

impl Model304 {}

impl crate::Model for Model304 {
    const ID: u16 = 304;
    fn from_data(_data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {})
    }
}
