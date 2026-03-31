//! Tracker Controller DRAFT 2
/// Type alias for [`TrackerController`].
pub type Model601 = TrackerController;
/// Tracker Controller DRAFT 2
///
/// Monitors and controls multiple trackers
///
/// Detail: Trackers may include GPS model 305 for location information
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct TrackerController {
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
    /// Detail: The global controls all trackers
    pub glbl_ctl: Option<GlblCtl>,
    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Detail: Combined tracker alarm conditions.  See individual trackers for alarms
    pub glbl_alm: Option<GlblAlm>,
    /// SF
    ///
    /// Scale Factor for targets and position measurements in degrees
    pub dgr_sf: i16,
    /// Trackers
    ///
    /// Number of trackers being controlled.  Size of repeating block.
    pub n: u16,
    #[allow(missing_docs)]
    pub tracker: Vec<Tracker>,
}
#[allow(missing_docs)]
impl TrackerController {
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
static TRACKER_CONTROLLER_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "nam",
        label: "Controller",
        description: "Descriptive name for this control unit",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "typ",
        label: "Type",
        description: "Type of tracker",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dt_loc",
        label: "Date",
        description: "Local date in YYYYMMDD format",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tm_loc",
        label: "Time",
        description: "24 hour local time stamp to second",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "day",
        label: "Day",
        description: "Number of the day in the year (1-366)",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "glbl_el_ctl",
        label: "Manual Elevation",
        description: "Global manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "glbl_az_ctl",
        label: "Manual Azimuth",
        description: "Global manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "glbl_ctl",
        label: "Global Mode",
        description: "Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "glbl_alm",
        label: "Global Alarm",
        description: "Global tracker alarm conditions",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dgr_sf",
        label: "SF",
        description: "Scale Factor for targets and position measurements in degrees",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n",
        label: "Trackers",
        description: "Number of trackers being controlled.  Size of repeating block.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "tracker",
        label: "tracker",
        description: "",
        kind: crate::FieldKind::RepeatingGroup(<Tracker as crate::GroupMeta>::group_info),
    },
];
static TRACKER_CONTROLLER_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "tracker_controller",
    label: "Tracker Controller DRAFT 2",
    description: "Monitors and controls multiple trackers",
    fields: TRACKER_CONTROLLER_FIELDS,
};
impl crate::GroupMeta for TrackerController {
    fn group_info() -> &'static crate::GroupInfo {
        &TRACKER_CONTROLLER_GROUP_INFO
    }
}
impl crate::Group for TrackerController {
    const LEN: u16 = 26;
}
impl TrackerController {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, tracker) = Tracker::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
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
                tracker,
            },
        ))
    }
}
/// Type
///
/// Type of tracker
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Typ {
    #[allow(missing_docs)]
    Unknown,
    #[allow(missing_docs)]
    Fixed,
    #[allow(missing_docs)]
    Horizontal,
    #[allow(missing_docs)]
    Tilted,
    #[allow(missing_docs)]
    Azimuth,
    #[allow(missing_docs)]
    Dual,
    #[allow(missing_docs)]
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Typ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Fixed,
            2 => Self::Horizontal,
            3 => Self::Tilted,
            4 => Self::Azimuth,
            5 => Self::Dual,
            99 => Self::Other,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Unknown => 0,
            Self::Fixed => 1,
            Self::Horizontal => 2,
            Self::Tilted => 3,
            Self::Azimuth => 4,
            Self::Dual => 5,
            Self::Other => 99,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Typ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Global Mode
///
/// Global Control register operates on all trackers. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
///
/// Detail: The global controls all trackers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum GlblCtl {
    #[allow(missing_docs)]
    Automatic,
    #[allow(missing_docs)]
    Manual,
    #[allow(missing_docs)]
    Calibrate,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for GlblCtl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Automatic,
            1 => Self::Manual,
            2 => Self::Calibrate,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Automatic => 0,
            Self::Manual => 1,
            Self::Calibrate => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for GlblCtl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Global Alarm"] #[doc = " "] #[doc = " Global tracker alarm conditions"]
    #[doc = " "] #[doc =
    " Detail: Combined tracker alarm conditions.  See individual trackers for alarms"]
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
impl crate::FixedSize for GlblAlm {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
#[allow(missing_docs)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Tracker {
    /// Tracker
    ///
    /// Descriptive name for this tracker unit
    pub id: Option<String>,
    /// Target Elevation
    ///
    /// Auto target elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub el_trgt: Option<i32>,
    /// Target Azimuth
    ///
    /// Auto target azimuth  in degrees from true north towards east.  Unimplemented for single axis horizontal tracker type
    pub az_trgt: Option<i32>,
    /// Elevation
    ///
    /// Actual elevation position  in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub el_pos: Option<i32>,
    /// Azimuth
    ///
    /// Actual azimuth position  in degrees from true north towards east.  Unimplemented for single axis horizontal tracker type
    pub az_pos: Option<i32>,
    /// Manual Elevation
    ///
    /// Manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type
    pub el_ctl: Option<i32>,
    /// Manual Azimuth
    ///
    /// Manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type
    pub az_ctl: Option<i32>,
    /// Mode
    ///
    /// Control register. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    pub ctl: Option<TrackerCtl>,
    /// Alarm
    ///
    /// Tracker alarm conditions
    pub alm: Option<TrackerAlm>,
}
#[allow(missing_docs)]
impl Tracker {
    pub const ID: crate::Point<Self, Option<String>> = crate::Point::new(0, 8, false);
    pub const EL_TRGT: crate::Point<Self, Option<i32>> = crate::Point::new(8, 2, false);
    pub const AZ_TRGT: crate::Point<Self, Option<i32>> = crate::Point::new(10, 2, false);
    pub const EL_POS: crate::Point<Self, Option<i32>> = crate::Point::new(12, 2, false);
    pub const AZ_POS: crate::Point<Self, Option<i32>> = crate::Point::new(14, 2, false);
    pub const EL_CTL: crate::Point<Self, Option<i32>> = crate::Point::new(16, 2, true);
    pub const AZ_CTL: crate::Point<Self, Option<i32>> = crate::Point::new(18, 2, true);
    pub const CTL: crate::Point<Self, Option<TrackerCtl>> = crate::Point::new(20, 1, true);
    pub const ALM: crate::Point<Self, Option<TrackerAlm>> = crate::Point::new(21, 1, false);
}
static TRACKER_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "id",
        label: "Tracker",
        description: "Descriptive name for this tracker unit",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "el_trgt",
        label: "Target Elevation",
        description: "Auto target elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "az_trgt",
        label: "Target Azimuth",
        description: "Auto target azimuth  in degrees from true north towards east.  Unimplemented for single axis horizontal tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "el_pos",
        label: "Elevation",
        description: "Actual elevation position  in degrees from horizontal.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "az_pos",
        label: "Azimuth",
        description: "Actual azimuth position  in degrees from true north towards east.  Unimplemented for single axis horizontal tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "el_ctl",
        label: "Manual Elevation",
        description: "Manual override target position of elevation in degrees from horizontal.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "az_ctl",
        label: "Manual Azimuth",
        description: "Manual override target position of azimuth in degrees from true north towards east.  Unimplemented for single axis azimuth tracker type",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ctl",
        label: "Mode",
        description: "Control register. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alm",
        label: "Alarm",
        description: "Tracker alarm conditions",
        kind: crate::FieldKind::Point,
    },
];
static TRACKER_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "tracker",
    label: "tracker",
    description: "",
    fields: TRACKER_FIELDS,
};
impl crate::GroupMeta for Tracker {
    fn group_info() -> &'static crate::GroupInfo {
        &TRACKER_GROUP_INFO
    }
}
impl crate::Group for Tracker {
    const LEN: u16 = 22;
}
impl Tracker {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                id: Self::ID.from_data(data)?,
                el_trgt: Self::EL_TRGT.from_data(data)?,
                az_trgt: Self::AZ_TRGT.from_data(data)?,
                el_pos: Self::EL_POS.from_data(data)?,
                az_pos: Self::AZ_POS.from_data(data)?,
                el_ctl: Self::EL_CTL.from_data(data)?,
                az_ctl: Self::AZ_CTL.from_data(data)?,
                ctl: Self::CTL.from_data(data)?,
                alm: Self::ALM.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<Tracker as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = Tracker::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
/// Mode
///
/// Control register. Normal operation is automatic.  Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum TrackerCtl {
    #[allow(missing_docs)]
    Automatic,
    #[allow(missing_docs)]
    Manual,
    #[allow(missing_docs)]
    Calibrate,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for TrackerCtl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Automatic,
            1 => Self::Manual,
            2 => Self::Calibrate,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Automatic => 0,
            Self::Manual => 1,
            Self::Calibrate => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for TrackerCtl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Alarm"] #[doc = " "] #[doc = " Tracker alarm conditions"] #[derive(Copy,
    Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct TrackerAlm : u16 {
    #[allow(missing_docs)] const SetPoint = 1; #[allow(missing_docs)] const ObsEl = 2;
    #[allow(missing_docs)] const ObsAz = 4; }
}
impl crate::Value for TrackerAlm {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for TrackerAlm {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
impl crate::Model for TrackerController {
    const ID: u16 = 601;
    const NAME: &'static str = "tracker_controller";
    const LABEL: &'static str = "Tracker Controller DRAFT 2";
    const DESCRIPTION: &'static str = "Monitors and controls multiple trackers";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m601
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
