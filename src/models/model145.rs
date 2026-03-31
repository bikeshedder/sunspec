//! Extended Settings
/// Type alias for [`ExtSettings`].
pub type Model145 = ExtSettings;
/// Extended Settings
///
/// Inverter controls extended settings
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ExtSettings {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    pub nom_rmp_up_rte: Option<u16>,
    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    pub nom_rmp_dn_rte: Option<u16>,
    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    pub emg_rmp_up_rte: Option<u16>,
    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    pub emg_rmp_dn_rte: Option<u16>,
    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    pub conn_rmp_up_rte: Option<u16>,
    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    pub conn_rmp_dn_rte: Option<u16>,
    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    pub a_gra: Option<u16>,
    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    pub rmp_sf: Option<i16>,
}
#[allow(missing_docs)]
impl ExtSettings {
    pub const NOM_RMP_UP_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, true);
    pub const NOM_RMP_DN_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, true);
    pub const EMG_RMP_UP_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, true);
    pub const EMG_RMP_DN_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, true);
    pub const CONN_RMP_UP_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const CONN_RMP_DN_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const A_GRA: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, true);
    pub const RMP_SF: crate::Point<Self, Option<i16>> = crate::Point::new(7, 1, false);
}
static EXT_SETTINGS_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "nom_rmp_up_rte",
        label: "Ramp Up Rate",
        description: "Ramp up rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nom_rmp_dn_rte",
        label: "NomRmpDnRte",
        description: "Ramp down rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "emg_rmp_up_rte",
        label: "Emergency Ramp Up Rate",
        description: "Emergency ramp up rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "emg_rmp_dn_rte",
        label: "Emergency Ramp Down Rate",
        description: "Emergency ramp down rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "conn_rmp_up_rte",
        label: "Connect Ramp Up Rate",
        description: "Connect ramp up rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "conn_rmp_dn_rte",
        label: "Connect Ramp Down Rate",
        description: "Connect ramp down rate as a percentage of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_gra",
        label: "Default Ramp Rate",
        description: "Ramp rate specified in percent of max current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "rmp_sf",
        label: "Ramp Rate Scale Factor",
        description: "Ramp Rate Scale Factor",
        kind: crate::FieldKind::Point,
    },
];
static EXT_SETTINGS_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "ext_settings",
    label: "Extended Settings",
    description: "Inverter controls extended settings ",
    fields: EXT_SETTINGS_FIELDS,
};
impl crate::GroupMeta for ExtSettings {
    fn group_info() -> &'static crate::GroupInfo {
        &EXT_SETTINGS_GROUP_INFO
    }
}
impl crate::Group for ExtSettings {
    const LEN: u16 = 8;
}
impl ExtSettings {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                nom_rmp_up_rte: Self::NOM_RMP_UP_RTE.from_data(data)?,
                nom_rmp_dn_rte: Self::NOM_RMP_DN_RTE.from_data(data)?,
                emg_rmp_up_rte: Self::EMG_RMP_UP_RTE.from_data(data)?,
                emg_rmp_dn_rte: Self::EMG_RMP_DN_RTE.from_data(data)?,
                conn_rmp_up_rte: Self::CONN_RMP_UP_RTE.from_data(data)?,
                conn_rmp_dn_rte: Self::CONN_RMP_DN_RTE.from_data(data)?,
                a_gra: Self::A_GRA.from_data(data)?,
                rmp_sf: Self::RMP_SF.from_data(data)?,
            },
        ))
    }
}
impl crate::Model for ExtSettings {
    const ID: u16 = 145;
    const NAME: &'static str = "ext_settings";
    const LABEL: &'static str = "Extended Settings";
    const DESCRIPTION: &'static str = "Inverter controls extended settings ";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m145
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
