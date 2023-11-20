/// Lithium-Ion Battery Bank Model
#[derive(Debug)]
pub struct Model803 {
    /// String Count
    ///
    /// Number of strings in the bank.
    pub nstr: u16,
    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    pub nstrcon: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub modtmpmax: i16,
    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    pub modtmpmaxstr: Option<u16>,
    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    pub modtmpmaxmod: Option<u16>,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Notes: Measurement.
    pub modtmpmin: i16,
    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    pub modtmpminstr: Option<u16>,
    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    pub modtmpminmod: Option<u16>,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub modtmpavg: Option<i16>,
    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub strvmax: Option<u16>,
    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    pub strvmaxstr: Option<u16>,
    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Notes: Measurement.
    pub strvmin: Option<u16>,
    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    pub strvminstr: Option<u16>,
    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub strvavg: Option<u16>,
    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub stramax: Option<i16>,
    /// Max String Current String
    ///
    /// String with the maximum current.
    pub stramaxstr: Option<u16>,
    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Notes: Measurement.
    pub stramin: Option<i16>,
    /// Min String Current String
    ///
    /// String with the minimum current.
    pub straminstr: Option<u16>,
    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Notes: Calculation based on measurements.
    pub straavg: Option<i16>,
    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    pub ncellbal: Option<u16>,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperatures.
    pub modtmp_sf: i16,
    /// Scale factor for string currents.
    pub a_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model803 {
    pub const NSTR: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NSTRCON: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const MODTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const MODTMPMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MODTMPMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const MODTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const MODTMPMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const MODTMPMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const MODTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const STRVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const STRVMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const STRVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const STRVMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const STRVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const STRAMAX: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const STRAMAXSTR: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const STRAMIN: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const STRAMINSTR: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const STRAAVG: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MODTMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
}

impl crate::Model for Model803 {
    const ID: u16 = 803;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nstr: Self::NSTR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            nstrcon: Self::NSTRCON
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmax: Self::MODTMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpmaxstr: Self::MODTMPMAXSTR.from_data(data)?,
            modtmpmaxmod: Self::MODTMPMAXMOD.from_data(data)?,
            modtmpmin: Self::MODTMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmpminstr: Self::MODTMPMINSTR.from_data(data)?,
            modtmpminmod: Self::MODTMPMINMOD.from_data(data)?,
            modtmpavg: Self::MODTMPAVG.from_data(data)?,
            strvmax: Self::STRVMAX.from_data(data)?,
            strvmaxstr: Self::STRVMAXSTR.from_data(data)?,
            strvmin: Self::STRVMIN.from_data(data)?,
            strvminstr: Self::STRVMINSTR.from_data(data)?,
            strvavg: Self::STRVAVG.from_data(data)?,
            stramax: Self::STRAMAX.from_data(data)?,
            stramaxstr: Self::STRAMAXSTR.from_data(data)?,
            stramin: Self::STRAMIN.from_data(data)?,
            straminstr: Self::STRAMINSTR.from_data(data)?,
            straavg: Self::STRAAVG.from_data(data)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modtmp_sf: Self::MODTMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            soc_sf: Self::SOC_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF.from_data(data)?,
        })
    }
}
