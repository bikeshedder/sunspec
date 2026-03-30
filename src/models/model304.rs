//! Inclinometer Model
pub type Model304 = Inclinometer;
/// Inclinometer Model
///
/// Include to support orientation measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Inclinometer {
    #[allow(missing_docs)]
    pub incl: Vec<Incl>,
}
#[allow(missing_docs)]
impl Inclinometer {}
impl crate::Group for Inclinometer {
    const LEN: u16 = 0;
}
impl Inclinometer {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self { incl: Vec::new() },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.incl) = Incl::parse_multiple(data, &group)?;
        Ok((data, group))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Incl {
    /// X
    ///
    /// X-Axis inclination
    pub inclx: i32,
    /// Y
    ///
    /// Y-Axis inclination
    pub incly: Option<i32>,
    /// Z
    ///
    /// Z-Axis inclination
    pub inclz: Option<i32>,
}
#[allow(missing_docs)]
impl Incl {
    pub const INCLX: crate::Point<Self, i32> = crate::Point::new(0, 2, false);
    pub const INCLY: crate::Point<Self, Option<i32>> = crate::Point::new(2, 2, false);
    pub const INCLZ: crate::Point<Self, Option<i32>> = crate::Point::new(4, 2, false);
}
impl crate::Group for Incl {
    const LEN: u16 = 6;
}
impl Incl {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                inclx: Self::INCLX.from_data(data)?,
                incly: Self::INCLY.from_data(data)?,
                inclz: Self::INCLZ.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &Inclinometer,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &Inclinometer,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..0 {
            let group;
            (data, group) = Incl::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for Inclinometer {
    const ID: u16 = 304;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m304
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
