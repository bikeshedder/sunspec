//! Flow Battery Module Model
/// Type alias for [`FlowBatteryModule`].
pub type Model808 = FlowBatteryModule;
/// Flow Battery Module Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FlowBatteryModule {
    /// Module Points To Be Determined
    pub module_tbd: u16,
    #[allow(missing_docs)]
    pub stack: Vec<Stack>,
}
#[allow(missing_docs)]
impl FlowBatteryModule {
    pub const MODULE_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
static FLOW_BATTERY_MODULE_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "module_tbd",
        label: "Module Points To Be Determined",
        description: "",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "stack",
        label: "stack",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Stack as crate::GroupMeta>::group_info),
    },
];
static FLOW_BATTERY_MODULE_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "flow_battery_module",
    label: "Flow Battery Module Model",
    description: "",
    fields: FLOW_BATTERY_MODULE_FIELDS,
};
impl crate::GroupMeta for FlowBatteryModule {
    fn group_info() -> &'static crate::GroupInfo {
        &FLOW_BATTERY_MODULE_GROUP_INFO
    }
}
impl crate::Group for FlowBatteryModule {
    const LEN: u16 = 1;
}
impl FlowBatteryModule {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, stack) = Stack::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                module_tbd: Self::MODULE_TBD.from_data(data)?,
                stack,
            },
        ))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Stack {
    /// Stack Points To Be Determined
    pub stack_tbd: u16,
}
#[allow(missing_docs)]
impl Stack {
    pub const STACK_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
static STACK_FIELDS: &[crate::FieldInfo] = &[crate::FieldInfo {
    name: "stack_tbd",
    label: "Stack Points To Be Determined",
    description: "",
    kind: crate::FieldKind::Point,
}];
static STACK_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "stack",
    label: "stack",
    description: "",
    fields: STACK_FIELDS,
};
impl crate::GroupMeta for Stack {
    fn group_info() -> &'static crate::GroupInfo {
        &STACK_GROUP_INFO
    }
}
impl crate::Group for Stack {
    const LEN: u16 = 1;
}
impl Stack {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                stack_tbd: Self::STACK_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Stack as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Stack::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for FlowBatteryModule {
    const ID: u16 = 808;
    const NAME: &'static str = "flow_battery_module";
    const LABEL: &'static str = "Flow Battery Module Model";
    const DESCRIPTION: &'static str = "";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m808
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
