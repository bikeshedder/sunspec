//! Flow Battery Stack Model
/// Type alias for [`FlowBatteryStack`].
pub type Model809 = FlowBatteryStack;
/// Flow Battery Stack Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FlowBatteryStack {
    /// Stack Points To Be Determined
    pub stack_tbd: u16,
    #[allow(missing_docs)]
    pub cell: Vec<Cell>,
}
#[allow(missing_docs)]
impl FlowBatteryStack {
    pub const STACK_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::Group for FlowBatteryStack {
    const LEN: u16 = 1;
}
impl FlowBatteryStack {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        let (nested_data, cell) = Cell::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                stack_tbd: Self::STACK_TBD.from_data(data)?,
                cell,
            },
        ))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Cell {
    /// Cell Points To Be Determined
    pub cell_tbd: u16,
}
#[allow(missing_docs)]
impl Cell {
    pub const CELL_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::Group for Cell {
    const LEN: u16 = 1;
}
impl Cell {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                cell_tbd: Self::CELL_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Cell as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Cell::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for FlowBatteryStack {
    const ID: u16 = 809;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m809
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
