//! PV Curves
/// Type alias for [`PvSimCurves`].
pub type Model64413 = PvSimCurves;
struct Counts {
    iv_len: Option<u16>,
}
/// PV Curves
///
/// Current-Voltage and Power-Voltage Profiles for PV Simulation.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PvSimCurves {
    /// IV length
    ///
    /// Number of points in the IV curve.
    pub iv_len: Option<u16>,
    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    pub irr: Option<u16>,
    #[allow(missing_docs)]
    pub irr_sf: Option<i16>,
    /// Comments: IV Curve Points
    pub iv: Vec<Iv>,
}
#[allow(missing_docs)]
impl PvSimCurves {
    pub const IV_LEN: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const IRR: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const IRR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
}
static PV_SIM_CURVES_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "iv_len",
        label: "IV length",
        description: "Number of points in the IV curve.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "irr",
        label: "POA Irradiance",
        description: "Plane of Array Irradiance",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "irr_sf",
        label: "Irr_SF",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "iv",
        label: "IV",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Iv as crate::GroupMeta>::group_info),
    },
];
static PV_SIM_CURVES_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "PVSimCurves",
    label: "PV Curves",
    description: "Current-Voltage and Power-Voltage Profiles for PV Simulation.",
    fields: PV_SIM_CURVES_FIELDS,
};
impl crate::GroupMeta for PvSimCurves {
    fn group_info() -> &'static crate::GroupInfo {
        &PV_SIM_CURVES_GROUP_INFO
    }
}
impl crate::Group for PvSimCurves {
    const LEN: u16 = 3;
}
impl PvSimCurves {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            iv_len: Self::IV_LEN.from_data(data)?,
        };
        let (nested_data, iv) = Iv::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                iv_len: Self::IV_LEN.from_data(data)?,
                irr: Self::IRR.from_data(data)?,
                irr_sf: Self::IRR_SF.from_data(data)?,
                iv,
            },
        ))
    }
}
/// Comments: IV Curve Points
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Iv {
    /// Power
    ///
    /// Power
    pub p: Option<f32>,
    /// Current
    ///
    /// Current
    pub i: Option<f32>,
    /// Voltage
    ///
    /// Voltage
    pub v: Option<f32>,
}
#[allow(missing_docs)]
impl Iv {
    pub const P: crate::Point<Self, Option<f32>> = crate::Point::new(0, 2, false);
    pub const I: crate::Point<Self, Option<f32>> = crate::Point::new(2, 2, false);
    pub const V: crate::Point<Self, Option<f32>> = crate::Point::new(4, 2, false);
}
static IV_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "p",
        label: "Power",
        description: "Power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "i",
        label: "Current",
        description: "Current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "Voltage",
        description: "Voltage",
        kind: crate::FieldKind::Point,
    },
];
static IV_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "IV",
    label: "IV",
    description: "",
    fields: IV_FIELDS,
};
impl crate::GroupMeta for Iv {
    fn group_info() -> &'static crate::GroupInfo {
        &IV_GROUP_INFO
    }
}
impl crate::Group for Iv {
    const LEN: u16 = 6;
}
impl Iv {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                v: Self::V.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) = (0..counts.iv_len.unwrap_or_default()).try_fold(
            (data, Vec::new()),
            |(data, mut groups), _| {
                let (data, group) = Iv::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            },
        )?;
        Ok((data, groups))
    }
}
impl crate::Model for PvSimCurves {
    const ID: u16 = 64413;
    const NAME: &'static str = "PVSimCurves";
    const LABEL: &'static str = "PV Curves";
    const DESCRIPTION: &'static str =
        "Current-Voltage and Power-Voltage Profiles for PV Simulation.";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64413
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
