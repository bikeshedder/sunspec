/// split single phase (ABN) meter
///
/// Notes: Float
#[derive(Debug)]
pub struct Model212 {
    /// Amps
    ///
    /// Total AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    pub phv: f32,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    pub ppv: f32,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: f32,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Hz
    ///
    /// Frequency
    pub hz: f32,
    /// Watts
    ///
    /// Total Real Power
    pub w: f32,
    /// Watts phase A
    pub wpha: Option<f32>,
    /// Watts phase B
    pub wphb: Option<f32>,
    /// Watts phase C
    pub wphc: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VA phase A
    pub vapha: Option<f32>,
    /// VA phase B
    pub vaphb: Option<f32>,
    /// VA phase C
    pub vaphc: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    pub var: Option<f32>,
    /// VAR phase A
    pub varpha: Option<f32>,
    /// VAR phase B
    pub varphb: Option<f32>,
    /// VAR phase C
    pub varphc: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pub pf: Option<f32>,
    /// PF phase A
    pub pfpha: Option<f32>,
    /// PF phase B
    pub pfphb: Option<f32>,
    /// PF phase C
    pub pfphc: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    pub totwhexp: f32,
    /// Total Watt-hours Exported phase A
    pub totwhexppha: Option<f32>,
    /// Total Watt-hours Exported phase B
    pub totwhexpphb: Option<f32>,
    /// Total Watt-hours Exported phase C
    pub totwhexpphc: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    pub totwhimp: f32,
    /// Total Watt-hours Imported phase A
    pub totwhimppha: Option<f32>,
    /// Total Watt-hours Imported phase B
    pub totwhimpphb: Option<f32>,
    /// Total Watt-hours Imported phase C
    pub totwhimpphc: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    pub totvahexp: Option<f32>,
    /// Total VA-hours Exported phase A
    pub totvahexppha: Option<f32>,
    /// Total VA-hours Exported phase B
    pub totvahexpphb: Option<f32>,
    /// Total VA-hours Exported phase C
    pub totvahexpphc: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    pub totvahimp: Option<f32>,
    /// Total VA-hours Imported phase A
    pub totvahimppha: Option<f32>,
    /// Total VA-hours Imported phase B
    pub totvahimpphb: Option<f32>,
    /// Total VA-hours Imported phase C
    pub totvahimpphc: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    pub totvarhimpq1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    pub totvarhimpq1pha: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    pub totvarhimpq1phb: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    pub totvarhimpq1phc: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    pub totvarhimpq2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    pub totvarhimpq2pha: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    pub totvarhimpq2phb: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    pub totvarhimpq2phc: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    pub totvarhexpq3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    pub totvarhexpq3pha: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    pub totvarhexpq3phb: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    pub totvarhexpq3phc: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    pub totvarhexpq4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    pub totvarhexpq4pha: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    pub totvarhexpq4phb: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    pub totvarhexpq4phc: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    pub evt: u32,
}

#[allow(missing_docs)]

impl Model212 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, f32> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, f32> = crate::PointDef::new(6, 2, false);
    pub const PHV: crate::PointDef<Self, f32> = crate::PointDef::new(8, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(10, 2, false);
    pub const PHVPHB: crate::PointDef<Self, f32> = crate::PointDef::new(12, 2, false);
    pub const PHVPHC: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PPV: crate::PointDef<Self, f32> = crate::PointDef::new(16, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, f32> = crate::PointDef::new(18, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(24, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(26, 2, false);
    pub const WPHA: crate::PointDef<Self, f32> = crate::PointDef::new(28, 2, false);
    pub const WPHB: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const WPHC: crate::PointDef<Self, f32> = crate::PointDef::new(32, 2, false);
    pub const VA: crate::PointDef<Self, f32> = crate::PointDef::new(34, 2, false);
    pub const VAPHA: crate::PointDef<Self, f32> = crate::PointDef::new(36, 2, false);
    pub const VAPHB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const VAPHC: crate::PointDef<Self, f32> = crate::PointDef::new(40, 2, false);
    pub const VAR: crate::PointDef<Self, f32> = crate::PointDef::new(42, 2, false);
    pub const VARPHA: crate::PointDef<Self, f32> = crate::PointDef::new(44, 2, false);
    pub const VARPHB: crate::PointDef<Self, f32> = crate::PointDef::new(46, 2, false);
    pub const VARPHC: crate::PointDef<Self, f32> = crate::PointDef::new(48, 2, false);
    pub const PF: crate::PointDef<Self, f32> = crate::PointDef::new(50, 2, false);
    pub const PFPHA: crate::PointDef<Self, f32> = crate::PointDef::new(52, 2, false);
    pub const PFPHB: crate::PointDef<Self, f32> = crate::PointDef::new(54, 2, false);
    pub const PFPHC: crate::PointDef<Self, f32> = crate::PointDef::new(56, 2, false);
    pub const TOTWHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(58, 2, false);
    pub const TOTWHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(60, 2, false);
    pub const TOTWHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(62, 2, false);
    pub const TOTWHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(64, 2, false);
    pub const TOTWHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(66, 2, false);
    pub const TOTWHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(68, 2, false);
    pub const TOTWHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(70, 2, false);
    pub const TOTWHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(72, 2, false);
    pub const TOTVAHEXP: crate::PointDef<Self, f32> = crate::PointDef::new(74, 2, false);
    pub const TOTVAHEXPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(76, 2, false);
    pub const TOTVAHEXPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(78, 2, false);
    pub const TOTVAHEXPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(80, 2, false);
    pub const TOTVAHIMP: crate::PointDef<Self, f32> = crate::PointDef::new(82, 2, false);
    pub const TOTVAHIMPPHA: crate::PointDef<Self, f32> = crate::PointDef::new(84, 2, false);
    pub const TOTVAHIMPPHB: crate::PointDef<Self, f32> = crate::PointDef::new(86, 2, false);
    pub const TOTVAHIMPPHC: crate::PointDef<Self, f32> = crate::PointDef::new(88, 2, false);
    pub const TOTVARHIMPQ1: crate::PointDef<Self, f32> = crate::PointDef::new(90, 2, false);
    pub const TOTVARHIMPQ1PHA: crate::PointDef<Self, f32> = crate::PointDef::new(92, 2, false);
    pub const TOTVARHIMPQ1PHB: crate::PointDef<Self, f32> = crate::PointDef::new(94, 2, false);
    pub const TOTVARHIMPQ1PHC: crate::PointDef<Self, f32> = crate::PointDef::new(96, 2, false);
    pub const TOTVARHIMPQ2: crate::PointDef<Self, f32> = crate::PointDef::new(98, 2, false);
    pub const TOTVARHIMPQ2PHA: crate::PointDef<Self, f32> = crate::PointDef::new(100, 2, false);
    pub const TOTVARHIMPQ2PHB: crate::PointDef<Self, f32> = crate::PointDef::new(102, 2, false);
    pub const TOTVARHIMPQ2PHC: crate::PointDef<Self, f32> = crate::PointDef::new(104, 2, false);
    pub const TOTVARHEXPQ3: crate::PointDef<Self, f32> = crate::PointDef::new(106, 2, false);
    pub const TOTVARHEXPQ3PHA: crate::PointDef<Self, f32> = crate::PointDef::new(108, 2, false);
    pub const TOTVARHEXPQ3PHB: crate::PointDef<Self, f32> = crate::PointDef::new(110, 2, false);
    pub const TOTVARHEXPQ3PHC: crate::PointDef<Self, f32> = crate::PointDef::new(112, 2, false);
    pub const TOTVARHEXPQ4: crate::PointDef<Self, f32> = crate::PointDef::new(114, 2, false);
    pub const TOTVARHEXPQ4PHA: crate::PointDef<Self, f32> = crate::PointDef::new(116, 2, false);
    pub const TOTVARHEXPQ4PHB: crate::PointDef<Self, f32> = crate::PointDef::new(118, 2, false);
    pub const TOTVARHEXPQ4PHC: crate::PointDef<Self, f32> = crate::PointDef::new(120, 2, false);
    pub const EVT: crate::PointDef<Self, u32> = crate::PointDef::new(122, 2, false);
}

impl crate::Model for Model212 {
    const ID: u16 = 212;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            apha: Self::APHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphb: Self::APHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            aphc: Self::APHC.from_data(data)?,
            phv: Self::PHV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            ppv: Self::PPV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wpha: Self::WPHA.from_data(data)?,
            wphb: Self::WPHB.from_data(data)?,
            wphc: Self::WPHC.from_data(data)?,
            va: Self::VA.from_data(data)?,
            vapha: Self::VAPHA.from_data(data)?,
            vaphb: Self::VAPHB.from_data(data)?,
            vaphc: Self::VAPHC.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            varpha: Self::VARPHA.from_data(data)?,
            varphb: Self::VARPHB.from_data(data)?,
            varphc: Self::VARPHC.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pfpha: Self::PFPHA.from_data(data)?,
            pfphb: Self::PFPHB.from_data(data)?,
            pfphc: Self::PFPHC.from_data(data)?,
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
            totvahexp: Self::TOTVAHEXP.from_data(data)?,
            totvahexppha: Self::TOTVAHEXPPHA.from_data(data)?,
            totvahexpphb: Self::TOTVAHEXPPHB.from_data(data)?,
            totvahexpphc: Self::TOTVAHEXPPHC.from_data(data)?,
            totvahimp: Self::TOTVAHIMP.from_data(data)?,
            totvahimppha: Self::TOTVAHIMPPHA.from_data(data)?,
            totvahimpphb: Self::TOTVAHIMPPHB.from_data(data)?,
            totvahimpphc: Self::TOTVAHIMPPHC.from_data(data)?,
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
            evt: Self::EVT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
