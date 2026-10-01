//! Flow Battery Module Model
/// Type alias for [`FlowBatteryModule`].
pub type Model808 = FlowBatteryModule;
/// Flow Battery Module Model
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FlowBatteryModule {
    /// Module Points To Be Determined
    pub module_tbd: u16,
    #[allow(missing_docs)]
    pub stack: Vec<Stack>,
}
#[allow(missing_docs)]
impl FlowBatteryModule {
    pub const MODULE_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1);
}
impl crate::sealed::Sealed for FlowBatteryModule {}
impl crate::Group for FlowBatteryModule {
    const LEN: u16 = 1;
}
impl FlowBatteryModule {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(1..).unwrap_or(&[]);
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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Stack {
    /// Stack Points To Be Determined
    pub stack_tbd: u16,
}
#[allow(missing_docs)]
impl Stack {
    pub const STACK_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1);
}
impl crate::sealed::Sealed for Stack {}
impl crate::Group for Stack {
    const LEN: u16 = 1;
}
impl Stack {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(1..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                stack_tbd: Self::STACK_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::ParseError> {
        let group_len = usize::from(<Stack as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::ParseError::InvalidGroupLength);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Stack::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::ParseError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl From<FlowBatteryModule> for crate::AnyModel {
    fn from(model: FlowBatteryModule) -> Self {
        Self::M808(model)
    }
}
impl crate::Model for FlowBatteryModule {
    const ID: u16 = 808;
    const NAME: &'static str = "flow_battery_module";
    const LABEL: &'static str = "Flow Battery Module Model";
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
