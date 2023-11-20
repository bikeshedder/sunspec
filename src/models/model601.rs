/// Tracker Controller DRAFT 2
///
/// Monitors and controls multiple trackers
///
/// Notes: Trackers may include GPS model 305 for location information
#[derive(Debug)]
pub struct Model601 {
    /// Controller
    ///
    /// Descriptive name for this control unit
    pub nam: Option<String>,
    /// Type
    ///
    /// Type of tracker
    pub typ: u16,
    /// Date
    ///
    /// Local date in YYYYMMDD format
    pub dtloc: Option<String>,
    /// Time
    ///
    /// 24 hour local time stamp to second
    pub tmloc: Option<String>,
    /// Day
    ///
    /// Number of the day in the year (1-366)
    pub day: Option<u16>,
    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub glblelctl: Option<i32>,
    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type
    pub glblazctl: Option<i32>,
    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// Notes: The global controls all trackers
    pub glblctl: Option<u16>,
    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Notes: Combined tracker alarm conditions.  See individual trackers for alarms
    pub glblalm: Option<u16>,
    /// SF
    ///
    /// Scale Factor for targets and position measurements in degrees
    pub dgr_sf: i16,
    /// Trackers
    ///
    /// Number of trackers being controlled.  Size of repeating block.
    pub n: u16,
}

#[allow(missing_docs)]

impl Model601 {
    pub const NAM: crate::PointDef<Self, String> = crate::PointDef::new(0, 8, false);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const DTLOC: crate::PointDef<Self, String> = crate::PointDef::new(9, 5, false);
    pub const TMLOC: crate::PointDef<Self, String> = crate::PointDef::new(14, 3, false);
    pub const DAY: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const GLBLELCTL: crate::PointDef<Self, i32> = crate::PointDef::new(18, 2, true);
    pub const GLBLAZCTL: crate::PointDef<Self, i32> = crate::PointDef::new(20, 2, true);
    pub const GLBLCTL: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, true);
    pub const GLBLALM: crate::PointDef<Self, u16> = crate::PointDef::new(23, 1, false);
    pub const DGR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
}

impl crate::Model for Model601 {
    const ID: u16 = 601;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            typ: Self::TYP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dtloc: Self::DTLOC.from_data(data)?,
            tmloc: Self::TMLOC.from_data(data)?,
            day: Self::DAY.from_data(data)?,
            glblelctl: Self::GLBLELCTL.from_data(data)?,
            glblazctl: Self::GLBLAZCTL.from_data(data)?,
            glblctl: Self::GLBLCTL.from_data(data)?,
            glblalm: Self::GLBLALM.from_data(data)?,
            dgr_sf: Self::DGR_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
