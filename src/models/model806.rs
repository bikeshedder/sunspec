//! Flow Battery Model
pub type Model806 = FlowBattery;
/// Flow Battery Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FlowBattery {
    /// Battery Points To Be Determined
    pub bat_tbd: u16,
    #[allow(missing_docs)]
    pub battery_string: Vec<BatteryString>,
}
#[allow(missing_docs)]
impl FlowBattery {
    pub const BAT_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::Group for FlowBattery {
    const LEN: u16 = 1;
}
impl FlowBattery {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                bat_tbd: Self::BAT_TBD.from_data(data)?,
                battery_string: Vec::new(),
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.battery_string) = BatteryString::parse_multiple(data, &group)?;
        Ok((data, group))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct BatteryString {
    /// Battery String Points To Be Determined
    pub bat_st_tbd: u16,
}
#[allow(missing_docs)]
impl BatteryString {
    pub const BAT_ST_TBD: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
}
impl crate::Group for BatteryString {
    const LEN: u16 = 1;
}
impl BatteryString {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                bat_st_tbd: Self::BAT_ST_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &FlowBattery,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &FlowBattery,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..0 {
            let group;
            (data, group) = BatteryString::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for FlowBattery {
    const ID: u16 = 806;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m806
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
