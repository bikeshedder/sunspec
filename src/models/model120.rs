/// Nameplate
///
/// Inverter Controls Nameplate Ratings
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model120 {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    pub dertyp: u16,
    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    pub wrtg: u16,
    /// WRtg_SF
    ///
    /// Scale factor
    pub wrtg_sf: i16,
    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    pub vartg: u16,
    /// VARtg_SF
    ///
    /// Scale factor
    pub vartg_sf: i16,
    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    pub varrtgq1: i16,
    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    pub varrtgq2: i16,
    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    pub varrtgq3: i16,
    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    pub varrtgq4: i16,
    /// VArRtg_SF
    ///
    /// Scale factor
    pub varrtg_sf: i16,
    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Notes: Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    pub artg: u16,
    /// ARtg_SF
    ///
    /// Scale factor
    pub artg_sf: i16,
    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq1: i16,
    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq2: i16,
    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq3: i16,
    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// Notes: EEI sign convention.
    pub pfrtgq4: i16,
    /// PFRtg_SF
    ///
    /// Scale factor
    pub pfrtg_sf: i16,
    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    pub whrtg: Option<u16>,
    /// WHRtg_SF
    ///
    /// Scale factor
    pub whrtg_sf: Option<i16>,
    /// AhrRtg
    ///
    /// The usable capacity of the battery.  Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    pub ahrrtg: Option<u16>,
    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    pub ahrrtg_sf: Option<i16>,
    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    pub maxcharte: Option<u16>,
    /// MaxChaRte_SF
    ///
    /// Scale factor
    pub maxcharte_sf: Option<i16>,
    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    pub maxdischarte: Option<u16>,
    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    pub maxdischarte_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model120 {
    pub const DERTYP: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WRTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const WRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const VARTG: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const VARTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const VARRTGQ1: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const VARRTGQ2: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const VARRTGQ3: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const VARRTGQ4: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const VARRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const ARTG: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const ARTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PFRTGQ1: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const PFRTGQ2: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const PFRTGQ3: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const PFRTGQ4: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const PFRTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WHRTG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, false);
    pub const WHRTG_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const AHRRTG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(19, 1, false);
    pub const AHRRTG_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(20, 1, false);
    pub const MAXCHARTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(21, 1, false);
    pub const MAXCHARTE_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const MAXDISCHARTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(23, 1, false);
    pub const MAXDISCHARTE_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(24, 1, false);
}

impl crate::Model for Model120 {
    const ID: u16 = 120;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            dertyp: Self::DERTYP.from_data(data)?,
            wrtg: Self::WRTG.from_data(data)?,
            wrtg_sf: Self::WRTG_SF.from_data(data)?,
            vartg: Self::VARTG.from_data(data)?,
            vartg_sf: Self::VARTG_SF.from_data(data)?,
            varrtgq1: Self::VARRTGQ1.from_data(data)?,
            varrtgq2: Self::VARRTGQ2.from_data(data)?,
            varrtgq3: Self::VARRTGQ3.from_data(data)?,
            varrtgq4: Self::VARRTGQ4.from_data(data)?,
            varrtg_sf: Self::VARRTG_SF.from_data(data)?,
            artg: Self::ARTG.from_data(data)?,
            artg_sf: Self::ARTG_SF.from_data(data)?,
            pfrtgq1: Self::PFRTGQ1.from_data(data)?,
            pfrtgq2: Self::PFRTGQ2.from_data(data)?,
            pfrtgq3: Self::PFRTGQ3.from_data(data)?,
            pfrtgq4: Self::PFRTGQ4.from_data(data)?,
            pfrtg_sf: Self::PFRTG_SF.from_data(data)?,
            whrtg: Self::WHRTG.from_data(data)?,
            whrtg_sf: Self::WHRTG_SF.from_data(data)?,
            ahrrtg: Self::AHRRTG.from_data(data)?,
            ahrrtg_sf: Self::AHRRTG_SF.from_data(data)?,
            maxcharte: Self::MAXCHARTE.from_data(data)?,
            maxcharte_sf: Self::MAXCHARTE_SF.from_data(data)?,
            maxdischarte: Self::MAXDISCHARTE.from_data(data)?,
            maxdischarte_sf: Self::MAXDISCHARTE_SF.from_data(data)?,
        })
    }
}
