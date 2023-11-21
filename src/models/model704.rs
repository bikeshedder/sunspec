/// DER AC Controls
///
/// DER AC controls model.
#[derive(Debug)]
pub struct Model704 {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    ///
    /// Comments: Set Power Factor (when injecting active power)
    pub pfwinjena: Option<u16>,
    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    pub pfwinjenarvrt: Option<u16>,
    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    pub pfwinjrvrttms: Option<u32>,
    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    pub pfwinjrvrtrem: Option<u32>,
    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    ///
    /// Comments: Set Power Factor (when absorbing active power)
    pub pfwabsena: Option<u16>,
    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    pub pfwabsenarvrt: Option<u16>,
    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    pub pfwabsrvrttms: Option<u32>,
    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    pub pfwabsrvrtrem: Option<u32>,
    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    ///
    /// Comments: Limit Maximum Active Power Generation
    pub wmaxlimpctena: Option<u16>,
    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    pub wmaxlimpct: Option<u16>,
    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    pub wmaxlimpctrvrt: Option<u16>,
    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    pub wmaxlimpctenarvrt: Option<u16>,
    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    pub wmaxlimpctrvrttms: Option<u32>,
    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    pub wmaxlimpctrvrtrem: Option<u32>,
    /// Set Active Power Enable
    ///
    /// Set active power enable.
    ///
    /// Comments: Set Active Power Level (may be negative for charging)
    pub wsetena: Option<u16>,
    /// Set Active Power Mode
    ///
    /// Set active power mode.
    pub wsetmod: Option<u16>,
    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    pub wset: Option<i32>,
    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    pub wsetrvrt: Option<i32>,
    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    pub wsetpct: Option<i16>,
    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    pub wsetpctrvrt: Option<i16>,
    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    pub wsetenarvrt: Option<u16>,
    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    pub wsetrvrttms: Option<u32>,
    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    pub wsetrvrtrem: Option<u32>,
    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    ///
    /// Comments: Set Reactive Power Level
    pub varsetena: Option<u16>,
    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    pub varsetmod: Option<u16>,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    pub varsetpri: Option<u16>,
    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    pub varset: Option<i32>,
    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    pub varsetrvrt: Option<i32>,
    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    pub varsetpct: Option<i16>,
    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    pub varsetpctrvrt: Option<i16>,
    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    pub varsetenarvrt: Option<u16>,
    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    pub varsetrvrttms: Option<u32>,
    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    pub varsetrvrtrem: Option<u32>,
    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    ///
    /// Comments: Ramp Rate
    pub wrmp: Option<u16>,
    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    pub wrmpref: Option<u16>,
    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    pub varrmp: Option<u16>,
    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    pub antiislena: Option<u16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    ///
    /// Comments: Scale Factors
    pub pf_sf: Option<i16>,
    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    pub wmaxlimpct_sf: Option<i16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    pub wset_sf: Option<i16>,
    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    pub wsetpct_sf: Option<i16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    pub varset_sf: Option<i16>,
    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    pub varsetpct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model704 {
    pub const PFWINJENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const PFWINJENARVRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const PFWINJRVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(2, 2, true);
    pub const PFWINJRVRTREM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(4, 2, false);
    pub const PFWABSENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, true);
    pub const PFWABSENARVRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, true);
    pub const PFWABSRVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(8, 2, true);
    pub const PFWABSRVRTREM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(10, 2, false);
    pub const WMAXLIMPCTENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(12, 1, true);
    pub const WMAXLIMPCT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(13, 1, true);
    pub const WMAXLIMPCTRVRT: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(14, 1, true);
    pub const WMAXLIMPCTENARVRT: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(15, 1, true);
    pub const WMAXLIMPCTRVRTTMS: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(16, 2, true);
    pub const WMAXLIMPCTRVRTREM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(18, 2, false);
    pub const WSETENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(20, 1, true);
    pub const WSETMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(21, 1, true);
    pub const WSET: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(22, 2, true);
    pub const WSETRVRT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(24, 2, true);
    pub const WSETPCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(26, 1, true);
    pub const WSETPCTRVRT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(27, 1, true);
    pub const WSETENARVRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(28, 1, true);
    pub const WSETRVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(29, 2, true);
    pub const WSETRVRTREM: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(31, 2, false);
    pub const VARSETENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(33, 1, true);
    pub const VARSETMOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(34, 1, true);
    pub const VARSETPRI: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(35, 1, true);
    pub const VARSET: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(36, 2, true);
    pub const VARSETRVRT: crate::PointDef<Self, Option<i32>> = crate::PointDef::new(38, 2, true);
    pub const VARSETPCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(40, 1, true);
    pub const VARSETPCTRVRT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(41, 1, true);
    pub const VARSETENARVRT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(42, 1, true);
    pub const VARSETRVRTTMS: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(43, 2, true);
    pub const VARSETRVRTREM: crate::PointDef<Self, Option<u32>> =
        crate::PointDef::new(45, 2, false);
    pub const WRMP: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(47, 1, true);
    pub const WRMPREF: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(48, 1, true);
    pub const VARRMP: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(49, 1, true);
    pub const ANTIISLENA: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(50, 1, true);
    pub const PF_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(51, 1, false);
    pub const WMAXLIMPCT_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(52, 1, false);
    pub const WSET_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(53, 1, false);
    pub const WSETPCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(54, 1, false);
    pub const VARSET_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(55, 1, false);
    pub const VARSETPCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(56, 1, false);
}

impl crate::Model for Model704 {
    const ID: u16 = 704;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            pfwinjena: Self::PFWINJENA.from_data(data)?,
            pfwinjenarvrt: Self::PFWINJENARVRT.from_data(data)?,
            pfwinjrvrttms: Self::PFWINJRVRTTMS.from_data(data)?,
            pfwinjrvrtrem: Self::PFWINJRVRTREM.from_data(data)?,
            pfwabsena: Self::PFWABSENA.from_data(data)?,
            pfwabsenarvrt: Self::PFWABSENARVRT.from_data(data)?,
            pfwabsrvrttms: Self::PFWABSRVRTTMS.from_data(data)?,
            pfwabsrvrtrem: Self::PFWABSRVRTREM.from_data(data)?,
            wmaxlimpctena: Self::WMAXLIMPCTENA.from_data(data)?,
            wmaxlimpct: Self::WMAXLIMPCT.from_data(data)?,
            wmaxlimpctrvrt: Self::WMAXLIMPCTRVRT.from_data(data)?,
            wmaxlimpctenarvrt: Self::WMAXLIMPCTENARVRT.from_data(data)?,
            wmaxlimpctrvrttms: Self::WMAXLIMPCTRVRTTMS.from_data(data)?,
            wmaxlimpctrvrtrem: Self::WMAXLIMPCTRVRTREM.from_data(data)?,
            wsetena: Self::WSETENA.from_data(data)?,
            wsetmod: Self::WSETMOD.from_data(data)?,
            wset: Self::WSET.from_data(data)?,
            wsetrvrt: Self::WSETRVRT.from_data(data)?,
            wsetpct: Self::WSETPCT.from_data(data)?,
            wsetpctrvrt: Self::WSETPCTRVRT.from_data(data)?,
            wsetenarvrt: Self::WSETENARVRT.from_data(data)?,
            wsetrvrttms: Self::WSETRVRTTMS.from_data(data)?,
            wsetrvrtrem: Self::WSETRVRTREM.from_data(data)?,
            varsetena: Self::VARSETENA.from_data(data)?,
            varsetmod: Self::VARSETMOD.from_data(data)?,
            varsetpri: Self::VARSETPRI.from_data(data)?,
            varset: Self::VARSET.from_data(data)?,
            varsetrvrt: Self::VARSETRVRT.from_data(data)?,
            varsetpct: Self::VARSETPCT.from_data(data)?,
            varsetpctrvrt: Self::VARSETPCTRVRT.from_data(data)?,
            varsetenarvrt: Self::VARSETENARVRT.from_data(data)?,
            varsetrvrttms: Self::VARSETRVRTTMS.from_data(data)?,
            varsetrvrtrem: Self::VARSETRVRTREM.from_data(data)?,
            wrmp: Self::WRMP.from_data(data)?,
            wrmpref: Self::WRMPREF.from_data(data)?,
            varrmp: Self::VARRMP.from_data(data)?,
            antiislena: Self::ANTIISLENA.from_data(data)?,
            pf_sf: Self::PF_SF.from_data(data)?,
            wmaxlimpct_sf: Self::WMAXLIMPCT_SF.from_data(data)?,
            wset_sf: Self::WSET_SF.from_data(data)?,
            wsetpct_sf: Self::WSETPCT_SF.from_data(data)?,
            varset_sf: Self::VARSET_SF.from_data(data)?,
            varsetpct_sf: Self::VARSETPCT_SF.from_data(data)?,
        })
    }
}
