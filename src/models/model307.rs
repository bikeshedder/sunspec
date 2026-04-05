//! Base Met
/// Type alias for [`BaseMet`].
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
static BASE_MET_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "tmp_amb",
        label: "Ambient Temperature",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rh",
        label: "Relative Humidity",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "pres",
        label: "Barometric Pressure",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wnd_spd",
        label: "Wind Speed",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wnd_dir",
        label: "Wind Direction",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rain",
        label: "Rainfall",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "snw",
        label: "Snow Depth",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ppt",
        label: "Precipitation Type",
        description: "\u{a0}Precipitation Type (WMO 4680 SYNOP code reference)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "elec_fld",
        label: "Electric Field",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "sur_wet",
        label: "Surface Wetness",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soil_wet",
        label: "Soil Wetness",
        description: "",
        kind: crate::FieldKind::Point,
    },
];
static BASE_MET_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "base_met",
    label: "Base Met",
    description: "Base Meteorological Model",
    fields: BASE_MET_FIELDS,
};
impl crate::GroupMeta for BaseMet {
    fn group_info() -> &'static crate::GroupInfo {
        &BASE_MET_GROUP_INFO
    }
}
impl crate::Group for BaseMet {
    const LEN: u16 = 11;
}
impl BaseMet {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
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
}
impl crate::Model for BaseMet {
    const ID: u16 = 307;
    const NAME: &'static str = "base_met";
    const LABEL: &'static str = "Base Met";
    const DESCRIPTION: &'static str = "Base Meteorological Model";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m307
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
