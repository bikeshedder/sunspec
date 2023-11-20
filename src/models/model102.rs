/// Inverter (Split-Phase)
///
/// Include this model for split phase inverter monitoring
#[derive(Debug)]
pub struct Model102 {
    /// Amps
    ///
    /// AC Current
    ///
    /// Notes: Sum of active phases
    pub a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Notes: Connected Phase
    pub apha: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Notes: Connected Phase
    pub aphb: u16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    pub aphc: Option<u16>,
    #[allow(missing_docs)]
    pub a_sf: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pub ppvphab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pub ppvphbc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pub ppvphca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    pub phvpha: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    pub phvphb: u16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    pub phvphc: Option<u16>,
    #[allow(missing_docs)]
    pub v_sf: i16,
    /// Watts
    ///
    /// AC Power
    pub w: i16,
    #[allow(missing_docs)]
    pub w_sf: i16,
    /// Hz
    ///
    /// Line Frequency
    pub hz: u16,
    #[allow(missing_docs)]
    pub hz_sf: i16,
    /// VA
    ///
    /// AC Apparent Power
    pub va: Option<i16>,
    #[allow(missing_docs)]
    pub va_sf: Option<i16>,
    /// VAr
    ///
    /// AC Reactive Power
    pub var: Option<i16>,
    #[allow(missing_docs)]
    pub var_sf: Option<i16>,
    /// PF
    ///
    /// AC Power Factor
    pub pf: Option<i16>,
    #[allow(missing_docs)]
    pub pf_sf: Option<i16>,
    /// WattHours
    ///
    /// AC Energy
    pub wh: u32,
    #[allow(missing_docs)]
    pub wh_sf: i16,
    /// DC Amps
    ///
    /// DC Current
    pub dca: Option<u16>,
    #[allow(missing_docs)]
    pub dca_sf: Option<i16>,
    /// DC Voltage
    ///
    /// DC Voltage
    pub dcv: Option<u16>,
    #[allow(missing_docs)]
    pub dcv_sf: Option<i16>,
    /// DC Watts
    ///
    /// DC Power
    pub dcw: Option<i16>,
    #[allow(missing_docs)]
    pub dcw_sf: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    pub tmpcab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    pub tmpsnk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    pub tmptrns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    pub tmpot: Option<i16>,
    #[allow(missing_docs)]
    pub tmp_sf: i16,
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

impl Model102 {
    pub const A: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const APHA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const APHB: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const APHC: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const PPVPHAB: crate::PointDef<Self, u16> = crate::PointDef::new(5, 1, false);
    pub const PPVPHBC: crate::PointDef<Self, u16> = crate::PointDef::new(6, 1, false);
    pub const PPVPHCA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const PHVPHA: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const PHVPHB: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const PHVPHC: crate::PointDef<Self, u16> = crate::PointDef::new(10, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(11, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(12, 1, false);
    pub const W_SF: crate::PointDef<Self, i16> = crate::PointDef::new(13, 1, false);
    pub const HZ: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const HZ_SF: crate::PointDef<Self, i16> = crate::PointDef::new(15, 1, false);
    pub const VA: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const VA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VAR: crate::PointDef<Self, i16> = crate::PointDef::new(18, 1, false);
    pub const VAR_SF: crate::PointDef<Self, i16> = crate::PointDef::new(19, 1, false);
    pub const PF: crate::PointDef<Self, i16> = crate::PointDef::new(20, 1, false);
    pub const PF_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const WH: crate::PointDef<Self, u32> = crate::PointDef::new(22, 2, false);
    pub const WH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(24, 1, false);
    pub const DCA: crate::PointDef<Self, u16> = crate::PointDef::new(25, 1, false);
    pub const DCA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(26, 1, false);
    pub const DCV: crate::PointDef<Self, u16> = crate::PointDef::new(27, 1, false);
    pub const DCV_SF: crate::PointDef<Self, i16> = crate::PointDef::new(28, 1, false);
    pub const DCW: crate::PointDef<Self, i16> = crate::PointDef::new(29, 1, false);
    pub const DCW_SF: crate::PointDef<Self, i16> = crate::PointDef::new(30, 1, false);
    pub const TMPCAB: crate::PointDef<Self, i16> = crate::PointDef::new(31, 1, false);
    pub const TMPSNK: crate::PointDef<Self, i16> = crate::PointDef::new(32, 1, false);
    pub const TMPTRNS: crate::PointDef<Self, i16> = crate::PointDef::new(33, 1, false);
    pub const TMPOT: crate::PointDef<Self, i16> = crate::PointDef::new(34, 1, false);
    pub const TMP_SF: crate::PointDef<Self, i16> = crate::PointDef::new(35, 1, false);
    pub const ST: crate::PointDef<Self, u16> = crate::PointDef::new(36, 1, false);
    pub const STVND: crate::PointDef<Self, u16> = crate::PointDef::new(37, 1, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(38, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(40, 2, false);
    pub const EVTVND1: crate::PointDef<Self, u32> = crate::PointDef::new(42, 2, false);
    pub const EVTVND2: crate::PointDef<Self, u32> = crate::PointDef::new(44, 2, false);
    pub const EVTVND3: crate::PointDef<Self, u32> = crate::PointDef::new(46, 2, false);
    pub const EVTVND4: crate::PointDef<Self, u32> = crate::PointDef::new(48, 2, false);
}

impl crate::Model for Model102 {
    const ID: u16 = 102;
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
            a_sf: Self::A_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ppvphab: Self::PPVPHAB.from_data(data)?,
            ppvphbc: Self::PPVPHBC.from_data(data)?,
            ppvphca: Self::PPVPHCA.from_data(data)?,
            phvpha: Self::PHVPHA
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphb: Self::PHVPHB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            phvphc: Self::PHVPHC.from_data(data)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w: Self::W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            w_sf: Self::W_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz: Self::HZ
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            hz_sf: Self::HZ_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            va: Self::VA.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wh: Self::WH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            wh_sf: Self::WH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            dca: Self::DCA.from_data(data)?,
            dca_sf: Self::DCA_SF.from_data(data)?,
            dcv: Self::DCV.from_data(data)?,
            dcv_sf: Self::DCV_SF.from_data(data)?,
            dcw: Self::DCW.from_data(data)?,
            dcw_sf: Self::DCW_SF.from_data(data)?,
            tmpcab: Self::TMPCAB
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            tmp_sf: Self::TMP_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            st: Self::ST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            stvnd: Self::STVND.from_data(data)?,
            evt1: Self::EVT1
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evt2: Self::EVT2
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            evtvnd1: Self::EVTVND1.from_data(data)?,
            evtvnd2: Self::EVTVND2.from_data(data)?,
            evtvnd3: Self::EVTVND3.from_data(data)?,
            evtvnd4: Self::EVTVND4.from_data(data)?,
        })
    }
}
