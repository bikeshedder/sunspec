/// Flow Battery String Model
#[derive(Debug)]
pub struct Model807 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Number of modules in this string.
    pub nmod: u16,
    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    pub nmodcon: u16,
    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modvmax: u16,
    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    pub modvmaxmod: Option<u16>,
    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modvmin: u16,
    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    pub modvminmod: Option<u16>,
    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub modvavg: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmax: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    pub cellvmaxmod: Option<u16>,
    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    pub cellvmaxstk: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmin: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    pub cellvminmod: Option<u16>,
    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    pub cellvminstk: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: Option<u16>,
    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmpmax: i16,
    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    pub tmpmaxmod: Option<u16>,
    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub tmpmin: i16,
    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    pub tmpminmod: Option<u16>,
    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub tmpavg: i16,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evtvnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evtvnd2: u32,
    #[allow(missing_docs)]
    pub modv_sf: i16,
    /// Scale factor for voltage.
    pub cellv_sf: i16,
    /// Scale factor for temperature.
    pub tmp_sf: i16,
    /// Scale factor for state of charge.
    pub soc_sf: i16,
    /// Scale factor for open circuit voltage.
    pub ocv_sf: i16,
}

#[allow(missing_docs)]

impl Model807 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NMOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const NMODCON: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const MODVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const MODVMAXMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const MODVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const MODVMINMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const MODVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, false);
    pub const CELLVMAXSTK: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const CELLVMINSTK: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const TMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const TMPMAXMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const TMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const TMPMINMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(18, 1, false);
    pub const TMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(20, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const MODV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const OCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
}

impl crate::Model for Model807 {
    const ID: u16 = 807;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            idx: Self::IDX.from_data(data)?,
            nmod: Self::NMOD.from_data(data)?,
            nmodcon: Self::NMODCON.from_data(data)?,
            modvmax: Self::MODVMAX.from_data(data)?,
            modvmaxmod: Self::MODVMAXMOD.from_data(data)?,
            modvmin: Self::MODVMIN.from_data(data)?,
            modvminmod: Self::MODVMINMOD.from_data(data)?,
            modvavg: Self::MODVAVG.from_data(data)?,
            cellvmax: Self::CELLVMAX.from_data(data)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmaxstk: Self::CELLVMAXSTK.from_data(data)?,
            cellvmin: Self::CELLVMIN.from_data(data)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvminstk: Self::CELLVMINSTK.from_data(data)?,
            cellvavg: Self::CELLVAVG.from_data(data)?,
            tmpmax: Self::TMPMAX.from_data(data)?,
            tmpmaxmod: Self::TMPMAXMOD.from_data(data)?,
            tmpmin: Self::TMPMIN.from_data(data)?,
            tmpminmod: Self::TMPMINMOD.from_data(data)?,
            tmpavg: Self::TMPAVG.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            modv_sf: Self::MODV_SF.from_data(data)?,
            cellv_sf: Self::CELLV_SF.from_data(data)?,
            tmp_sf: Self::TMP_SF.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            ocv_sf: Self::OCV_SF.from_data(data)?,
        })
    }
}
