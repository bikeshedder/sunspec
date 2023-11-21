/// Mersen GreenString
#[derive(Debug)]
pub struct Model64020 {
    /// Aux 0 temperature
    pub aux0_tmp: Option<i16>,
    /// Aux 1 temperature
    pub aux1_tmp: Option<i16>,
    /// Aux 2 temperature
    pub aux2_tmp: Option<i16>,
    /// Aux 3 temperature
    pub aux3_tmp: Option<i16>,
    /// Aux 4 temperature
    pub aux4_tmp: Option<i16>,
    /// Probe Temperature
    pub probe_tmp: i16,
    /// Main Temperature
    pub main_tmp: i16,
    /// Voltage scale factor for the sensors
    pub sensor_v_sf: i16,
    /// Current scale factor for the sensors
    pub sensor_a_sf: i16,
    /// Frequency scale factor for the sensors
    pub sensor_hz_sf: i16,
    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    pub sensor1_voltage: Option<i16>,
    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    pub sensor2_voltage: Option<i16>,
    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    pub sensor3_voltage: Option<i16>,
    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    pub sensor4_voltage: Option<i16>,
    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    pub sensor5_voltage: Option<i16>,
    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    pub sensor6_voltage: Option<i16>,
    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    pub sensor7_voltage: Option<i16>,
    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    pub sensor1_current: Option<i16>,
    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor2_current: Option<i16>,
    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor3_current: Option<i16>,
    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor4_current: Option<i16>,
    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor5_current: Option<i16>,
    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor6_current: Option<i16>,
    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    pub sensor7_current: Option<i16>,
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
    pub reset_accumulators: Option<u16>,
    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    pub reset: Option<u16>,
}

#[allow(missing_docs)]

impl Model64020 {
    pub const AUX0_TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(0, 1, false);
    pub const AUX1_TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const AUX2_TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const AUX3_TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const AUX4_TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const PROBE_TMP: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const MAIN_TMP: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const SENSOR_V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const SENSOR_A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const SENSOR_HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const SENSOR1_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(10, 1, false);
    pub const SENSOR2_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(11, 1, false);
    pub const SENSOR3_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(12, 1, false);
    pub const SENSOR4_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(13, 1, false);
    pub const SENSOR5_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(14, 1, false);
    pub const SENSOR6_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(15, 1, false);
    pub const SENSOR7_VOLTAGE: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(16, 1, false);
    pub const SENSOR1_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(17, 1, false);
    pub const SENSOR2_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(18, 1, false);
    pub const SENSOR3_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(19, 1, false);
    pub const SENSOR4_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(20, 1, false);
    pub const SENSOR5_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(21, 1, false);
    pub const SENSOR6_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(22, 1, false);
    pub const SENSOR7_CURRENT: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(23, 1, false);
    pub const SENSOR8: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(24, 1, false);
    pub const RELAY1: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(25, 1, false);
    pub const RELAY2: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(26, 1, false);
    pub const RELAY3: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(27, 1, false);
    pub const RESET_ACCUMULATORS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(28, 1, false);
    pub const RESET: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(29, 1, false);
}

impl crate::Model for Model64020 {
    const ID: u16 = 64020;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            aux0_tmp: Self::AUX0_TMP.from_data(data)?,
            aux1_tmp: Self::AUX1_TMP.from_data(data)?,
            aux2_tmp: Self::AUX2_TMP.from_data(data)?,
            aux3_tmp: Self::AUX3_TMP.from_data(data)?,
            aux4_tmp: Self::AUX4_TMP.from_data(data)?,
            probe_tmp: Self::PROBE_TMP.from_data(data)?,
            main_tmp: Self::MAIN_TMP.from_data(data)?,
            sensor_v_sf: Self::SENSOR_V_SF.from_data(data)?,
            sensor_a_sf: Self::SENSOR_A_SF.from_data(data)?,
            sensor_hz_sf: Self::SENSOR_HZ_SF.from_data(data)?,
            sensor1_voltage: Self::SENSOR1_VOLTAGE.from_data(data)?,
            sensor2_voltage: Self::SENSOR2_VOLTAGE.from_data(data)?,
            sensor3_voltage: Self::SENSOR3_VOLTAGE.from_data(data)?,
            sensor4_voltage: Self::SENSOR4_VOLTAGE.from_data(data)?,
            sensor5_voltage: Self::SENSOR5_VOLTAGE.from_data(data)?,
            sensor6_voltage: Self::SENSOR6_VOLTAGE.from_data(data)?,
            sensor7_voltage: Self::SENSOR7_VOLTAGE.from_data(data)?,
            sensor1_current: Self::SENSOR1_CURRENT.from_data(data)?,
            sensor2_current: Self::SENSOR2_CURRENT.from_data(data)?,
            sensor3_current: Self::SENSOR3_CURRENT.from_data(data)?,
            sensor4_current: Self::SENSOR4_CURRENT.from_data(data)?,
            sensor5_current: Self::SENSOR5_CURRENT.from_data(data)?,
            sensor6_current: Self::SENSOR6_CURRENT.from_data(data)?,
            sensor7_current: Self::SENSOR7_CURRENT.from_data(data)?,
            sensor8: Self::SENSOR8.from_data(data)?,
            relay1: Self::RELAY1.from_data(data)?,
            relay2: Self::RELAY2.from_data(data)?,
            relay3: Self::RELAY3.from_data(data)?,
            reset_accumulators: Self::RESET_ACCUMULATORS.from_data(data)?,
            reset: Self::RESET.from_data(data)?,
        })
    }
}
