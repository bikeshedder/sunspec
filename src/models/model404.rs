/// String Combiner (Advanced)
///
/// An advanced string combiner including voltage and energy measurements
///
/// Notes: This model supersedes model 402
#[derive(Debug)]
pub struct Model404 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
    /// Power scale factor
    pub dcw_sf: Option<i16>,
    /// Energy scale factor
    pub dcwh_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dcamax: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Bitmask value.  Events
    pub evt: u32,
    /// Vendor Event
    ///
    /// Bitmask value.  Vendor defined events
    pub evtvnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    pub dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dcahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dcv: Option<i16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dcpr: Option<i16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dcwh: Option<u32>,
    /// Current scale factor for inputs
    pub indca_sf: Option<i16>,
    /// Amp-hour scale factor for inputs
    pub indcahr_sf: Option<i16>,
    /// Voltage scale factor for inputs
    pub indcv_sf: Option<i16>,
    /// Power scale factor for inputs
    pub indcw_sf: Option<i16>,
    /// Energy scale factor for inputs
    pub indcwh_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model404 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(2, 1, false);
    pub const DCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(3, 1, false);
    pub const DCWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(4, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const EVTVND: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(9, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const DCAHR: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
    pub const DCV: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(14, 1, false);
    pub const TMP: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, false);
    pub const DCW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(16, 1, false);
    pub const DCPR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(17, 1, false);
    pub const DCWH: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(18, 2, false);
    pub const INDCA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(20, 1, false);
    pub const INDCAHR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const INDCV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const INDCW_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
    pub const INDCWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(24, 1, false);
}

impl crate::Model for Model404 {
    const ID: u16 = 404;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            dcwh_sf: Self::DCWH_SF.from_data(data)?,
            dcamax: Self::DCAMAX.from_data(data)?,
            n: Self::N.from_data(data)?,
            evt: Self::EVT.from_data(data)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcpr: Self::DCPR.from_data(data)?,
            dcwh: Self::DCWH.from_data(data)?,
            indca_sf: Self::INDCA_SF.from_data(data)?,
            indcahr_sf: Self::INDCAHR_SF.from_data(data)?,
            indcv_sf: Self::INDCV_SF.from_data(data)?,
            indcw_sf: Self::INDCW_SF.from_data(data)?,
            indcwh_sf: Self::INDCWH_SF.from_data(data)?,
        })
    }
}
