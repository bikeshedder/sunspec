//! PV Curves
/// Type alias for [`PvSimCurves`].
pub type Model64413 = PvSimCurves;
struct Counts {
    iv_len: Option<u16>,
}
/// PV Curves
///
/// Current-Voltage and Power-Voltage Profiles for PV Simulation.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PvSimCurves {
    /// IV length
    ///
    /// Number of points in the IV curve.
    pub iv_len: Option<u16>,
    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    pub irr: Option<u16>,
    #[allow(missing_docs)]
    pub irr_sf: Option<i16>,
    /// Comments: IV Curve Points
    pub iv: Vec<Iv>,
}
#[allow(missing_docs)]
impl PvSimCurves {
    pub const IV_LEN: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1, false);
    pub const IRR: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1, false);
    pub const IRR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
}
impl crate::Group for PvSimCurves {
    const LEN: u16 = 3;
}
impl PvSimCurves {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let counts = Counts {
            iv_len: Self::IV_LEN.from_data(data)?,
        };
        let (nested_data, iv) = Iv::parse_multiple(nested_data, &counts)?;
        Ok((
            nested_data,
            Self {
                iv_len: Self::IV_LEN.from_data(data)?,
                irr: Self::IRR.from_data(data)?,
                irr_sf: Self::IRR_SF.from_data(data)?,
                iv,
            },
        ))
    }
}
/// Comments: IV Curve Points
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Iv {
    /// Power
    ///
    /// Power
    pub p: Option<f32>,
    /// Current
    ///
    /// Current
    pub i: Option<f32>,
    /// Voltage
    ///
    /// Voltage
    pub v: Option<f32>,
}
#[allow(missing_docs)]
impl Iv {
    pub const P: crate::Point<Self, Option<f32>> = crate::Point::new(0, 2, false);
    pub const I: crate::Point<Self, Option<f32>> = crate::Point::new(2, 2, false);
    pub const V: crate::Point<Self, Option<f32>> = crate::Point::new(4, 2, false);
}
impl crate::Group for Iv {
    const LEN: u16 = 6;
}
impl Iv {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                p: Self::P.from_data(data)?,
                i: Self::I.from_data(data)?,
                v: Self::V.from_data(data)?,
            },
        ))
    }
    fn parse_multiple<'a>(
        data: &'a [u16],
        counts: &Counts,
    ) -> Result<(&'a [u16], Vec<Self>), crate::DecodeError> {
        let (data, groups) = (0..counts.iv_len.unwrap_or_default()).try_fold(
            (data, Vec::new()),
            |(data, mut groups), _| {
                let (data, group) = Iv::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            },
        )?;
        Ok((data, groups))
    }
}
impl crate::Model for PvSimCurves {
    const ID: u16 = 64413;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64413
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
