/// Meter (Single Phase)single phase (AN or AB) meter
///
/// Include this model for single phase (AN or AB) metering
#[derive(Debug)]
pub struct Model201 {
    /// Amps
    ///
    /// Total AC Current
    pub a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: i16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<i16>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<i16>,
    /// Current scale factor
    pub a_sf: i16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ///
    /// Notes: Conditional AN connection
    pub phv: Option<i16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ///
    /// Notes: Conditional AN connection
    pub phvpha: Option<i16>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<i16>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: Option<i16>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ///
    /// Notes: Conditional AB connection
    pub ppvphab: Option<i16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<i16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<i16>,
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
    /// Watts phase A
    pub wpha: Option<i16>,
    /// Watts phase B
    pub wphb: Option<i16>,
    /// Watts phase C
    pub wphc: Option<i16>,
    /// Real Power scale factor
    pub w_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    /// VA phase A
    pub vapha: Option<i16>,
    /// VA phase B
    pub vaphb: Option<i16>,
    /// VA phase C
    pub vaphc: Option<i16>,
    /// Apparent Power scale factor
    pub va_sf: Option<i16>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<i16>,
    /// VAR phase A
    pub varpha: Option<i16>,
    /// VAR phase B
    pub varphb: Option<i16>,
    /// VAR phase C
    pub varphc: Option<i16>,
    /// Reactive Power scale factor
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<i16>,
    /// PF phase A
    pub pfpha: Option<i16>,
    /// PF phase B
    pub pfphb: Option<i16>,
    /// PF phase C
    pub pfphc: Option<i16>,
    /// Power Factor scale factor
    pub pf_sf: Option<i16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: u32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<u32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<u32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: u32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<u32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<u32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<u32>,
    /// Real Energy scale factor
    pub totwh_sf: i16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<u32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<u32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<u32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<u32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<u32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<u32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<u32>,
    /// Apparent Energy scale factor
    pub totvah_sf: Option<i16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<u32>,
    /// Reactive Energy scale factor
    pub totvarh_sf: Option<i16>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model201 {
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PHV: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const PHVPHA: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const PHVPHB: crate::PointDef<Self, i16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHC: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, false);
    pub const PPV: crate::PointDef<Self, i16> = crate::PointDef::new(9, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, i16> = crate::PointDef::new(10, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, i16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WPHA: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const WPHB: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const WPHC: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const VAPHA: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VAPHB: crate::PointDef<Self, i16> = crate::PointDef::new(23, 1, false);
    pub const VAPHC: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(25, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const VARPHA: crate::PointDef<Self, i16> = crate::PointDef::new(27, 1, false);
    pub const VARPHB: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const VARPHC: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const PFPHA: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const PFPHB: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const PFPHC: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const TOTWHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(36, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const TOTWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const TOTVAHEXP: crate::PointDef<Self, u32> = crate::PointDef::new(53, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(55, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(57, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(59, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, u32> = crate::PointDef::new(61, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, u32> = crate::PointDef::new(63, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, u32> = crate::PointDef::new(65, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, u32> = crate::PointDef::new(67, 2, false);
    pub const TOTVAH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(69, 1, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, u32> = crate::PointDef::new(70, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, u32> = crate::PointDef::new(72, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, u32> = crate::PointDef::new(74, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, u32> = crate::PointDef::new(76, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, u32> = crate::PointDef::new(78, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, u32> = crate::PointDef::new(80, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, u32> = crate::PointDef::new(82, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, u32> = crate::PointDef::new(84, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, u32> = crate::PointDef::new(86, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, u32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, u32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, u32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, u32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, u32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, u32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, u32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(102, 1, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(103, 2, false);
}

impl crate::Model for Model201 {
    const ID: u16 = 201;
    const LENGTH: u16 = 105;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phv: Self::PHV.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
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
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            totwhexp: Self::TOTWHEXP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhexppha: Self::TOTWHEXPPHA.from_data(data)?,
            totwhexpphb: Self::TOTWHEXPPHB.from_data(data)?,
            totwhexpphc: Self::TOTWHEXPPHC.from_data(data)?,
            totwhimp: Self::TOTWHIMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totwhimppha: Self::TOTWHIMPPHA.from_data(data)?,
            totwhimpphb: Self::TOTWHIMPPHB.from_data(data)?,
            totwhimpphc: Self::TOTWHIMPPHC.from_data(data)?,
            totwh_sf: Self::TOTWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
            totvah_sf: Self::TOTVAH_SF.from_data(data)?,
            totvarhimpq1: Self::TOTVARHIMPQ1.from_data(data)?,
            totvarhimpq1pha: Self::TOTVARHIMPQ1PHA.from_data(data)?,
            totvarhimpq1phb: Self::TOTVARHIMPQ1PHB.from_data(data)?,
            totvarhimpq1phc: Self::TOTVARHIMPQ1PHC.from_data(data)?,
            totvarhimpq2: Self::TOTVARHIMPQ2.from_data(data)?,
            totvarhimpq2pha: Self::TOTVARHIMPQ2PHA.from_data(data)?,
            totvarhimpq2phb: Self::TOTVARHIMPQ2PHB.from_data(data)?,
            totvarhimpq2phc: Self::TOTVARHIMPQ2PHC.from_data(data)?,
            totvarhexpq3: Self::TOTVARHEXPQ3.from_data(data)?,
            totvarhexpq3pha: Self::TOTVARHEXPQ3PHA.from_data(data)?,
            totvarhexpq3phb: Self::TOTVARHEXPQ3PHB.from_data(data)?,
            totvarhexpq3phc: Self::TOTVARHEXPQ3PHC.from_data(data)?,
            totvarhexpq4: Self::TOTVARHEXPQ4.from_data(data)?,
            totvarhexpq4pha: Self::TOTVARHEXPQ4PHA.from_data(data)?,
            totvarhexpq4phb: Self::TOTVARHEXPQ4PHB.from_data(data)?,
            totvarhexpq4phc: Self::TOTVARHEXPQ4PHC.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
