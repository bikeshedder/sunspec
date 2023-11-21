/// DER AC Measurement
///
/// DER AC measurement model.
#[derive(Debug)]
pub struct Model701 {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    ///
    /// Comments: Wiring Type
    pub actype: u16,
    /// Operating State
    ///
    /// Operating state of the DER.
    ///
    /// Comments: Operating State
    pub st: Option<u16>,
    /// Inverter State
    ///
    /// Enumerated value.  Inverter state.
    ///
    /// Comments: Inverter State
    pub invst: Option<u16>,
    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    ///
    /// Comments: Grid Connection State
    pub connst: Option<u16>,
    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    ///
    /// Comments: Alarms
    pub alrm: Option<u32>,
    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    pub dermode: Option<u32>,
    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    ///
    /// Comments: Summary
    pub w: Option<i16>,
    /// Apparent Power
    ///
    /// Total apparent power.
    pub va: Option<i16>,
    /// Reactive Power
    ///
    /// Total reactive power.
    pub var: Option<i16>,
    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    pub pf: Option<i16>,
    /// Total AC Current
    ///
    /// Total AC current.
    pub a: Option<i16>,
    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    pub llv: Option<u16>,
    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    pub lnv: Option<u16>,
    /// Frequency
    ///
    /// AC frequency.
    pub hz: Option<u32>,
    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    pub totwhinj: Option<u64>,
    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    pub totwhabs: Option<u64>,
    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    pub totvarhinj: Option<u64>,
    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    pub totvarhabs: Option<u64>,
    /// Ambient Temperature
    ///
    /// Ambient temperature.
    ///
    /// Comments: Temperatures
    pub tmpamb: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    pub tmpcab: Option<i16>,
    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    pub tmpsnk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer temperature.
    pub tmptrns: Option<i16>,
    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    pub tmpsw: Option<i16>,
    /// Other Temperature
    ///
    /// Other temperature.
    pub tmpot: Option<i16>,
    /// Watts L1
    ///
    /// Active power L1.
    ///
    /// Comments: L1
    pub wl1: Option<i16>,
    /// VA L1
    ///
    /// Apparent power L1.
    pub val1: Option<i16>,
    /// Var L1
    ///
    /// Reactive power L1.
    pub varl1: Option<i16>,
    /// PF L1
    ///
    /// Power factor phase L1.
    pub pfl1: Option<i16>,
    /// Amps L1
    ///
    /// Current phase L1.
    pub al1: Option<i16>,
    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    pub vl1l2: Option<u16>,
    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    pub vl1: Option<u16>,
    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    pub totwhinjl1: Option<u64>,
    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    pub totwhabsl1: Option<u64>,
    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    pub totvarhinjl1: Option<u64>,
    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    pub totvarhabsl1: Option<u64>,
    /// Watts L2
    ///
    /// Active power L2.
    ///
    /// Comments: L2
    pub wl2: Option<i16>,
    /// VA L2
    ///
    /// Apparent power L2.
    pub val2: Option<i16>,
    /// Var L2
    ///
    /// Reactive power L2.
    pub varl2: Option<i16>,
    /// PF L2
    ///
    /// Power factor L2.
    pub pfl2: Option<i16>,
    /// Amps L2
    ///
    /// Current L2.
    pub al2: Option<i16>,
    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    pub vl2l3: Option<u16>,
    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    pub vl2: Option<u16>,
    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    pub totwhinjl2: Option<u64>,
    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    pub totwhabsl2: Option<u64>,
    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    pub totvarhinjl2: Option<u64>,
    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    pub totvarhabsl2: Option<u64>,
    /// Watts L3
    ///
    /// Active power L3.
    ///
    /// Comments: L3
    pub wl3: Option<i16>,
    /// VA L3
    ///
    /// Apparent power L3.
    pub val3: Option<i16>,
    /// Var L3
    ///
    /// Reactive power L3.
    pub varl3: Option<i16>,
    /// PF L3
    ///
    /// Power factor L3.
    pub pfl3: Option<i16>,
    /// Amps L3
    ///
    /// Current L3.
    pub al3: Option<i16>,
    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    pub vl3l1: Option<u16>,
    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    pub vl3: Option<u16>,
    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    pub totwhinjl3: Option<u64>,
    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    pub totwhabsl3: Option<u64>,
    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    pub totvarhinjl3: Option<u64>,
    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    pub totvarhabsl3: Option<u64>,
    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    ///
    /// Comments: Active Power Throttling
    pub throtpct: Option<u16>,
    /// Throttle Source Information
    ///
    /// Active throttling source.
    pub throtsrc: Option<u32>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    ///
    /// Comments: Scale Factors
    pub a_sf: Option<i16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    pub v_sf: Option<i16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    pub hz_sf: Option<i16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    pub w_sf: Option<i16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pub pf_sf: Option<i16>,
    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    pub va_sf: Option<i16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    pub var_sf: Option<i16>,
    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    pub totwh_sf: Option<i16>,
    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    pub totvarh_sf: Option<i16>,
    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    pub tmp_sf: Option<i16>,
    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    ///
    /// Comments: Manufacturer Alarm Information
    pub mnalrminfo: Option<String>,
}

#[allow(missing_docs)]

impl Model701 {
    pub const ACTYPE: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const ST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, false);
    pub const INVST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(2, 1, false);
    pub const CONNST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(3, 1, false);
    pub const ALRM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, false);
    pub const DERMODE: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(6, 2, false);
    pub const W: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(8, 1, false);
    pub const VA: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(9, 1, false);
    pub const VAR: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(10, 1, false);
    pub const PF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(11, 1, false);
    pub const A: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(12, 1, false);
    pub const LLV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, false);
    pub const LNV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const HZ: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(15, 2, false);
    pub const TOTWHINJ: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(17, 4, false);
    pub const TOTWHABS: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(21, 4, false);
    pub const TOTVARHINJ: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(25, 4, false);
    pub const TOTVARHABS: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(29, 4, false);
    pub const TMPAMB: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(33, 1, false);
    pub const TMPCAB: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(34, 1, false);
    pub const TMPSNK: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(35, 1, false);
    pub const TMPTRNS: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(36, 1, false);
    pub const TMPSW: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(37, 1, false);
    pub const TMPOT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(38, 1, false);
    pub const WL1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(39, 1, false);
    pub const VAL1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(40, 1, false);
    pub const VARL1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(41, 1, false);
    pub const PFL1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(42, 1, false);
    pub const AL1: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(43, 1, false);
    pub const VL1L2: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(44, 1, false);
    pub const VL1: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(45, 1, false);
    pub const TOTWHINJL1: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(46, 4, false);
    pub const TOTWHABSL1: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(50, 4, false);
    pub const TOTVARHINJL1: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(54, 4, false);
    pub const TOTVARHABSL1: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(58, 4, false);
    pub const WL2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(62, 1, false);
    pub const VAL2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(63, 1, false);
    pub const VARL2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(64, 1, false);
    pub const PFL2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(65, 1, false);
    pub const AL2: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(66, 1, false);
    pub const VL2L3: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(67, 1, false);
    pub const VL2: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(68, 1, false);
    pub const TOTWHINJL2: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(69, 4, false);
    pub const TOTWHABSL2: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(73, 4, false);
    pub const TOTVARHINJL2: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(77, 4, false);
    pub const TOTVARHABSL2: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(81, 4, false);
    pub const WL3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(85, 1, false);
    pub const VAL3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(86, 1, false);
    pub const VARL3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(87, 1, false);
    pub const PFL3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(88, 1, false);
    pub const AL3: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(89, 1, false);
    pub const VL3L1: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(90, 1, false);
    pub const VL3: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(91, 1, false);
    pub const TOTWHINJL3: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(92, 4, false);
    pub const TOTWHABSL3: crate::PointDef<Self, Option<u64>> = crate::PointDef::new(96, 4, false);
    pub const TOTVARHINJL3: crate::PointDef<Self, Option<u64>> =
        crate::PointDef::new(100, 4, false);
    pub const TOTVARHABSL3: crate::PointDef<Self, Option<u64>> =
        crate::PointDef::new(104, 4, false);
    pub const THROTPCT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(108, 1, false);
    pub const THROTSRC: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(109, 2, false);
    pub const A_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(111, 1, false);
    pub const V_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(112, 1, false);
    pub const HZ_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(113, 1, false);
    pub const W_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(114, 1, false);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(115, 1, false);
    pub const VA_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(116, 1, false);
    pub const VAR_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(117, 1, false);
    pub const TOTWH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(118, 1, false);
    pub const TOTVARH_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(119, 1, false);
    pub const TMP_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(120, 1, false);
    pub const MNALRMINFO: crate::PointDef<Self, Option<String>> =
        crate::PointDef::new(121, 32, false);
}

impl crate::Model for Model701 {
    const ID: u16 = 701;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            actype: Self::ACTYPE.from_data(data)?,
            st: Self::ST.from_data(data)?,
            invst: Self::INVST.from_data(data)?,
            connst: Self::CONNST.from_data(data)?,
            alrm: Self::ALRM.from_data(data)?,
            dermode: Self::DERMODE.from_data(data)?,
            w: Self::W.from_data(data)?,
            va: Self::VA.from_data(data)?,
            var: Self::VAR.from_data(data)?,
            pf: Self::PF.from_data(data)?,
            a: Self::A.from_data(data)?,
            llv: Self::LLV.from_data(data)?,
            lnv: Self::LNV.from_data(data)?,
            hz: Self::HZ.from_data(data)?,
            totwhinj: Self::TOTWHINJ.from_data(data)?,
            totwhabs: Self::TOTWHABS.from_data(data)?,
            totvarhinj: Self::TOTVARHINJ.from_data(data)?,
            totvarhabs: Self::TOTVARHABS.from_data(data)?,
            tmpamb: Self::TMPAMB.from_data(data)?,
            tmpcab: Self::TMPCAB.from_data(data)?,
            tmpsnk: Self::TMPSNK.from_data(data)?,
            tmptrns: Self::TMPTRNS.from_data(data)?,
            tmpsw: Self::TMPSW.from_data(data)?,
            tmpot: Self::TMPOT.from_data(data)?,
            wl1: Self::WL1.from_data(data)?,
            val1: Self::VAL1.from_data(data)?,
            varl1: Self::VARL1.from_data(data)?,
            pfl1: Self::PFL1.from_data(data)?,
            al1: Self::AL1.from_data(data)?,
            vl1l2: Self::VL1L2.from_data(data)?,
            vl1: Self::VL1.from_data(data)?,
            totwhinjl1: Self::TOTWHINJL1.from_data(data)?,
            totwhabsl1: Self::TOTWHABSL1.from_data(data)?,
            totvarhinjl1: Self::TOTVARHINJL1.from_data(data)?,
            totvarhabsl1: Self::TOTVARHABSL1.from_data(data)?,
            wl2: Self::WL2.from_data(data)?,
            val2: Self::VAL2.from_data(data)?,
            varl2: Self::VARL2.from_data(data)?,
            pfl2: Self::PFL2.from_data(data)?,
            al2: Self::AL2.from_data(data)?,
            vl2l3: Self::VL2L3.from_data(data)?,
            vl2: Self::VL2.from_data(data)?,
            totwhinjl2: Self::TOTWHINJL2.from_data(data)?,
            totwhabsl2: Self::TOTWHABSL2.from_data(data)?,
            totvarhinjl2: Self::TOTVARHINJL2.from_data(data)?,
            totvarhabsl2: Self::TOTVARHABSL2.from_data(data)?,
            wl3: Self::WL3.from_data(data)?,
            val3: Self::VAL3.from_data(data)?,
            varl3: Self::VARL3.from_data(data)?,
            pfl3: Self::PFL3.from_data(data)?,
            al3: Self::AL3.from_data(data)?,
            vl3l1: Self::VL3L1.from_data(data)?,
            vl3: Self::VL3.from_data(data)?,
            totwhinjl3: Self::TOTWHINJL3.from_data(data)?,
            totwhabsl3: Self::TOTWHABSL3.from_data(data)?,
            totvarhinjl3: Self::TOTVARHINJL3.from_data(data)?,
            totvarhabsl3: Self::TOTVARHABSL3.from_data(data)?,
            throtpct: Self::THROTPCT.from_data(data)?,
            throtsrc: Self::THROTSRC.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            hz_sf: Self::HZ_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            va_sf: Self::VA_SF.from_data(data)?,
            var_sf: Self::VAR_SF.from_data(data)?,
            totwh_sf: Self::TOTWH_SF.from_data(data)?,
            totvarh_sf: Self::TOTVARH_SF.from_data(data)?,
            tmp_sf: Self::TMP_SF.from_data(data)?,
            mnalrminfo: Self::MNALRMINFO.from_data(data)?,
        })
    }
}
