//! Flow Battery Model
/// Type alias for [`FlowBattery`].
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
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, battery_string) = BatteryString::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                bat_tbd: Self::BAT_TBD.from_data(data)?,
                battery_string,
            },
        ))
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
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                bat_st_tbd: Self::BAT_ST_TBD.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<BatteryString as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = BatteryString::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for FlowBattery {
    const ID: u16 = 806;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m806
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
