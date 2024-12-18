//! Common
/// Common
///
/// All SunSpec compliant devices must include this as the first model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model1 {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    pub mn: String,
    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    pub md: String,
    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    pub opt: Option<String>,
    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    pub vr: Option<String>,
    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    pub sn: String,
    /// Device Address
    ///
    /// Modbus device address
    ///
    /// Detail: This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    pub da: Option<u16>,
}
#[allow(missing_docs)]
impl Model1 {
    pub const MN: crate::Point<Self, String> = crate::Point::new(0, 16, false);
    pub const MD: crate::Point<Self, String> = crate::Point::new(16, 16, false);
    pub const OPT: crate::Point<Self, Option<String>> = crate::Point::new(32, 8, false);
    pub const VR: crate::Point<Self, Option<String>> = crate::Point::new(40, 8, false);
    pub const SN: crate::Point<Self, String> = crate::Point::new(48, 16, false);
    pub const DA: crate::Point<Self, Option<u16>> = crate::Point::new(64, 1, true);
}
impl crate::Model for Model1 {
    const ID: u16 = 1;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            mn: Self::MN.from_data(data)?,
            md: Self::MD.from_data(data)?,
            opt: Self::OPT.from_data(data)?,
            vr: Self::VR.from_data(data)?,
            sn: Self::SN.from_data(data)?,
            da: Self::DA.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m1
    }
}
