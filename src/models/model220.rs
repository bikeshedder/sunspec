/// Secure AC Meter Selected Readings
///
/// Include this model for secure metering
#[derive(Debug)]
pub struct Model220 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage
    ///
    /// Average phase or line voltage
    pub phv: Option<i16>,
    /// Voltage scale factor
    pub v_sf: i16,
    /// Hz
    ///
    /// Frequency
    pub hz: i16,
    /// Frequency scale factor
    pub hz_sf: Option<i16>,
    /// Watts
    ///
    /// Total Real Power
    pub w: i16,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    pub ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    pub ms: u16,
    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Notes: Shall be advanced for each request
    pub seq: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// Notes: For future proof
    pub alg: u16,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// Notes: The value of N must be at least 4 (64 bits)
    pub n: u16,
}

#[allow(missing_docs)]

impl Model220 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(14, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(16, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(19, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(21, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(33, 2, false);
    pub const TS: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const MS: crate::PointDef<Self, u16> = crate::PointDef::new(38, 1, false);
    pub const SEQ: crate::PointDef<Self, u16> = crate::PointDef::new(39, 1, false);
    pub const ALG: crate::PointDef<Self, u16> = crate::PointDef::new(40, 1, false);
    pub const N: crate::PointDef<Self, u16> = crate::PointDef::new(41, 1, false);
}

impl crate::Model for Model220 {
    const ID: u16 = 220;
    const LENGTH: u16 = 43;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ts: Self::TS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ms: Self::MS
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            seq: Self::SEQ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            alg: Self::ALG
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            n: Self::N
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
