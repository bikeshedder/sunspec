//! Veris Status and Configuration
/// Veris Status and Configuration
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model64001 {
    /// Command Code
    pub cmd: Option<u16>,
    /// Hardware Revision
    pub hw_rev: Option<u16>,
    /// RS FW Revision
    pub rsfw_rev: Option<u16>,
    /// OS FW Revision
    pub osfw_rev: Option<u16>,
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
    pub le_dblink: Option<u16>,
    /// LED On Threshold
    pub le_don: Option<u16>,
    #[allow(missing_docs)]
    pub reserved: Option<u16>,
    /// Location String
    pub loc: Option<String>,
    /// Sensor 1 Unit ID
    pub s1id: Option<u16>,
    /// Sensor 1 Address
    pub s1_addr: Option<u16>,
    /// Sensor 1 OS Version
    pub s1os_ver: Option<u16>,
    /// Sensor 1 Product Version
    pub s1_ver: Option<String>,
    /// Sensor 1 Serial Num
    pub s1_serial: Option<String>,
    /// Sensor 2 Unit ID
    pub s2id: Option<u16>,
    /// Sensor 2 Address
    pub s2_addr: Option<u16>,
    /// Sensor 2 OS Version
    pub s2os_ver: Option<u16>,
    /// Sensor 2 Product Version
    pub s2_ver: Option<String>,
    /// Sensor 2 Serial Num
    pub s2_serial: Option<String>,
    /// Sensor 3 Unit ID
    pub s3id: Option<u16>,
    /// Sensor 3 Address
    pub s3_addr: Option<u16>,
    /// Sensor 3 OS Version
    pub s3os_ver: Option<u16>,
    /// Sensor 3 Product Version
    pub s3_ver: Option<String>,
    /// Sensor 3 Serial Num
    pub s3_serial: Option<String>,
    /// Sensor 4 Unit ID
    pub s4id: Option<u16>,
    /// Sensor 4 Address
    pub s4_addr: Option<u16>,
    /// Sensor 4 OS Version
    pub s4os_ver: Option<u16>,
    /// Sensor 4 Product Version
    pub s4_ver: Option<String>,
    /// Sensor 4 Serial Num
    pub s4_serial: Option<String>,
}
#[allow(missing_docs)]
impl Model64001 {
    pub const CMD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const HW_REV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, false);
    pub const RSFW_REV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const OSFW_REV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
    pub const PROD_REV: crate::PointDef<Self, Option<String>> = crate::PointDef::new(4, 2, false);
    pub const BOOTS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const SWITCH: crate::PointDef<Self, Option<Switch>> = crate::PointDef::new(7, 1, false);
    pub const SENSORS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, false);
    pub const TALKING: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, false);
    pub const STATUS: crate::PointDef<Self, Option<Status>> = crate::PointDef::new(10, 1, false);
    pub const CONFIG: crate::PointDef<Self, Option<Config>> = crate::PointDef::new(11, 1, false);
    pub const LE_DBLINK: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const LE_DON: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, false);
    pub const RESERVED: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const LOC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(15, 16, false);
    pub const S1ID: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(31, 1, false);
    pub const S1_ADDR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(32, 1, false);
    pub const S1OS_VER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(33, 1, false);
    pub const S1_VER: crate::PointDef<Self, Option<String>> = crate::PointDef::new(34, 2, false);
    pub const S1_SERIAL: crate::PointDef<Self, Option<String>> = crate::PointDef::new(36, 5, false);
    pub const S2ID: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(41, 1, false);
    pub const S2_ADDR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(42, 1, false);
    pub const S2OS_VER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(43, 1, false);
    pub const S2_VER: crate::PointDef<Self, Option<String>> = crate::PointDef::new(44, 2, false);
    pub const S2_SERIAL: crate::PointDef<Self, Option<String>> = crate::PointDef::new(46, 5, false);
    pub const S3ID: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(51, 1, false);
    pub const S3_ADDR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(52, 1, false);
    pub const S3OS_VER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(53, 1, false);
    pub const S3_VER: crate::PointDef<Self, Option<String>> = crate::PointDef::new(54, 2, false);
    pub const S3_SERIAL: crate::PointDef<Self, Option<String>> = crate::PointDef::new(56, 5, false);
    pub const S4ID: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(61, 1, false);
    pub const S4_ADDR: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(62, 1, false);
    pub const S4OS_VER: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(63, 1, false);
    pub const S4_VER: crate::PointDef<Self, Option<String>> = crate::PointDef::new(64, 2, false);
    pub const S4_SERIAL: crate::PointDef<Self, Option<String>> = crate::PointDef::new(66, 5, false);
}
impl crate::Model for Model64001 {
    const ID: u16 = 64001;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            cmd: Self::CMD.from_data(data)?,
            hw_rev: Self::HW_REV.from_data(data)?,
            rsfw_rev: Self::RSFW_REV.from_data(data)?,
            osfw_rev: Self::OSFW_REV.from_data(data)?,
            prod_rev: Self::PROD_REV.from_data(data)?,
            boots: Self::BOOTS.from_data(data)?,
            switch: Self::SWITCH.from_data(data)?,
            sensors: Self::SENSORS.from_data(data)?,
            talking: Self::TALKING.from_data(data)?,
            status: Self::STATUS.from_data(data)?,
            config: Self::CONFIG.from_data(data)?,
            le_dblink: Self::LE_DBLINK.from_data(data)?,
            le_don: Self::LE_DON.from_data(data)?,
            reserved: Self::RESERVED.from_data(data)?,
            loc: Self::LOC.from_data(data)?,
            s1id: Self::S1ID.from_data(data)?,
            s1_addr: Self::S1_ADDR.from_data(data)?,
            s1os_ver: Self::S1OS_VER.from_data(data)?,
            s1_ver: Self::S1_VER.from_data(data)?,
            s1_serial: Self::S1_SERIAL.from_data(data)?,
            s2id: Self::S2ID.from_data(data)?,
            s2_addr: Self::S2_ADDR.from_data(data)?,
            s2os_ver: Self::S2OS_VER.from_data(data)?,
            s2_ver: Self::S2_VER.from_data(data)?,
            s2_serial: Self::S2_SERIAL.from_data(data)?,
            s3id: Self::S3ID.from_data(data)?,
            s3_addr: Self::S3_ADDR.from_data(data)?,
            s3os_ver: Self::S3OS_VER.from_data(data)?,
            s3_ver: Self::S3_VER.from_data(data)?,
            s3_serial: Self::S3_SERIAL.from_data(data)?,
            s4id: Self::S4ID.from_data(data)?,
            s4_addr: Self::S4_ADDR.from_data(data)?,
            s4os_ver: Self::S4OS_VER.from_data(data)?,
            s4_ver: Self::S4_VER.from_data(data)?,
            s4_serial: Self::S4_SERIAL.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64001
    }
}
bitflags::bitflags! {
    #[doc = " DIP Switches"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Switch : u16 {}
}
impl crate::Value for Switch {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Switch> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Switch::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
bitflags::bitflags! {
    #[doc = " System Status"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Status : u16 {}
}
impl crate::Value for Status {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Status> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Status::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
bitflags::bitflags! {
    #[doc = " System Configuration"] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Config : u16 {}
}
impl crate::Value for Config {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<Config> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(Config::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
