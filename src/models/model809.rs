//! Flow Battery Stack Model
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
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                stack_tbd: Self::STACK_TBD.from_data(data)?,
                cell: Vec::new(),
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.cell) = Cell::parse_multiple(data, &group)?;
        Ok((data, group))
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
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                cell_tbd: Self::CELL_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &FlowBatteryStack,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &FlowBatteryStack,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..0 {
            let group;
            (data, group) = Cell::parse_group(data, model)?;
            groups.push(group);
        }
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
