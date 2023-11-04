/// Lithium-Ion Module Model
#[derive(Debug)]
pub struct Model805 {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Notes: Indices are one-based.
    pub stridx: u16,
    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Notes: Indices are one-based.
    pub modidx: u16,
    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    pub ncell: u16,
    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    pub soc: Option<u16>,
    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Notes: Measurement.
    pub dod: Option<u16>,
    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    pub soh: Option<u16>,
    /// Cycle Count
    ///
    /// Count of cycles executed.
    pub ncyc: Option<u32>,
    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Notes: Measurement.
    pub v: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Notes: Measurement.
    pub cellvmax: u16,
    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    pub cellvmaxcell: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Notes: Measurement.
    pub cellvmin: u16,
    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    pub cellvmincell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Notes: Calculation based on measurements.
    pub cellvavg: u16,
    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Notes: Measurement.
    pub celltmpmax: i16,
    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    pub celltmpmaxcell: Option<u16>,
    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Notes: Measurement.
    pub celltmpmin: i16,
    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    pub celltmpmincell: Option<u16>,
    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Notes: Calculation based on measurements.
    pub celltmpavg: i16,
    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    pub ncellbal: Option<u16>,
    /// Serial Number
    ///
    /// Serial number for the module.
    pub sn: Option<String>,
    /// Scale factor for module state of charge.
    pub soc_sf: Option<i16>,
    /// Scale factor for module state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for module depth of discharge.
    pub dod_sf: Option<i16>,
    /// Scale factor for module voltage.
    pub v_sf: i16,
    /// Scale factor for cell voltage.
    pub cellv_sf: i16,
    /// Scale factor for module temperature.
    pub tmp_sf: i16,
}

#[allow(missing_docs)]

impl Model805 {
    pub const STRIDX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const MODIDX: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const NCELL: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const SOC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const DOD: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const SOH: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NCYC: crate::PointDef<Self, u32> = crate::PointDef::new(6, 2, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CELLVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CELLVMAXCELL: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const CELLVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const CELLVMINCELL: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CELLVAVG: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CELLTMPMAX: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const CELLTMPMAXCELL: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const CELLTMPMIN: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const CELLTMPMINCELL: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const CELLTMPAVG: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const NCELLBAL: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const SN: crate::PointDef<Self, String> = crate::PointDef::new(20, 16, false);
    pub const SOC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(36, 1, false);
    pub const SOH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(37, 1, false);
    pub const DOD_SF: crate::PointDef<Self, i16> = crate::PointDef::new(38, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(39, 1, false);
    pub const CELLV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(40, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(41, 1, false);
}

impl crate::Model for Model805 {
    const ID: u16 = 805;
    const LENGTH: u16 = 46;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            stridx: Self::STRIDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modidx: Self::MODIDX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ncell: Self::NCELL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            soc: Self::SOC.from_data(data)?,
            dod: Self::DOD.from_data(data)?,
            soh: Self::SOH.from_data(data)?,
            ncyc: Self::NCYC.from_data(data)?,
            v: Self::V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmax: Self::CELLVMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmaxcell: Self::CELLVMAXCELL.from_data(data)?,
            cellvmin: Self::CELLVMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellvmincell: Self::CELLVMINCELL.from_data(data)?,
            cellvavg: Self::CELLVAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmax: Self::CELLTMPMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmaxcell: Self::CELLTMPMAXCELL.from_data(data)?,
            celltmpmin: Self::CELLTMPMIN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            celltmpmincell: Self::CELLTMPMINCELL.from_data(data)?,
            celltmpavg: Self::CELLTMPAVG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ncellbal: Self::NCELLBAL.from_data(data)?,
            sn: Self::SN.from_data(data)?,
            soc_sf: Self::SOC_SF.from_data(data)?,
            soh_sf: Self::SOH_SF.from_data(data)?,
            dod_sf: Self::DOD_SF.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cellv_sf: Self::CELLV_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
