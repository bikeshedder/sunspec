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
    pub const NAM: crate::Point<Self, Option<String>> = crate::Point::new(0, 8, false);
    pub const TYP: crate::Point<Self, Typ> = crate::Point::new(8, 1, false);
    pub const DT_LOC: crate::Point<Self, Option<String>> = crate::Point::new(9, 5, false);
    pub const TM_LOC: crate::Point<Self, Option<String>> = crate::Point::new(14, 3, false);
    pub const DAY: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, false);
    pub const GLBL_EL_CTL: crate::Point<Self, Option<i32>> = crate::Point::new(18, 2, true);
    pub const GLBL_AZ_CTL: crate::Point<Self, Option<i32>> = crate::Point::new(20, 2, true);
    pub const GLBL_CTL: crate::Point<Self, Option<GlblCtl>> = crate::Point::new(22, 1, true);
    pub const GLBL_ALM: crate::Point<Self, Option<GlblAlm>> = crate::Point::new(23, 1, false);
    pub const DGR_SF: crate::Point<Self, i16> = crate::Point::new(24, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(25, 1, false);
}
impl crate::Model for Model601 {
    const ID: u16 = 601;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
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
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m601
    }
}
/// Type
///
/// Type of tracker
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum Typ {
    #[allow(missing_docs)]
    Unknown = 0,
    #[allow(missing_docs)]
    Fixed = 1,
    #[allow(missing_docs)]
    Horizontal = 2,
    #[allow(missing_docs)]
    Tilted = 3,
    #[allow(missing_docs)]
    Azimuth = 4,
    #[allow(missing_docs)]
    Dual = 5,
    #[allow(missing_docs)]
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
/// Global Mode
///
/// Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
///
/// Notes: The global controls all trackers
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum GlblCtl {
    #[allow(missing_docs)]
    Automatic = 0,
    #[allow(missing_docs)]
    Manual = 1,
    #[allow(missing_docs)]
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
bitflags::bitflags! {
    #[doc = " Global Alarm"] #[doc = " "] #[doc = " Global tracker alarm conditions"]
    #[doc = " "] #[doc =
    " Notes: Combined tracker alarm conditions.  See individual trackers for alarms"]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct GlblAlm : u16 {
    #[allow(missing_docs)] const SetPoint = 1; #[allow(missing_docs)] const ObsEl = 2;
    #[allow(missing_docs)] const ObsAz = 4; }
}
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
