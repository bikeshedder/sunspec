/// Communication Interface Header
///
/// To be included first for a complete interface description
#[derive(Debug)]
pub struct Model10 {
    /// Interface Status
    ///
    /// Overall interface status
    pub st: u16,
    /// Interface Control
    ///
    /// Overall interface control (TBD)
    pub ctl: Option<u16>,
    /// Physical Access Type
    ///
    /// Enumerated value.  Type of physical media
    pub typ: Option<u16>,
}

#[allow(missing_docs)]

impl Model10 {
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const CTL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const TYP: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
}

impl crate::Model for Model10 {
    const ID: u16 = 10;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            st: Self::ST.from_data(data)?,
            ctl: Self::CTL.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
        })
    }
}
