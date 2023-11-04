/// Mersen GreenString
#[derive(Debug)]
pub struct Model64020 {
    /// Aux 0 temperature
    pub aux0tmp: Option<i16>,
    /// Aux 1 temperature
    pub aux1tmp: Option<i16>,
    /// Aux 2 temperature
    pub aux2tmp: Option<i16>,
    /// Aux 3 temperature
    pub aux3tmp: Option<i16>,
    /// Aux 4 temperature
    pub aux4tmp: Option<i16>,
    /// Probe Temperature
    pub probetmp: i16,
    /// Main Temperature
    pub maintmp: i16,
    /// Voltage scale factor for the sensors
    pub sensorv_sf: i16,
    /// Current scale factor for the sensors
    pub sensora_sf: i16,
    /// Frequency scale factor for the sensors
    pub sensorhz_sf: i16,
    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    pub sensor1voltage: Option<i16>,
    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    pub sensor2voltage: Option<i16>,
    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    pub sensor3voltage: Option<i16>,
    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    pub sensor4voltage: Option<i16>,
    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    pub sensor5voltage: Option<i16>,
    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    pub sensor6voltage: Option<i16>,
    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    pub sensor7voltage: Option<i16>,
    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    pub sensor1current: Option<i16>,
    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor2current: Option<i16>,
    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor3current: Option<i16>,
    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor4current: Option<i16>,
    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor5current: Option<i16>,
    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor6current: Option<i16>,
    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor7current: Option<i16>,
    /// Sensor8 frequency
    ///
    /// frequency in Hz
    pub sensor8: Option<u16>,
    /// Relay 1 state
    pub relay1: Option<u16>,
    /// Relay 2 state
    pub relay2: Option<u16>,
    /// Relay 3 state
    pub relay3: Option<u16>,
    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    pub resetaccumulators: Option<u16>,
    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    pub reset: Option<u16>,
}

#[allow(missing_docs)]

impl Model64020 {
    pub const AUX0TMP: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const AUX1TMP: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const AUX2TMP: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const AUX3TMP: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const AUX4TMP: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PROBETMP: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const MAINTMP: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const SENSORV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const SENSORA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const SENSORHZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const SENSOR1VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const SENSOR2VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const SENSOR3VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const SENSOR4VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const SENSOR5VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const SENSOR6VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const SENSOR7VOLTAGE: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const SENSOR1CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const SENSOR2CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const SENSOR3CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const SENSOR4CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const SENSOR5CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const SENSOR6CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const SENSOR7CURRENT: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const SENSOR8: crate::PointDef<Self, u16> = crate::PointDef::new(24, 1, false);
    pub const RELAY1: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const RELAY2: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, false);
    pub const RELAY3: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const RESETACCUMULATORS: crate::PointDef<Self, u16> = crate::PointDef::new(28, 1, false);
    pub const RESET: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
}

impl crate::Model for Model64020 {
    const ID: u16 = 64020;
    const LENGTH: u16 = 46;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aux0tmp: Self::AUX0TMP.from_data(data)?,
            aux1tmp: Self::AUX1TMP.from_data(data)?,
            aux2tmp: Self::AUX2TMP.from_data(data)?,
            aux3tmp: Self::AUX3TMP.from_data(data)?,
            aux4tmp: Self::AUX4TMP.from_data(data)?,
            probetmp: Self::PROBETMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            maintmp: Self::MAINTMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensorv_sf: Self::SENSORV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensora_sf: Self::SENSORA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensorhz_sf: Self::SENSORHZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            sensor1voltage: Self::SENSOR1VOLTAGE.from_data(data)?,
            sensor2voltage: Self::SENSOR2VOLTAGE.from_data(data)?,
            sensor3voltage: Self::SENSOR3VOLTAGE.from_data(data)?,
            sensor4voltage: Self::SENSOR4VOLTAGE.from_data(data)?,
            sensor5voltage: Self::SENSOR5VOLTAGE.from_data(data)?,
            sensor6voltage: Self::SENSOR6VOLTAGE.from_data(data)?,
            sensor7voltage: Self::SENSOR7VOLTAGE.from_data(data)?,
            sensor1current: Self::SENSOR1CURRENT.from_data(data)?,
            sensor2current: Self::SENSOR2CURRENT.from_data(data)?,
            sensor3current: Self::SENSOR3CURRENT.from_data(data)?,
            sensor4current: Self::SENSOR4CURRENT.from_data(data)?,
            sensor5current: Self::SENSOR5CURRENT.from_data(data)?,
            sensor6current: Self::SENSOR6CURRENT.from_data(data)?,
            sensor7current: Self::SENSOR7CURRENT.from_data(data)?,
            sensor8: Self::SENSOR8.from_data(data)?,
            relay1: Self::RELAY1.from_data(data)?,
            relay2: Self::RELAY2.from_data(data)?,
            relay3: Self::RELAY3.from_data(data)?,
            resetaccumulators: Self::RESETACCUMULATORS.from_data(data)?,
            reset: Self::RESET.from_data(data)?,
        })
    }
}
