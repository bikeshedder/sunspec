//! Irradiance Model
/// Type alias for [`Irradiance`].
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
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, repeating) = Repeating::parse_multiple(nested_data)?;
        Ok((nested_data, Self { repeating }))
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
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ghi: Self::GHI.from_data(data)?,
                poai: Self::POAI.from_data(data)?,
                dfi: Self::DFI.from_data(data)?,
                dni: Self::DNI.from_data(data)?,
                oti: Self::OTI.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Repeating as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Repeating::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
impl crate::Model for Irradiance {
    const ID: u16 = 302;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m302
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
