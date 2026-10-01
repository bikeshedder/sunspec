//! Eltek Inverter Extension
/// Eltek Inverter Extension
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model64101 {
    #[allow(missing_docs)]
    pub eltek_country_code: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_feeding_phase: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_apd_power_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_method: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_q_ref: Option<u16>,
    #[allow(missing_docs)]
    pub eltek_rps_cos_phi_ref: Option<i16>,
}
#[allow(missing_docs)]
impl Model64101 {
    pub const ELTEK_COUNTRY_CODE: crate::Point<Self, Option<u16>> = crate::Point::new(0, 1);
    pub const ELTEK_FEEDING_PHASE: crate::Point<Self, Option<u16>> = crate::Point::new(1, 1);
    pub const ELTEK_APD_METHOD: crate::Point<Self, Option<u16>> = crate::Point::new(2, 1);
    pub const ELTEK_APD_POWER_REF: crate::Point<Self, Option<u16>> = crate::Point::new(3, 1);
    pub const ELTEK_RPS_METHOD: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1);
    pub const ELTEK_RPS_Q_REF: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1);
    pub const ELTEK_RPS_COS_PHI_REF: crate::Point<Self, Option<i16>> = crate::Point::new(6, 1);
}
impl crate::sealed::Sealed for Model64101 {}
impl crate::Group for Model64101 {
    const LEN: u16 = 7;
}
impl Model64101 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::ParseError> {
        let nested_data = data.get(7..).unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                eltek_country_code: Self::ELTEK_COUNTRY_CODE.from_data(data)?,
                eltek_feeding_phase: Self::ELTEK_FEEDING_PHASE.from_data(data)?,
                eltek_apd_method: Self::ELTEK_APD_METHOD.from_data(data)?,
                eltek_apd_power_ref: Self::ELTEK_APD_POWER_REF.from_data(data)?,
                eltek_rps_method: Self::ELTEK_RPS_METHOD.from_data(data)?,
                eltek_rps_q_ref: Self::ELTEK_RPS_Q_REF.from_data(data)?,
                eltek_rps_cos_phi_ref: Self::ELTEK_RPS_COS_PHI_REF.from_data(data)?,
            },
        ))
    }
}
impl From<Model64101> for crate::AnyModel {
    fn from(model: Model64101) -> Self {
        Self::M64101(model)
    }
}
impl crate::Model for Model64101 {
    const ID: u16 = 64101;
    const NAME: &'static str = "model_64101";
    const LABEL: &'static str = "Eltek Inverter Extension";
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
