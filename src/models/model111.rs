/// Inverter (Single Phase) FLOAT
///
/// Include this model for single phase inverter monitoring using float values
#[derive(Debug)]
pub struct Model111 {
    /// Amps
    ///
    /// AC Current
    pub a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    pub aphb: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<f32>,
    /// Watts
    ///
    /// AC Power
    pub w: f32,
    /// Hz
    ///
    /// Line Frequency
    pub hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: f32,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<f32>,
    /// Operating State
    ///
    /// Enumerated value.  Operating state
    pub st: u16,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    pub stvnd: Option<u16>,
    /// Event1
    ///
    /// Bitmask value. Event fields
    pub evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    pub evtvnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    pub evtvnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    pub evtvnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    pub evtvnd4: Option<u32>,
}

#[allow(missing_docs)]

impl Model111 {
    pub const A: crate::PointDef<Self, f32> = crate::PointDef::new(0, 2, false);
    pub const APHA: crate::PointDef<Self, f32> = crate::PointDef::new(2, 2, false);
    pub const APHB: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(4, 2, false);
    pub const APHC: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(6, 2, false);
    pub const PPVPHAB: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(8, 2, false);
    pub const PPVPHBC: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(10, 2, false);
    pub const PPVPHCA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(12, 2, false);
    pub const PHVPHA: crate::PointDef<Self, f32> = crate::PointDef::new(14, 2, false);
    pub const PHVPHB: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(16, 2, false);
    pub const PHVPHC: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(18, 2, false);
    pub const W: crate::PointDef<Self, f32> = crate::PointDef::new(20, 2, false);
    pub const HZ: crate::PointDef<Self, f32> = crate::PointDef::new(22, 2, false);
    pub const VA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(24, 2, false);
    pub const VAR: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(26, 2, false);
    pub const PF: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(28, 2, false);
    pub const WH: crate::PointDef<Self, f32> = crate::PointDef::new(30, 2, false);
    pub const DCA: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(32, 2, false);
    pub const DCV: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(34, 2, false);
    pub const DCW: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(36, 2, false);
    pub const TMPCAB: crate::PointDef<Self, f32> = crate::PointDef::new(38, 2, false);
    pub const TMPSNK: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(40, 2, false);
    pub const TMPTRNS: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(42, 2, false);
    pub const TMPOT: crate::PointDef<Self, Option<f32>> = crate::PointDef::new(44, 2, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(46, 1, false);
    pub const STVND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(47, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(50, 2, false);
    pub const EVTVND1: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(52, 2, false);
    pub const EVTVND2: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(54, 2, false);
    pub const EVTVND3: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(56, 2, false);
    pub const EVTVND4: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(58, 2, false);
}

impl crate::Model for Model111 {
    const ID: u16 = 111;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            a: Self::A.from_data(data)?,
            apha: Self::APHA.from_data(data)?,
            aphb: Self::APHB.from_data(data)?,
            aphc: Self::APHC.from_data(data)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA.from_data(data)?,
            phvphb: Self::PHVPHB.from_data(data)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            w: Self::W.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            va: Self::VA.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            wh: Self::WH.from_data(data)?,
            dca: Self::DCA.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            tmpcab: Self::TMPCAB.from_data(data)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            st: Self::ST.from_data(data)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1.from_data(data)?,
            evt2: Self::EVT2.from_data(data)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}
