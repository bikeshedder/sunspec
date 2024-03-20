//! Tracker Controller DRAFT 2

/// Tracker Controller DRAFT 2
///
/// Monitors and controls multiple trackers
///
/// Notes: Trackers may include GPS model 305 for location information
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model601 {
    /// Controller
    ///
    /// Descriptive name for this control unit
    pub nam: Option<String>,
    /// Type
    ///
    /// Type of tracker
    pub typ: Typ,
    /// Date
    ///
    /// Local date in YYYYMMDD format
    pub dt_loc: Option<String>,
    /// Time
    ///
    /// 24 hour local time stamp to second
    pub tm_loc: Option<String>,
    /// Day
    ///
    /// Number of the day in the year (1-366)
    pub day: Option<u16>,
    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub glbl_el_ctl: Option<i32>,
    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type
    pub glbl_az_ctl: Option<i32>,
    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// Notes: The global controls all trackers
    pub glbl_ctl: Option<GlblCtl>,
    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Notes: Combined tracker alarm conditions.  See individual trackers for alarms
    pub glbl_alm: Option<GlblAlm>,
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
    pub const NAM: crate::PointDef<Self, Option<String>> = crate::PointDef::new(0, 8, false);
    pub const TYP: crate::PointDef<Self, Typ> = crate::PointDef::new(8, 1, false);
    pub const DT_LOC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(9, 5, false);
    pub const TM_LOC: crate::PointDef<Self, Option<String>> = crate::PointDef::new(14, 3, false);
    pub const DAY: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, false);
    pub const GLBL_EL_CTL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(18, 2, true);
    pub const GLBL_AZ_CTL: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(20, 2, true);
    pub const GLBL_CTL: crate::PointDef<Self, Option<GlblCtl>> = crate::PointDef::new(22, 1, true);
    pub const GLBL_ALM: crate::PointDef<Self, Option<GlblAlm>> = crate::PointDef::new(23, 1, false);
    pub const DGR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
}

impl crate::Model for Model601 {
    const ID: u16 = 601;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            nam: Self::NAM.from_data(data)?,
            typ: Self::TYP.from_data(data)?,
            dt_loc: Self::DT_LOC.from_data(data)?,
            tm_loc: Self::TM_LOC.from_data(data)?,
            day: Self::DAY.from_data(data)?,
            glbl_el_ctl: Self::GLBL_EL_CTL.from_data(data)?,
            glbl_az_ctl: Self::GLBL_AZ_CTL.from_data(data)?,
            glbl_ctl: Self::GLBL_CTL.from_data(data)?,
            glbl_alm: Self::GLBL_ALM.from_data(data)?,
            dgr_sf: Self::DGR_SF.from_data(data)?,
            n: Self::N.from_data(data)?,
        })
    }
}

#[doc = "Type\n\nType of tracker"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Typ {
    #[doc = ""]
    Unknown = 0,
    #[doc = ""]
    Fixed = 1,
    #[doc = ""]
    Horizontal = 2,
    #[doc = ""]
    Tilted = 3,
    #[doc = ""]
    Azimuth = 4,
    #[doc = ""]
    Dual = 5,
    #[doc = ""]
    Other = 99,
}
impl crate::Value for Typ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<Typ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                Typ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

#[doc = "Global Mode\n\nGlobal Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.\n\nNotes: The global controls all trackers"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum :: FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum GlblCtl {
    #[doc = ""]
    Automatic = 0,
    #[doc = ""]
    Manual = 1,
    #[doc = ""]
    Calibrate = 2,
}
impl crate::Value for GlblCtl {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<GlblCtl> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                GlblCtl::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}

bitflags::bitflags! { # [doc = "Global Alarm\n\nGlobal tracker alarm conditions\n\nNotes: Combined tracker alarm conditions.  See individual trackers for alarms"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] pub struct GlblAlm : u16 { # [doc = ""] const SetPoint = 1 ; # [doc = ""] const ObsEl = 2 ; # [doc = ""] const ObsAz = 4 ; } }
impl crate::Value for GlblAlm {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<GlblAlm> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(GlblAlm::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
