/// Lithium-Ion String Model
#[derive(Debug)]
pub struct Model804 {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Notes: Indices are one-based.
    pub idx: u16,
    /// Module Count
    ///
    /// Count of modules in the string.
    pub nmod: u16,
    /// String Status
    ///
    /// Current status of the string.
    pub st: u32,
    /// Connection Failure Reason
    pub confail: Option<u16>,
    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    pub ncellbal: Option<u16>,
    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub soc: u16,
    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub dod: Option<u16>,
    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    pub ncyc: Option<u32>,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub soh: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    ///
    /// Notes: Measurement.
    pub a: i16,
    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Notes: Measurement.
    pub v: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmax: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    pub cellvmaxmod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Notes: Measurement.
    pub cellvmin: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    pub cellvminmod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modtmpmax: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    pub modtmpmaxmod: u16,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Notes: Measurement.
    pub modtmpmin: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    pub modtmpminmod: u16,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Notes: Calculation based on measurements.
    pub modtmpavg: i16,
    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    pub r#const: Option<u32>,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.  Bit flags.
    pub evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.  Bit flags.
    ///
    /// Notes: Reserved for future use.
    pub evt2: Option<u32>,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evtvnd2: Option<u32>,
    /// Enable/Disable String
    ///
    /// Enables and disables the string.  Should reset to 0 upon completion.
    pub setena: Option<u16>,
    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Notes: Should reset to 0 upon completion.
    pub setcon: Option<u16>,
    /// Scale factor for string state of charge.
    pub soc_sf: i16,
    /// Scale factor for string state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for string depth of discharge.
    pub dod_sf: Option<i16>,
    /// Scale factor for string current.
    pub a_sf: i16,
    /// Scale factor for string voltage.
    pub v_sf: Option<i16>,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperature.
    pub modtmp_sf: i16,
}

#[allow(missing_docs)]

impl Model804 {
    pub const IDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const NMOD: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ST: crate::PointDef<Self, u32> = crate::PointDef::new(2, 2, false);
    pub const CONFAIL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const DOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
    pub const NCYC: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(8, 2, false);
    pub const SOH: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const V: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELLVMAXMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const CELLVMINMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const MODTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const MODTMPMAXMOD: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const MODTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const MODTMPMINMOD: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const MODTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const CONST: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(24, 2, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const EVT2: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(28, 2, false);
    pub const EVTVND1: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(30, 2, false);
    pub const EVTVND2: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(32, 2, false);
    pub const SETENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(34, 1, true);
    pub const SETCON: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(35, 1, true);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(36, 1, false);
    pub const SOH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(37, 1, false);
    pub const DOD_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(38, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(39, 1, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(40, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(41, 1, false);
    pub const MODTMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
}

impl crate::Model for Model804 {
    const ID: u16 = 804;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            idx: Self::IDX.from_data(data)?,
            nmod: Self::NMOD.from_data(data)?,
            st: Self::ST.from_data(data)?,
            confail: Self::CONFAIL.from_data(data)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            soc: Self::SOC.from_data(data)?,
            dod: Self::DOD.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            a: Self::A.from_data(data)?,
            v: Self::V.from_data(data)?,
            cellvmax: Self::CELLVMAX.from_data(data)?,
            cellvmaxmod: Self::CELLVMAXMOD.from_data(data)?,
            cellvmin: Self::CELLVMIN.from_data(data)?,
            cellvminmod: Self::CELLVMINMOD.from_data(data)?,
            cellvavg: Self::CELLVAVG.from_data(data)?,
            modtmpmax: Self::MODTMPMAX.from_data(data)?,
            modtmpmaxmod: Self::MODTMPMAXMOD.from_data(data)?,
            modtmpmin: Self::MODTMPMIN.from_data(data)?,
            modtmpminmod: Self::MODTMPMINMOD.from_data(data)?,
            modtmpavg: Self::MODTMPAVG.from_data(data)?,
            r#const: Self::CONST.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            setena: Self::SETENA.from_data(data)?,
            setcon: Self::SETCON.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            cellv_sf: Self::CELLV_SF.from_data(data)?,
            modtmp_sf: Self::MODTMP_SF.from_data(data)?,
        })
    }
}
