//! Irradiance Model
pub type Model302 = Irradiance;
/// Irradiance Model
///
/// Include to support various irradiance measurements
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Irradiance {
    #[allow(missing_docs)]
    pub repeating: Vec<Repeating>,
}
#[allow(missing_docs)]
impl Irradiance {}
impl crate::Group for Irradiance {
    const LEN: u16 = 0;
}
impl Irradiance {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                repeating: Vec::new(),
            },
        ))
    }
    fn parse_group(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        (data, group.repeating) = Repeating::parse_multiple(data, &group)?;
        Ok((data, group))
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Repeating {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    pub ghi: Option<u16>,
    /// POAI
    ///
    /// Plane-of-Array Irradiance
    pub poai: Option<u16>,
    /// DFI
    ///
    /// Diffuse Irradiance
    pub dfi: Option<u16>,
    /// DNI
    ///
    /// Direct Normal Irradiance
    pub dni: Option<u16>,
    /// OTI
    ///
    /// Other Irradiance
    pub oti: Option<u16>,
}
#[allow(missing_docs)]
impl Repeating {
    pub const GHI: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const POAI: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const DFI: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1, false);
    pub const DNI: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1, false);
    pub const OTI: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
}
impl crate::Group for Repeating {
    const LEN: u16 = 5;
}
impl Repeating {
    fn parse_points(mut data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        Ok((
            &data[usize::from(<Self as crate::Group>::LEN)..],
            Self {
                ghi: Self::GHI.from_data(data)?,
                poai: Self::POAI.from_data(data)?,
                dfi: Self::DFI.from_data(data)?,
                dni: Self::DNI.from_data(data)?,
                oti: Self::OTI.from_data(data)?,
            },
        ))
    }
    fn parse_group<'a>(
        mut data: &'a [u16],
        model: &Irradiance,
    ) -> Result<(&'a [u16], Self), crate::DecodeError> {
        let mut group;
        (data, group) = Self::parse_points(data)?;
        Ok((data, group))
    }
    fn parse_multiple<'a>(
        mut data: &'a [u16],
        model: &Irradiance,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let mut groups = Vec::new();
        for _ in 0..0 {
            let group;
            (data, group) = Repeating::parse_group(data, model)?;
            groups.push(group);
        }
        Ok((data, groups))
    }
}
impl crate::Model for Irradiance {
    const ID: u16 = 302;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m302
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
