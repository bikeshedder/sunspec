//! Back of Module Temperature Model
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
impl crate::Group for BomTemp {
    const LEN: u16 = 0;
}
impl BomTemp {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self { temp: Vec::new() },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.temp) = Temp::parse_multiple(data, &group)?;
        Ok((data, group))
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
impl crate::Group for Temp {
    const LEN: u16 = 1;
}
impl Temp {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                tmp_bom: Self::TMP_BOM.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &BomTemp,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &BomTemp,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..0 {
            let group;
            (data, group) = Temp::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for BomTemp {
    const ID: u16 = 303;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m303
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
