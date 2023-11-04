/// Freq-Watt Crv
///
/// Curve-Based Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
pub struct Model134 {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    pub actcrv: u16,
    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    pub modena: u16,
    /// WinTms
    ///
    /// Time window for freq-watt change.
    pub wintms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    pub rvrttms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    pub rmptms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    pub ncrv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 10).
    pub npt: u16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    pub hz_sf: i16,
    /// W_SF
    ///
    /// Scale factor for percent WRef.
    pub w_sf: i16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model134 {
    pub const ACTCRV: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WINTMS: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const RVRTTMS: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const RMPTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const NCRV: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const NPT: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
}

impl crate::Model for Model134 {
    const ID: u16 = 134;
    const LENGTH: u16 = 68;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actcrv: Self::ACTCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wintms: Self::WINTMS.from_data(data)?,
            rvrttms: Self::RVRTTMS.from_data(data)?,
            rmptms: Self::RMPTMS.from_data(data)?,
            ncrv: Self::NCRV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            npt: Self::NPT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}
