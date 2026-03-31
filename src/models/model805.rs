//! Lithium-Ion Module Model
/// Type alias for [`LithiumIonModule`].
pub type Model805 = LithiumIonModule;
/// Lithium-Ion Module Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct LithiumIonModule {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Detail: Indices are one-based.
    pub str_idx: u16,
    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Detail: Indices are one-based.
    pub mod_idx: u16,
    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    pub n_cell: u16,
    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    pub soc: Option<u16>,
    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Detail: Measurement.
    pub do_d: Option<u16>,
    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    pub soh: Option<u16>,
    /// Cycle Count
    ///
    /// Count of cycles executed.
    pub n_cyc: Option<u32>,
    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Detail: Measurement.
    pub v: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Detail: Measurement.
    pub cell_v_max: u16,
    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    pub cell_v_max_cell: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Detail: Measurement.
    pub cell_v_min: u16,
    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    pub cell_v_min_cell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Detail: Calculation based on measurements.
    pub cell_v_avg: u16,
    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Detail: Measurement.
    pub cell_tmp_max: i16,
    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    pub cell_tmp_max_cell: Option<u16>,
    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Detail: Measurement.
    pub cell_tmp_min: i16,
    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    pub cell_tmp_min_cell: Option<u16>,
    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Detail: Calculation based on measurements.
    pub cell_tmp_avg: i16,
    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    pub n_cell_bal: Option<u16>,
    /// Serial Number
    ///
    /// Serial number for the module.
    pub sn: Option<String>,
    /// Scale factor for module state of charge.
    pub soc_sf: Option<i16>,
    /// Scale factor for module state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for module depth of discharge.
    pub do_d_sf: Option<i16>,
    /// Scale factor for module voltage.
    pub v_sf: i16,
    /// Scale factor for cell voltage.
    pub cell_v_sf: i16,
    /// Scale factor for module temperature.
    pub tmp_sf: i16,
    #[allow(missing_docs)]
    pub lithium_ion_module_cell: Vec<LithiumIonModuleCell>,
}
#[allow(missing_docs)]
impl LithiumIonModule {
    pub const STR_IDX: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const MOD_IDX: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const N_CELL: crate::Point<Self, u16> = crate::Point::new(2, 1, false);
    pub const SOC: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const DO_D: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const SOH: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const N_CYC: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2, false);
    pub const V: crate::Point<Self, u16> = crate::Point::new(8, 1, false);
    pub const CELL_V_MAX: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const CELL_V_MAX_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const CELL_V_MIN: crate::Point<Self, u16> = crate::Point::new(11, 1, false);
    pub const CELL_V_MIN_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const CELL_V_AVG: crate::Point<Self, u16> = crate::Point::new(13, 1, false);
    pub const CELL_TMP_MAX: crate::Point<Self, i16> = crate::Point::new(14, 1, false);
    pub const CELL_TMP_MAX_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(15, 1, false);
    pub const CELL_TMP_MIN: crate::Point<Self, i16> = crate::Point::new(16, 1, false);
    pub const CELL_TMP_MIN_CELL: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const CELL_TMP_AVG: crate::Point<Self, i16> = crate::Point::new(18, 1, false);
    pub const N_CELL_BAL: crate::Point<Self, Option<u16>> = crate::Point::new(19, 1, false);
    pub const SN: crate::Point<Self, Option<String>> = crate::Point::new(20, 16, false);
    pub const SOC_SF: crate::Point<Self, Option<i16>> = crate::Point::new(36, 1, false);
    pub const SOH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(37, 1, false);
    pub const DO_D_SF: crate::Point<Self, Option<i16>> = crate::Point::new(38, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(39, 1, false);
    pub const CELL_V_SF: crate::Point<Self, i16> = crate::Point::new(40, 1, false);
    pub const TMP_SF: crate::Point<Self, i16> = crate::Point::new(41, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::STR_IDX.is_invalid(&self.str_idx)
            || Self::MOD_IDX.is_invalid(&self.mod_idx)
            || Self::N_CELL.is_invalid(&self.n_cell)
            || Self::V.is_invalid(&self.v)
            || Self::CELL_V_MAX.is_invalid(&self.cell_v_max)
            || Self::CELL_V_MIN.is_invalid(&self.cell_v_min)
            || Self::CELL_V_AVG.is_invalid(&self.cell_v_avg)
            || Self::CELL_TMP_MAX.is_invalid(&self.cell_tmp_max)
            || Self::CELL_TMP_MIN.is_invalid(&self.cell_tmp_min)
            || Self::CELL_TMP_AVG.is_invalid(&self.cell_tmp_avg)
            || Self::V_SF.is_invalid(&self.v_sf)
            || Self::CELL_V_SF.is_invalid(&self.cell_v_sf)
            || Self::TMP_SF.is_invalid(&self.tmp_sf)
            || self
                .lithium_ion_module_cell
                .iter()
                .any(|group| group.has_invalid_points())
    }
}
impl crate::Group for LithiumIonModule {
    const LEN: u16 = 42;
}
impl LithiumIonModule {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, lithium_ion_module_cell) =
            LithiumIonModuleCell::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                str_idx: Self::STR_IDX.from_data(data)?,
                mod_idx: Self::MOD_IDX.from_data(data)?,
                n_cell: Self::N_CELL.from_data(data)?,
                soc: Self::SOC.from_data(data)?,
                do_d: Self::DO_D.from_data(data)?,
                soh: Self::SOH.from_data(data)?,
                n_cyc: Self::N_CYC.from_data(data)?,
                v: Self::V.from_data(data)?,
                cell_v_max: Self::CELL_V_MAX.from_data(data)?,
                cell_v_max_cell: Self::CELL_V_MAX_CELL.from_data(data)?,
                cell_v_min: Self::CELL_V_MIN.from_data(data)?,
                cell_v_min_cell: Self::CELL_V_MIN_CELL.from_data(data)?,
                cell_v_avg: Self::CELL_V_AVG.from_data(data)?,
                cell_tmp_max: Self::CELL_TMP_MAX.from_data(data)?,
                cell_tmp_max_cell: Self::CELL_TMP_MAX_CELL.from_data(data)?,
                cell_tmp_min: Self::CELL_TMP_MIN.from_data(data)?,
                cell_tmp_min_cell: Self::CELL_TMP_MIN_CELL.from_data(data)?,
                cell_tmp_avg: Self::CELL_TMP_AVG.from_data(data)?,
                n_cell_bal: Self::N_CELL_BAL.from_data(data)?,
                sn: Self::SN.from_data(data)?,
                soc_sf: Self::SOC_SF.from_data(data)?,
                soh_sf: Self::SOH_SF.from_data(data)?,
                do_d_sf: Self::DO_D_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                cell_v_sf: Self::CELL_V_SF.from_data(data)?,
                tmp_sf: Self::TMP_SF.from_data(data)?,
                lithium_ion_module_cell,
            },
        ))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct LithiumIonModuleCell {
    /// Cell Voltage
    ///
    /// Cell terminal voltage.
    pub cell_v: u16,
    /// Cell Temperature
    ///
    /// Cell temperature.
    pub cell_tmp: i16,
    /// Cell Status
    ///
    /// Status of the cell.
    pub cell_st: Option<LithiumIonModuleCellCellSt>,
}
#[allow(missing_docs)]
impl LithiumIonModuleCell {
    pub const CELL_V: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const CELL_TMP: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const CELL_ST: crate::Point<Self, Option<LithiumIonModuleCellCellSt>> =
        crate::Point::new(2, 2, false);
    fn has_invalid_points(&self) -> bool {
        Self::CELL_V.is_invalid(&self.cell_v) || Self::CELL_TMP.is_invalid(&self.cell_tmp)
    }
}
impl crate::Group for LithiumIonModuleCell {
    const LEN: u16 = 4;
}
impl LithiumIonModuleCell {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                cell_v: Self::CELL_V.from_data(data)?,
                cell_tmp: Self::CELL_TMP.from_data(data)?,
                cell_st: Self::CELL_ST.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<LithiumIonModuleCell as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = LithiumIonModuleCell::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    #[doc = " Cell Status"] #[doc = " "] #[doc = " Status of the cell."] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct
    LithiumIonModuleCellCellSt : u32 { #[allow(missing_docs)] const CellIsBalancing = 1;
    }
}
impl crate::Value for LithiumIonModuleCellCellSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for LithiumIonModuleCellCellSt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for LithiumIonModule {
    const ID: u16 = 805;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m805
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        if model.has_invalid_points() {
            Err(crate::ParseError::InvalidPointData(
                crate::InvalidPointData { model },
            ))
        } else {
            Ok(model)
        }
    }
}
