//! Veris Status and Configuration
/// Veris Status and Configuration
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model64001 {
    /// Command Code
    pub cmd: Option<u16>,
    /// Hardware Revision
    pub hw_rev: Option<u16>,
    /// RS FW Revision
    pub rs_fw_rev: Option<u16>,
    /// OS FW Revision
    pub os_fw_rev: Option<u16>,
    /// Product Revision
    pub prod_rev: Option<String>,
    /// Boot Count
    pub boots: Option<u16>,
    /// DIP Switches
    pub switch: Option<Switch>,
    /// Num Detected Sensors
    pub sensors: Option<u16>,
    /// Num Communicating Sensors
    pub talking: Option<u16>,
    /// System Status
    pub status: Option<Status>,
    /// System Configuration
    pub config: Option<Config>,
    /// LED Blink Threshold
    pub led_blink: Option<u16>,
    /// LED On Threshold
    pub led_on: Option<u16>,
    #[allow(missing_docs)]
    pub reserved: Option<u16>,
    /// Location String
    pub loc: Option<String>,
    /// Sensor 1 Unit ID
    pub s1_id: Option<u16>,
    /// Sensor 1 Address
    pub s1_addr: Option<u16>,
    /// Sensor 1 OS Version
    pub s1_os_ver: Option<u16>,
    /// Sensor 1 Product Version
    pub s1_ver: Option<String>,
    /// Sensor 1 Serial Num
    pub s1_serial: Option<String>,
    /// Sensor 2 Unit ID
    pub s2_id: Option<u16>,
    /// Sensor 2 Address
    pub s2_addr: Option<u16>,
    /// Sensor 2 OS Version
    pub s2_os_ver: Option<u16>,
    /// Sensor 2 Product Version
    pub s2_ver: Option<String>,
    /// Sensor 2 Serial Num
    pub s2_serial: Option<String>,
    /// Sensor 3 Unit ID
    pub s3_id: Option<u16>,
    /// Sensor 3 Address
    pub s3_addr: Option<u16>,
    /// Sensor 3 OS Version
    pub s3_os_ver: Option<u16>,
    /// Sensor 3 Product Version
    pub s3_ver: Option<String>,
    /// Sensor 3 Serial Num
    pub s3_serial: Option<String>,
    /// Sensor 4 Unit ID
    pub s4_id: Option<u16>,
    /// Sensor 4 Address
    pub s4_addr: Option<u16>,
    /// Sensor 4 OS Version
    pub s4_os_ver: Option<u16>,
    /// Sensor 4 Product Version
    pub s4_ver: Option<String>,
    /// Sensor 4 Serial Num
    pub s4_serial: Option<String>,
}
#[allow(missing_docs)]
impl Model64001 {
    pub const CMD: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const HW_REV: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const RS_FW_REV: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const OS_FW_REV: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const PROD_REV: crate::Point<Self, Option<String>> = crate::Point::new(4, 2, false);
    pub const BOOTS: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const SWITCH: crate::Point<Self, Option<Switch>> = crate::Point::new(7, 1, false);
    pub const SENSORS: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const TALKING: crate::Point<Self, Option<u16>> = crate::Point::new(9, 1, false);
    pub const STATUS: crate::Point<Self, Option<Status>> = crate::Point::new(10, 1, false);
    pub const CONFIG: crate::Point<Self, Option<Config>> = crate::Point::new(11, 1, false);
    pub const LED_BLINK: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const LED_ON: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
    pub const RESERVED: crate::Point<Self, Option<u16>> = crate::Point::new(14, 1, false);
    pub const LOC: crate::Point<Self, Option<String>> = crate::Point::new(15, 16, false);
    pub const S1_ID: crate::Point<Self, Option<u16>> = crate::Point::new(31, 1, false);
    pub const S1_ADDR: crate::Point<Self, Option<u16>> = crate::Point::new(32, 1, false);
    pub const S1_OS_VER: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, false);
    pub const S1_VER: crate::Point<Self, Option<String>> = crate::Point::new(34, 2, false);
    pub const S1_SERIAL: crate::Point<Self, Option<String>> = crate::Point::new(36, 5, false);
    pub const S2_ID: crate::Point<Self, Option<u16>> = crate::Point::new(41, 1, false);
    pub const S2_ADDR: crate::Point<Self, Option<u16>> = crate::Point::new(42, 1, false);
    pub const S2_OS_VER: crate::Point<Self, Option<u16>> = crate::Point::new(43, 1, false);
    pub const S2_VER: crate::Point<Self, Option<String>> = crate::Point::new(44, 2, false);
    pub const S2_SERIAL: crate::Point<Self, Option<String>> = crate::Point::new(46, 5, false);
    pub const S3_ID: crate::Point<Self, Option<u16>> = crate::Point::new(51, 1, false);
    pub const S3_ADDR: crate::Point<Self, Option<u16>> = crate::Point::new(52, 1, false);
    pub const S3_OS_VER: crate::Point<Self, Option<u16>> = crate::Point::new(53, 1, false);
    pub const S3_VER: crate::Point<Self, Option<String>> = crate::Point::new(54, 2, false);
    pub const S3_SERIAL: crate::Point<Self, Option<String>> = crate::Point::new(56, 5, false);
    pub const S4_ID: crate::Point<Self, Option<u16>> = crate::Point::new(61, 1, false);
    pub const S4_ADDR: crate::Point<Self, Option<u16>> = crate::Point::new(62, 1, false);
    pub const S4_OS_VER: crate::Point<Self, Option<u16>> = crate::Point::new(63, 1, false);
    pub const S4_VER: crate::Point<Self, Option<String>> = crate::Point::new(64, 2, false);
    pub const S4_SERIAL: crate::Point<Self, Option<String>> = crate::Point::new(66, 5, false);
}
impl crate::sealed::Sealed for Model64001 {}
impl crate::Group for Model64001 {
    const LEN: u16 = 71;
}
impl Model64001 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data.get(71..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                cmd: Self::CMD.from_data(data)?,
                hw_rev: Self::HW_REV.from_data(data)?,
                rs_fw_rev: Self::RS_FW_REV.from_data(data)?,
                os_fw_rev: Self::OS_FW_REV.from_data(data)?,
                prod_rev: Self::PROD_REV.from_data(data)?,
                boots: Self::BOOTS.from_data(data)?,
                switch: Self::SWITCH.from_data(data)?,
                sensors: Self::SENSORS.from_data(data)?,
                talking: Self::TALKING.from_data(data)?,
                status: Self::STATUS.from_data(data)?,
                config: Self::CONFIG.from_data(data)?,
                led_blink: Self::LED_BLINK.from_data(data)?,
                led_on: Self::LED_ON.from_data(data)?,
                reserved: Self::RESERVED.from_data(data)?,
                loc: Self::LOC.from_data(data)?,
                s1_id: Self::S1_ID.from_data(data)?,
                s1_addr: Self::S1_ADDR.from_data(data)?,
                s1_os_ver: Self::S1_OS_VER.from_data(data)?,
                s1_ver: Self::S1_VER.from_data(data)?,
                s1_serial: Self::S1_SERIAL.from_data(data)?,
                s2_id: Self::S2_ID.from_data(data)?,
                s2_addr: Self::S2_ADDR.from_data(data)?,
                s2_os_ver: Self::S2_OS_VER.from_data(data)?,
                s2_ver: Self::S2_VER.from_data(data)?,
                s2_serial: Self::S2_SERIAL.from_data(data)?,
                s3_id: Self::S3_ID.from_data(data)?,
                s3_addr: Self::S3_ADDR.from_data(data)?,
                s3_os_ver: Self::S3_OS_VER.from_data(data)?,
                s3_ver: Self::S3_VER.from_data(data)?,
                s3_serial: Self::S3_SERIAL.from_data(data)?,
                s4_id: Self::S4_ID.from_data(data)?,
                s4_addr: Self::S4_ADDR.from_data(data)?,
                s4_os_ver: Self::S4_OS_VER.from_data(data)?,
                s4_ver: Self::S4_VER.from_data(data)?,
                s4_serial: Self::S4_SERIAL.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    /// DIP Switches
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Switch: u16 {
    }
}
impl crate::sealed::Sealed for Switch {}
impl crate::Value for Switch {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Switch {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
bitflags::bitflags! {
    /// System Status
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Status: u16 {
    }
}
impl crate::sealed::Sealed for Status {}
impl crate::Value for Status {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Status {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
bitflags::bitflags! {
    /// System Configuration
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Config: u16 {
    }
}
impl crate::sealed::Sealed for Config {}
impl crate::Value for Config {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Config {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
impl From<Model64001> for crate::AnyModel {
    fn from(model: Model64001) -> Self {
        Self::M64001(model)
    }
}
impl crate::Model for Model64001 {
    const ID: u16 = 64001;
    const NAME: &'static str = "model_64001";
    const LABEL: &'static str = "Veris Status and Configuration";
    fn addr(models: &crate::Models) -> Option<crate::ModelAddr<Self>> {
        models.m64001
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
