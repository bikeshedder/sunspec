/// Extended Settings
///
/// Inverter controls extended settings
#[derive(Debug)]
pub struct Model145 {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    pub nomrmpuprte: Option<u16>,
    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    pub nomrmpdnrte: Option<u16>,
    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    pub emgrmpuprte: Option<u16>,
    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    pub emgrmpdnrte: Option<u16>,
    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    pub connrmpuprte: Option<u16>,
    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    pub connrmpdnrte: Option<u16>,
    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    pub agra: Option<u16>,
    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    pub rmp_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model145 {
    pub const NOMRMPUPRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const NOMRMPDNRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const EMGRMPUPRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const EMGRMPDNRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const CONNRMPUPRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const CONNRMPDNRTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const AGRA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const RMP_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model145 {
    const ID: u16 = 145;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nomrmpuprte: Self::NOMRMPUPRTE.from_data(data)?,
            nomrmpdnrte: Self::NOMRMPDNRTE.from_data(data)?,
            emgrmpuprte: Self::EMGRMPUPRTE.from_data(data)?,
            emgrmpdnrte: Self::EMGRMPDNRTE.from_data(data)?,
            connrmpuprte: Self::CONNRMPUPRTE.from_data(data)?,
            connrmpdnrte: Self::CONNRMPDNRTE.from_data(data)?,
            agra: Self::AGRA.from_data(data)?,
            rmp_sf: Self::RMP_SF.from_data(data)?,
        })
    }
}
