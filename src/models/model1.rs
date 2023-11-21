/// Common
///
/// All SunSpec compliant devices must include this as the first model
#[derive(Debug)]
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
    pub da: Option<u16>,
}

#[allow(missing_docs)]

impl Model1 {
    pub const MN: crate::PointDef<Self, String> = crate::PointDef::new(0, 16, false);
    pub const MD: crate::PointDef<Self, String> = crate::PointDef::new(16, 16, false);
    pub const OPT: crate::PointDef<Self, Option<String>> = crate::PointDef::new(32, 8, false);
    pub const VR: crate::PointDef<Self, Option<String>> = crate::PointDef::new(40, 8, false);
    pub const SN: crate::PointDef<Self, String> = crate::PointDef::new(48, 16, false);
    pub const DA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(64, 1, true);
}

impl crate::Model for Model1 {
    const ID: u16 = 1;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            mn: Self::MN.from_data(data)?,
            md: Self::MD.from_data(data)?,
            opt: Self::OPT.from_data(data)?,
            vr: Self::VR.from_data(data)?,
            sn: Self::SN.from_data(data)?,
            da: Self::DA.from_data(data)?,
        })
    }
}
