//! Cellular Link
/// Cellular Link
///
/// Include this model to support a cellular interface link
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model18 {
    /// Name
    ///
    /// Interface name
    pub nam: Option<String>,
    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    pub imei: Option<u32>,
    /// APN
    ///
    /// Access Point Name for the interface
    pub apn: Option<String>,
    /// Number
    ///
    /// Phone number for the interface
    pub num: Option<String>,
    /// PIN
    ///
    /// Personal Identification Number for the interface
    pub pin: Option<String>,
}
#[allow(missing_docs)]
impl Model18 {
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 4, true);
    pub const IMEI: crate::Point<Self, Option<u32>> = crate::Point::new(4, 2, true);
    pub const APN: crate::Point<Self, Option<String>> = crate::Point::new(6, 4, true);
    pub const NUM: crate::Point<Self, Option<String>> = crate::Point::new(10, 6, true);
    pub const PIN: crate::Point<Self, Option<String>> = crate::Point::new(16, 6, true);
}
impl crate::Model for Model18 {
    const ID: u16 = 18;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            imei: Self::IMEI.from_data(data)?,
            apn: Self::APN.from_data(data)?,
            num: Self::NUM.from_data(data)?,
            pin: Self::PIN.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m18
    }
}
