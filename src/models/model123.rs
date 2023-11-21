/// Immediate Controls
///
/// Immediate Inverter Controls
///
/// Notes: Ref 3: 8.7.1.2, 8.7.2.2, 8.7.3.2
#[derive(Debug)]
pub struct Model123 {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    pub conn_wintms: Option<u16>,
    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    pub conn_rvrttms: Option<u16>,
    /// Conn
    ///
    /// Enumerated valued.  Connection control.
    pub conn: u16,
    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    pub wmaxlimpct: u16,
    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    pub wmaxlimpct_wintms: Option<u16>,
    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    pub wmaxlimpct_rvrttms: Option<u16>,
    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub wmaxlimpct_rmptms: Option<u16>,
    /// WMaxLim_Ena
    ///
    /// Enumerated valued.  Throttle enable/disable control.
    pub wmaxlim_ena: u16,
    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    pub outpfset: i16,
    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    pub outpfset_wintms: Option<u16>,
    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    pub outpfset_rvrttms: Option<u16>,
    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub outpfset_rmptms: Option<u16>,
    /// OutPFSet_Ena
    ///
    /// Enumerated valued.  Fixed power factor enable/disable control.
    pub outpfset_ena: u16,
    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    pub varwmaxpct: Option<i16>,
    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    pub varmaxpct: Option<i16>,
    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    pub varavalpct: Option<i16>,
    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    pub varpct_wintms: Option<u16>,
    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    pub varpct_rvrttms: Option<u16>,
    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub varpct_rmptms: Option<u16>,
    /// VArPct_Mod
    ///
    /// Enumerated value. VAR percent limit mode.
    pub varpct_mod: Option<u16>,
    /// VArPct_Ena
    ///
    /// Enumerated valued.  Percent limit VAr enable/disable control.
    pub varpct_ena: u16,
    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    pub wmaxlimpct_sf: i16,
    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    pub outpfset_sf: i16,
    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    pub varpct_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model123 {
    pub const CONN_WINTMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(0, 1, true);
    pub const CONN_RVRTTMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(1, 1, true);
    pub const CONN: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const WMAXLIMPCT: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const WMAXLIMPCT_WINTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(4, 1, true);
    pub const WMAXLIMPCT_RVRTTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(5, 1, true);
    pub const WMAXLIMPCT_RMPTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(6, 1, true);
    pub const WMAXLIM_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, true);
    pub const OUTPFSET: crate::PointDef<Self, i16> = crate::PointDef::new(8, 1, true);
    pub const OUTPFSET_WINTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(9, 1, true);
    pub const OUTPFSET_RVRTTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(10, 1, true);
    pub const OUTPFSET_RMPTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(11, 1, true);
    pub const OUTPFSET_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, true);
    pub const VARWMAXPCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(13, 1, true);
    pub const VARMAXPCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(14, 1, true);
    pub const VARAVALPCT: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(15, 1, true);
    pub const VARPCT_WINTMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, true);
    pub const VARPCT_RVRTTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(17, 1, true);
    pub const VARPCT_RMPTMS: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(18, 1, true);
    pub const VARPCT_MOD: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(19, 1, true);
    pub const VARPCT_ENA: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, true);
    pub const WMAXLIMPCT_SF: crate::PointDef<Self, i16> = crate::PointDef::new(21, 1, false);
    pub const OUTPFSET_SF: crate::PointDef<Self, i16> = crate::PointDef::new(22, 1, false);
    pub const VARPCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
}

impl crate::Model for Model123 {
    const ID: u16 = 123;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            conn_wintms: Self::CONN_WINTMS.from_data(data)?,
            conn_rvrttms: Self::CONN_RVRTTMS.from_data(data)?,
            conn: Self::CONN.from_data(data)?,
            wmaxlimpct: Self::WMAXLIMPCT.from_data(data)?,
            wmaxlimpct_wintms: Self::WMAXLIMPCT_WINTMS.from_data(data)?,
            wmaxlimpct_rvrttms: Self::WMAXLIMPCT_RVRTTMS.from_data(data)?,
            wmaxlimpct_rmptms: Self::WMAXLIMPCT_RMPTMS.from_data(data)?,
            wmaxlim_ena: Self::WMAXLIM_ENA.from_data(data)?,
            outpfset: Self::OUTPFSET.from_data(data)?,
            outpfset_wintms: Self::OUTPFSET_WINTMS.from_data(data)?,
            outpfset_rvrttms: Self::OUTPFSET_RVRTTMS.from_data(data)?,
            outpfset_rmptms: Self::OUTPFSET_RMPTMS.from_data(data)?,
            outpfset_ena: Self::OUTPFSET_ENA.from_data(data)?,
            varwmaxpct: Self::VARWMAXPCT.from_data(data)?,
            varmaxpct: Self::VARMAXPCT.from_data(data)?,
            varavalpct: Self::VARAVALPCT.from_data(data)?,
            varpct_wintms: Self::VARPCT_WINTMS.from_data(data)?,
            varpct_rvrttms: Self::VARPCT_RVRTTMS.from_data(data)?,
            varpct_rmptms: Self::VARPCT_RMPTMS.from_data(data)?,
            varpct_mod: Self::VARPCT_MOD.from_data(data)?,
            varpct_ena: Self::VARPCT_ENA.from_data(data)?,
            wmaxlimpct_sf: Self::WMAXLIMPCT_SF.from_data(data)?,
            outpfset_sf: Self::OUTPFSET_SF.from_data(data)?,
            varpct_sf: Self::VARPCT_SF.from_data(data)?,
        })
    }
}
