//! Mini Met Model
/// Type alias for [`MiniMet`].
pub type Model308 = MiniMet;
/// Mini Met Model
///
/// Include to support a few basic measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct MiniMet {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// Temp
    ///
    /// Back of module temperature measurement
    pub tmp_bom: Option<i16>,
    /// Ambient Temperature
    pub tmp_amb: Option<i16>,
    /// Wind Speed
    pub wnd_spd: Option<u16>,
}
#[allow(missing_docs)]
impl MiniMet {
    pub const GHI: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const TMP_BOM: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const TMP_AMB: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const WND_SPD: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
}
static MINI_MET_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "ghi",
        label: "GHI",
        description: "Global Horizontal Irradiance",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_bom",
        label: "Temp",
        description: "Back of module temperature measurement",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tmp_amb",
        label: "Ambient Temperature",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wnd_spd",
        label: "Wind Speed",
        description: "",
        kind: crate::FieldKind::Point,
    },
];
static MINI_MET_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "mini_met",
    label: "Mini Met Model",
    description: "Include to support a few basic measurements",
    fields: MINI_MET_FIELDS,
};
impl crate::GroupMeta for MiniMet {
    fn group_info() -> &'static crate::GroupInfo {
        &MINI_MET_GROUP_INFO
    }
}
impl crate::Group for MiniMet {
    const LEN: u16 = 4;
}
impl MiniMet {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ghi: Self::GHI.from_data(data)?,
                tmp_bom: Self::TMP_BOM.from_data(data)?,
                tmp_amb: Self::TMP_AMB.from_data(data)?,
                wnd_spd: Self::WND_SPD.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for MiniMet {
    const ID: u16 = 308;
    const NAME: &'static str = "mini_met";
    const LABEL: &'static str = "Mini Met Model";
    const DESCRIPTION: &'static str = "Include to support a few basic measurements";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m308
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
