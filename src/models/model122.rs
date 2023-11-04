/// Measurements_Status
///
/// Inverter Controls Extended Measurements and Status
///
/// Notes: Ref 3: 8.14.3.2, Ref 4: 17
#[derive(Debug)]
pub struct Model122 {
    /// PVConn
    ///
    /// PV inverter present/available status. Enumerated value.
    pub pvconn: u16,
    /// StorConn
    ///
    /// Storage inverter present/available status. Enumerated value.
    pub storconn: u16,
    /// ECPConn
    ///
    /// ECP connection status: disconnected=0  connected=1.
    pub ecpconn: u16,
    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    pub actwh: Option<u64>,
    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    pub actvah: Option<u64>,
    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    pub actvarhq1: Option<u64>,
    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    pub actvarhq2: Option<u64>,
    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output  in quadrant 3.
    pub actvarhq3: Option<u64>,
    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output  in quadrant 4.
    pub actvarhq4: Option<u64>,
    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    pub varaval: Option<i16>,
    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    pub varaval_sf: Option<i16>,
    /// WAval
    ///
    /// Amount of Watts available.
    pub waval: Option<u16>,
    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    pub waval_sf: Option<i16>,
    /// StSetLimMsk
    ///
    /// Bit Mask indicating setpoint limit(s) reached.
    ///
    /// Notes: Bits shall be automatically cleared on read.
    pub stsetlimmsk: Option<u32>,
    /// StActCtl
    ///
    /// Bit Mask indicating which inverter controls are currently active.
    pub stactctl: Option<u32>,
    /// TmSrc
    ///
    /// Source of time synchronization.
    pub tmsrc: Option<String>,
    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    pub tms: Option<u32>,
    /// RtSt
    ///
    /// Bit Mask indicating active ride-through status.
    pub rtst: Option<u16>,
    /// Ris
    ///
    /// Isolation resistance.
    pub ris: Option<u16>,
    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    pub ris_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model122 {
    pub const PVCONN: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const STORCONN: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const ECPCONN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const ACTWH: crate::PointDef<Self, u64> = crate::PointDef::new(3, 4, false);
    pub const ACTVAH: crate::PointDef<Self, u64> = crate::PointDef::new(7, 4, false);
    pub const ACTVARHQ1: crate::PointDef<Self, u64> = crate::PointDef::new(11, 4, false);
    pub const ACTVARHQ2: crate::PointDef<Self, u64> = crate::PointDef::new(15, 4, false);
    pub const ACTVARHQ3: crate::PointDef<Self, u64> = crate::PointDef::new(19, 4, false);
    pub const ACTVARHQ4: crate::PointDef<Self, u64> = crate::PointDef::new(23, 4, false);
    pub const VARAVAL: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARAVAL_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const WAVAL: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
    pub const WAVAL_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const STSETLIMMSK: crate::PointDef<Self, u32> = crate::PointDef::new(31, 2, false);
    pub const STACTCTL: crate::PointDef<Self, u32> = crate::PointDef::new(33, 2, false);
    pub const TMSRC: crate::PointDef<Self, String> = crate::PointDef::new(35, 4, false);
    pub const TMS: crate::PointDef<Self, u32> = crate::PointDef::new(39, 2, false);
    pub const RTST: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
    pub const RIS: crate::PointDef<Self, u16> = crate::PointDef::new(42, 1, false);
    pub const RIS_SF: crate::PointDef<Self, i16> = crate::PointDef::new(43, 1, false);
}

impl crate::Model for Model122 {
    const ID: u16 = 122;
    const LENGTH: u16 = 44;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            pvconn: Self::PVCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            storconn: Self::STORCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ecpconn: Self::ECPCONN
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            actwh: Self::ACTWH.from_data(data)?,
            actvah: Self::ACTVAH.from_data(data)?,
            actvarhq1: Self::ACTVARHQ1.from_data(data)?,
            actvarhq2: Self::ACTVARHQ2.from_data(data)?,
            actvarhq3: Self::ACTVARHQ3.from_data(data)?,
            actvarhq4: Self::ACTVARHQ4.from_data(data)?,
            varaval: Self::VARAVAL.from_data(data)?,
            varaval_sf: Self::VARAVAL_SF.from_data(data)?,
            waval: Self::WAVAL.from_data(data)?,
            waval_sf: Self::WAVAL_SF.from_data(data)?,
            stsetlimmsk: Self::STSETLIMMSK.from_data(data)?,
            stactctl: Self::STACTCTL.from_data(data)?,
            tmsrc: Self::TMSRC.from_data(data)?,
            tms: Self::TMS.from_data(data)?,
            rtst: Self::RTST.from_data(data)?,
            ris: Self::RIS.from_data(data)?,
            ris_sf: Self::RIS_SF.from_data(data)?,
        })
    }
}
