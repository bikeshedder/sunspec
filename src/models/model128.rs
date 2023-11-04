/// Dynamic Reactive Current
///
/// Dynamic Reactive Current
///
/// Notes: Ref 3: 8.10.1.2; Ref 4: 12
#[derive(Debug)]
pub struct Model128 {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    pub argramod: u16,
    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    pub argrasag: u16,
    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current.  A value of 0 indicates no additional reactive current support.
    pub argraswell: u16,
    /// ModEna
    ///
    /// Activate dynamic reactive current model
    pub modena: u16,
    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    pub filtms: Option<u16>,
    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    pub dbvmin: Option<u16>,
    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    pub dbvmax: Option<u16>,
    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    pub blkznv: Option<u16>,
    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    pub hysblkznv: Option<u16>,
    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    pub blkzntmms: Option<u16>,
    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    pub holdtmms: Option<u16>,
    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    pub argra_sf: i16,
    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    pub vrefpct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model128 {
    pub const ARGRAMOD: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const ARGRASAG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const ARGRASWELL: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const MODENA: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const FILTMS: crate::PointDef<Self, u16> = crate::PointDef::new(4, 1, true);
    pub const DBVMIN: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, true);
    pub const DBVMAX: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, true);
    pub const BLKZNV: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const HYSBLKZNV: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, true);
    pub const BLKZNTMMS: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, true);
    pub const HOLDTMMS: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, true);
    pub const ARGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const VREFPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
}

impl crate::Model for Model128 {
    const ID: u16 = 128;
    const LENGTH: u16 = 14;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            argramod: Self::ARGRAMOD
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            argrasag: Self::ARGRASAG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            argraswell: Self::ARGRASWELL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            modena: Self::MODENA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            filtms: Self::FILTMS.from_data(data)?,
            dbvmin: Self::DBVMIN.from_data(data)?,
            dbvmax: Self::DBVMAX.from_data(data)?,
            blkznv: Self::BLKZNV.from_data(data)?,
            hysblkznv: Self::HYSBLKZNV.from_data(data)?,
            blkzntmms: Self::BLKZNTMMS.from_data(data)?,
            holdtmms: Self::HOLDTMMS.from_data(data)?,
            argra_sf: Self::ARGRA_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            vrefpct_sf: Self::VREFPCT_SF.from_data(data)?,
        })
    }
}
