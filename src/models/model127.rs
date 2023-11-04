/// Freq-Watt Param
///
/// Parameterized Frequency-Watt
///
/// Notes: Ref 3: 8.9.1.2, 8.9.4.2
#[derive(Debug)]
pub struct Model127 {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    pub wgra: u16,
    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    pub hzstr: i16,
    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    pub hzstop: i16,
    /// HysEna
    ///
    /// Enable hysteresis
    pub hysena: u16,
    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    pub modena: u16,
    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    pub hzstopwgra: Option<u16>,
    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    pub wgra_sf: Option<i16>,
    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    pub hzstrstop_sf: Option<i16>,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    pub rmpincdec_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model127 {
    pub const WGRA: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const HZSTR: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, true);
    pub const HZSTOP: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, true);
    pub const HYSENA: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const HZSTOPWGRA: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const WGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const HZSTRSTOP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const RMPINCDEC_SF: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
}

impl crate::Model for Model127 {
    const ID: u16 = 127;
    const LENGTH: u16 = 10;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wgra: Self::WGRA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstr: Self::HZSTR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstop: Self::HZSTOP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hysena: Self::HYSENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hzstopwgra: Self::HZSTOPWGRA.from_data(data)?,
            wgra_sf: Self::WGRA_SF.from_data(data)?,
            hzstrstop_sf: Self::HZSTRSTOP_SF.from_data(data)?,
            rmpincdec_sf: Self::RMPINCDEC_SF.from_data(data)?,
        })
    }
}
