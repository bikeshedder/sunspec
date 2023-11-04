/// Veris Status and Configuration
#[derive(Debug)]
pub struct Model64001 {
    /// Command Code
    pub cmd: Option<u16>,
    /// Hardware Revision
    pub hwrev: Option<u16>,
    /// RS FW Revision
    pub rsfwrev: Option<u16>,
    /// OS FW Revision
    pub osfwrev: Option<u16>,
    /// Product Revision
    pub prodrev: Option<String>,
    /// Boot Count
    pub boots: Option<u16>,
    /// DIP Switches
    pub switch: Option<u16>,
    /// Num Detected Sensors
    pub sensors: Option<u16>,
    /// Num Communicating Sensors
    pub talking: Option<u16>,
    /// System Status
    pub status: Option<u16>,
    /// System Configuration
    pub config: Option<u16>,
    /// LED Blink Threshold
    pub ledblink: Option<u16>,
    /// LED On Threshold
    pub ledon: Option<u16>,
    #[allow(missing_docs)]
    pub reserved: Option<u16>,
    /// Location String
    pub loc: Option<String>,
    /// Sensor 1 Unit ID
    pub s1id: Option<u16>,
    /// Sensor 1 Address
    pub s1addr: Option<u16>,
    /// Sensor 1 OS Version
    pub s1osver: Option<u16>,
    /// Sensor 1 Product Version
    pub s1ver: Option<String>,
    /// Sensor 1 Serial Num
    pub s1serial: Option<String>,
    /// Sensor 2 Unit ID
    pub s2id: Option<u16>,
    /// Sensor 2 Address
    pub s2addr: Option<u16>,
    /// Sensor 2 OS Version
    pub s2osver: Option<u16>,
    /// Sensor 2 Product Version
    pub s2ver: Option<String>,
    /// Sensor 2 Serial Num
    pub s2serial: Option<String>,
    /// Sensor 3 Unit ID
    pub s3id: Option<u16>,
    /// Sensor 3 Address
    pub s3addr: Option<u16>,
    /// Sensor 3 OS Version
    pub s3osver: Option<u16>,
    /// Sensor 3 Product Version
    pub s3ver: Option<String>,
    /// Sensor 3 Serial Num
    pub s3serial: Option<String>,
    /// Sensor 4 Unit ID
    pub s4id: Option<u16>,
    /// Sensor 4 Address
    pub s4addr: Option<u16>,
    /// Sensor 4 OS Version
    pub s4osver: Option<u16>,
    /// Sensor 4 Product Version
    pub s4ver: Option<String>,
    /// Sensor 4 Serial Num
    pub s4serial: Option<String>,
}

#[allow(missing_docs)]

impl Model64001 {
    pub const CMD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const HWREV: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const RSFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const OSFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const PRODREV: crate::PointDef<Self, String> = crate::PointDef::new(4, 2, false);
    pub const BOOTS: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const SWITCH: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const SENSORS: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const TALKING: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const STATUS: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const CONFIG: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const LEDBLINK: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const LEDON: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const RESERVED: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const LOC: crate::PointDef<Self, String> = crate::PointDef::new(15, 16, false);
    pub const S1ID: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, false);
    pub const S1ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const S1OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(33, 1, false);
    pub const S1VER: crate::PointDef<Self, String> = crate::PointDef::new(34, 2, false);
    pub const S1SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(36, 5, false);
    pub const S2ID: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const S2ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, false);
    pub const S2OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(43, 1, false);
    pub const S2VER: crate::PointDef<Self, String> = crate::PointDef::new(44, 2, false);
    pub const S2SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(46, 5, false);
    pub const S3ID: crate::PointDef<Self, u16> = crate::PointDef::new(51, 1, false);
    pub const S3ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(52, 1, false);
    pub const S3OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(53, 1, false);
    pub const S3VER: crate::PointDef<Self, String> = crate::PointDef::new(54, 2, false);
    pub const S3SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(56, 5, false);
    pub const S4ID: crate::PointDef<Self, u16> = crate::PointDef::new(61, 1, false);
    pub const S4ADDR: crate::PointDef<Self, u16> = crate::PointDef::new(62, 1, false);
    pub const S4OSVER: crate::PointDef<Self, u16> = crate::PointDef::new(63, 1, false);
    pub const S4VER: crate::PointDef<Self, String> = crate::PointDef::new(64, 2, false);
    pub const S4SERIAL: crate::PointDef<Self, String> = crate::PointDef::new(66, 5, false);
}

impl crate::Model for Model64001 {
    const ID: u16 = 64001;
    const LENGTH: u16 = 71;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            cmd: Self::CMD.from_data(data)?,
            hwrev: Self::HWREV.from_data(data)?,
            rsfwrev: Self::RSFWREV.from_data(data)?,
            osfwrev: Self::OSFWREV.from_data(data)?,
            prodrev: Self::PRODREV.from_data(data)?,
            boots: Self::BOOTS.from_data(data)?,
            switch: Self::SWITCH.from_data(data)?,
            sensors: Self::SENSORS.from_data(data)?,
            talking: Self::TALKING.from_data(data)?,
            status: Self::STATUS.from_data(data)?,
            config: Self::CONFIG.from_data(data)?,
            ledblink: Self::LEDBLINK.from_data(data)?,
            ledon: Self::LEDON.from_data(data)?,
            reserved: Self::RESERVED.from_data(data)?,
            loc: Self::LOC.from_data(data)?,
            s1id: Self::S1ID.from_data(data)?,
            s1addr: Self::S1ADDR.from_data(data)?,
            s1osver: Self::S1OSVER.from_data(data)?,
            s1ver: Self::S1VER.from_data(data)?,
            s1serial: Self::S1SERIAL.from_data(data)?,
            s2id: Self::S2ID.from_data(data)?,
            s2addr: Self::S2ADDR.from_data(data)?,
            s2osver: Self::S2OSVER.from_data(data)?,
            s2ver: Self::S2VER.from_data(data)?,
            s2serial: Self::S2SERIAL.from_data(data)?,
            s3id: Self::S3ID.from_data(data)?,
            s3addr: Self::S3ADDR.from_data(data)?,
            s3osver: Self::S3OSVER.from_data(data)?,
            s3ver: Self::S3VER.from_data(data)?,
            s3serial: Self::S3SERIAL.from_data(data)?,
            s4id: Self::S4ID.from_data(data)?,
            s4addr: Self::S4ADDR.from_data(data)?,
            s4osver: Self::S4OSVER.from_data(data)?,
            s4ver: Self::S4VER.from_data(data)?,
            s4serial: Self::S4SERIAL.from_data(data)?,
        })
    }
}
