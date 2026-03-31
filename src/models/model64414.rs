//! DER Simulation Controls
/// Type alias for [`DerSimControls`].
pub type Model64414 = DerSimControls;
/// DER Simulation Controls
///
/// Configuration parameters for the DER device simulator.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerSimControls {
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    pub time: Option<String>,
    /// Ambient temperature (degrees Celsius)
    pub temperature: Option<f32>,
    /// The data source for the grid model. 'csv' or 'const'
    pub grid_model_source: Option<String>,
    /// The data source for the irradiance model. 'csv' or 'const'
    pub irradiance_model_source: Option<String>,
    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    pub irradiance: Option<f32>,
    /// Phase A RMS Voltage (pu) for the 'const' grid model
    pub grid_voltage_a: Option<f32>,
    /// Phase B RMS Voltage (pu) for the 'const' grid model
    pub grid_voltage_b: Option<f32>,
    /// Phase C RMS Voltage (pu) for the 'const' grid model
    pub grid_voltage_c: Option<f32>,
    /// Grid frequency (Hz) for the 'const' grid model
    pub grid_frequency: Option<f32>,
}
#[allow(missing_docs)]
impl DerSimControls {
    pub const TIME: crate::Point<Self, Option<String>> = crate::Point::new(0, 10, false);
    pub const TEMPERATURE: crate::Point<Self, Option<f32>> = crate::Point::new(10, 2, true);
    pub const GRID_MODEL_SOURCE: crate::Point<Self, Option<String>> =
        crate::Point::new(12, 32, true);
    pub const IRRADIANCE_MODEL_SOURCE: crate::Point<Self, Option<String>> =
        crate::Point::new(44, 32, true);
    pub const IRRADIANCE: crate::Point<Self, Option<f32>> = crate::Point::new(76, 2, true);
    pub const GRID_VOLTAGE_A: crate::Point<Self, Option<f32>> = crate::Point::new(78, 2, true);
    pub const GRID_VOLTAGE_B: crate::Point<Self, Option<f32>> = crate::Point::new(80, 2, true);
    pub const GRID_VOLTAGE_C: crate::Point<Self, Option<f32>> = crate::Point::new(82, 2, true);
    pub const GRID_FREQUENCY: crate::Point<Self, Option<f32>> = crate::Point::new(84, 2, true);
}
impl crate::Group for DerSimControls {
    const LEN: u16 = 86;
}
impl DerSimControls {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                time: Self::TIME.from_data(data)?,
                temperature: Self::TEMPERATURE.from_data(data)?,
                grid_model_source: Self::GRID_MODEL_SOURCE.from_data(data)?,
                irradiance_model_source: Self::IRRADIANCE_MODEL_SOURCE.from_data(data)?,
                irradiance: Self::IRRADIANCE.from_data(data)?,
                grid_voltage_a: Self::GRID_VOLTAGE_A.from_data(data)?,
                grid_voltage_b: Self::GRID_VOLTAGE_B.from_data(data)?,
                grid_voltage_c: Self::GRID_VOLTAGE_C.from_data(data)?,
                grid_frequency: Self::GRID_FREQUENCY.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for DerSimControls {
    const ID: u16 = 64414;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64414
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
