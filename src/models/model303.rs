//! Back of Module Temperature Model
/// Type alias for [`BomTemp`].
pub type Model303 = BomTemp;
/// Back of Module Temperature Model
///
/// Include to support variable number of  back of module temperature measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct BomTemp {
    #[allow(missing_docs)]
    pub temp: Vec<Temp>,
}
#[allow(missing_docs)]
impl BomTemp {}
static BOM_TEMP_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "temp",
    label: "temp",
    description: "",
    kind: crate::FieldKind::RepeatingGroup(<Temp as crate::GroupMeta>::group_info),
}];
static BOM_TEMP_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "bom_temp",
    label: "Back of Module Temperature Model",
    description: "Include to support variable number of  back of module temperature measurements",
    fields: BOM_TEMP_FIELDS,
};
impl crate::GroupMeta for BomTemp {
    fn group_info() -> &'static crate::GroupInfo {
        &BOM_TEMP_GROUP_INFO
    }
}
impl crate::Group for BomTemp {
    const LEN: u16 = 0;
}
impl BomTemp {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, temp) = Temp::parse_multiple(nested_data)?;
        Ok((nested_data, Self { temp }))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Temp {
    /// Temp
    ///
    /// Back of module temperature measurement
    pub tmp_bom: i16,
}
#[allow(missing_docs)]
impl Temp {
    pub const TMP_BOM: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
}
static TEMP_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "tmp_bom",
    label: "Temp",
    description: "Back of module temperature measurement",
    kind: crate::FieldKind::Point,
}];
static TEMP_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "temp",
    label: "temp",
    description: "",
    fields: TEMP_FIELDS,
};
impl crate::GroupMeta for Temp {
    fn group_info() -> &'static crate::GroupInfo {
        &TEMP_GROUP_INFO
    }
}
impl crate::Group for Temp {
    const LEN: u16 = 1;
}
impl Temp {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                tmp_bom: Self::TMP_BOM.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Temp as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Temp::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for BomTemp {
    const ID: u16 = 303;
    const NAME: &'static str = "bom_temp";
    const LABEL: &'static str = "Back of Module Temperature Model";
    const DESCRIPTION: &'static str =
        "Include to support variable number of  back of module temperature measurements";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m303
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
