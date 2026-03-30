//! Base Met
pub type Model307 = BaseMet;
/// Base Met
///
/// Base Meteorological Model
///
/// Detail: This model supersedes model 301
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct BaseMet {
    /// Ambient Temperature
    pub tmp_amb: Option<i16>,
    /// Relative Humidity
    pub rh: Option<i16>,
    /// Barometric Pressure
    pub pres: Option<i16>,
    /// Wind Speed
    pub wnd_spd: Option<i16>,
    /// Wind Direction
    pub wnd_dir: Option<i16>,
    /// Rainfall
    pub rain: Option<i16>,
    /// Snow Depth
    pub snw: Option<i16>,
    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    pub ppt: Option<i16>,
    /// Electric Field
    pub elec_fld: Option<i16>,
    /// Surface Wetness
    pub sur_wet: Option<i16>,
    /// Soil Wetness
    pub soil_wet: Option<i16>,
}
#[allow(missing_docs)]
impl BaseMet {
    pub const TMP_AMB: crate::Point<Self, Option<i16>> = crate::Point::new(0, 1, false);
    pub const RH: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const PRES: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const WND_SPD: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const WND_DIR: crate::Point<Self, Option<i16>> = crate::Point::new(4, 1, false);
    pub const RAIN: crate::Point<Self, Option<i16>> = crate::Point::new(5, 1, false);
    pub const SNW: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1, false);
    pub const PPT: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, false);
    pub const ELEC_FLD: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const SUR_WET: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const SOIL_WET: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, false);
}
impl crate::Group for BaseMet {
    const LEN: u16 = 11;
}
impl BaseMet {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                tmp_amb: Self::TMP_AMB.from_data(data)?,
                rh: Self::RH.from_data(data)?,
                pres: Self::PRES.from_data(data)?,
                wnd_spd: Self::WND_SPD.from_data(data)?,
                wnd_dir: Self::WND_DIR.from_data(data)?,
                rain: Self::RAIN.from_data(data)?,
                snw: Self::SNW.from_data(data)?,
                ppt: Self::PPT.from_data(data)?,
                elec_fld: Self::ELEC_FLD.from_data(data)?,
                sur_wet: Self::SUR_WET.from_data(data)?,
                soil_wet: Self::SOIL_WET.from_data(data)?,
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
}
impl crate::Model for BaseMet {
    const ID: u16 = 307;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m307
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
