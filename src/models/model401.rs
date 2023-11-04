/// String Combiner (Current)
///
/// A basic string combiner
///
/// Notes: This model is SUPERSEDED by model 403
#[derive(Debug)]
pub struct Model401 {
    /// Current scale factor
    pub dca_sf: i16,
    /// Amp-hour scale factor
    pub dcahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dcv_sf: Option<i16>,
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
    pub dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
}

#[allow(missing_docs)]

impl Model401 {
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const DCAHR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const DCAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(5, 2, false);
    pub const EVTVND: crate::PointDef<Self, u32> = crate::PointDef::new(7, 2, false);
    pub const DCA: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const DCAHR: crate::PointDef<Self, u32> = crate::PointDef::new(10, 2, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const TMP: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
}

impl crate::Model for Model401 {
    const ID: u16 = 401;
    const LENGTH: u16 = 22;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dca_sf: Self::DCA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr_sf: Self::DCAHR_SF.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcamax: Self::DCAMAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd: Self::EVTVND.from_data(data)?,
            dca: Self::DCA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dcahr: Self::DCAHR.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            tmp: Self::TMP.from_data(data)?,
        })
    }
}
