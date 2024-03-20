//! Extended Settings

/// Extended Settings
///
/// Inverter controls extended settings
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model145 {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    pub nom_rmp_up_rte: Option<u16>,
    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    pub nom_rmp_dn_rte: Option<u16>,
    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    pub emg_rmp_up_rte: Option<u16>,
    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    pub emg_rmp_dn_rte: Option<u16>,
    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    pub conn_rmp_up_rte: Option<u16>,
    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    pub conn_rmp_dn_rte: Option<u16>,
    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    pub a_gra: Option<u16>,
    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    pub rmp_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model145 {
    pub const NOM_RMP_UP_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const NOM_RMP_DN_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const EMG_RMP_UP_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, true);
    pub const EMG_RMP_DN_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, true);
    pub const CONN_RMP_UP_RTE: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(4, 1, true);
    pub const CONN_RMP_DN_RTE: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(5, 1, true);
    pub const A_GRA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const RMP_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(7, 1, false);
}

impl crate::Model for Model145 {
    const ID: u16 = 145;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nom_rmp_up_rte: Self::NOM_RMP_UP_RTE.from_data(data)?,
            nom_rmp_dn_rte: Self::NOM_RMP_DN_RTE.from_data(data)?,
            emg_rmp_up_rte: Self::EMG_RMP_UP_RTE.from_data(data)?,
            emg_rmp_dn_rte: Self::EMG_RMP_DN_RTE.from_data(data)?,
            conn_rmp_up_rte: Self::CONN_RMP_UP_RTE.from_data(data)?,
            conn_rmp_dn_rte: Self::CONN_RMP_DN_RTE.from_data(data)?,
            a_gra: Self::A_GRA.from_data(data)?,
            rmp_sf: Self::RMP_SF.from_data(data)?,
        })
    }
}
