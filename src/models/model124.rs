/// Storage
///
/// Basic Storage Controls
///
/// Notes: Ref 3: 8.7.4.2
#[derive(Debug)]
pub struct Model124 {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    pub wchamax: u16,
    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    pub wchagra: u16,
    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    pub wdischagra: u16,
    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode. Bitfield value.
    pub storctl_mod: u16,
    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    pub vachamax: Option<u16>,
    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    pub minrsvpct: Option<u16>,
    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    pub chastate: Option<u16>,
    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    pub storaval: Option<u16>,
    /// InBatV
    ///
    /// Internal battery voltage.
    pub inbatv: Option<u16>,
    /// ChaSt
    ///
    /// Charge status of storage device. Enumerated value.
    pub chast: Option<u16>,
    /// OutWRte
    ///
    /// Percent of max discharge rate.
    pub outwrte: Option<i16>,
    /// InWRte
    ///
    /// Percent of max charging rate.
    pub inwrte: Option<i16>,
    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    pub inoutwrte_wintms: Option<u16>,
    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    pub inoutwrte_rvrttms: Option<u16>,
    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub inoutwrte_rmptms: Option<u16>,
    #[allow(missing_docs)]
    pub chagriset: Option<u16>,
    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    pub wchamax_sf: i16,
    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    pub wchadischagra_sf: i16,
    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    pub vachamax_sf: Option<i16>,
    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    pub minrsvpct_sf: Option<i16>,
    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    pub chastate_sf: Option<i16>,
    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    pub storaval_sf: Option<i16>,
    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    pub inbatv_sf: Option<i16>,
    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    pub inoutwrte_sf: Option<i16>,
}

#[allow(missing_docs)]

impl Model124 {
    pub const WCHAMAX: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, true);
    pub const WCHAGRA: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, true);
    pub const WDISCHAGRA: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, true);
    pub const STORCTL_MOD: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, true);
    pub const VACHAMAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, true);
    pub const MINRSVPCT: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, true);
    pub const CHASTATE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const STORAVAL: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, false);
    pub const INBATV: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, false);
    pub const CHAST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(9, 1, false);
    pub const OUTWRTE: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(10, 1, true);
    pub const INWRTE: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(11, 1, true);
    pub const INOUTWRTE_WINTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(12, 1, true);
    pub const INOUTWRTE_RVRTTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(13, 1, true);
    pub const INOUTWRTE_RMPTMS: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(14, 1, true);
    pub const CHAGRISET: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(15, 1, true);
    pub const WCHAMAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(16, 1, false);
    pub const WCHADISCHAGRA_SF: crate::PointDef<Self, i16> = crate::PointDef::new(17, 1, false);
    pub const VACHAMAX_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(18, 1, false);
    pub const MINRSVPCT_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(19, 1, false);
    pub const CHASTATE_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(20, 1, false);
    pub const STORAVAL_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(21, 1, false);
    pub const INBATV_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(22, 1, false);
    pub const INOUTWRTE_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(23, 1, false);
}

impl crate::Model for Model124 {
    const ID: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            wchamax: Self::WCHAMAX.from_data(data)?,
            wchagra: Self::WCHAGRA.from_data(data)?,
            wdischagra: Self::WDISCHAGRA.from_data(data)?,
            storctl_mod: Self::STORCTL_MOD.from_data(data)?,
            vachamax: Self::VACHAMAX.from_data(data)?,
            minrsvpct: Self::MINRSVPCT.from_data(data)?,
            chastate: Self::CHASTATE.from_data(data)?,
            storaval: Self::STORAVAL.from_data(data)?,
            inbatv: Self::INBATV.from_data(data)?,
            chast: Self::CHAST.from_data(data)?,
            outwrte: Self::OUTWRTE.from_data(data)?,
            inwrte: Self::INWRTE.from_data(data)?,
            inoutwrte_wintms: Self::INOUTWRTE_WINTMS.from_data(data)?,
            inoutwrte_rvrttms: Self::INOUTWRTE_RVRTTMS.from_data(data)?,
            inoutwrte_rmptms: Self::INOUTWRTE_RMPTMS.from_data(data)?,
            chagriset: Self::CHAGRISET.from_data(data)?,
            wchamax_sf: Self::WCHAMAX_SF.from_data(data)?,
            wchadischagra_sf: Self::WCHADISCHAGRA_SF.from_data(data)?,
            vachamax_sf: Self::VACHAMAX_SF.from_data(data)?,
            minrsvpct_sf: Self::MINRSVPCT_SF.from_data(data)?,
            chastate_sf: Self::CHASTATE_SF.from_data(data)?,
            storaval_sf: Self::STORAVAL_SF.from_data(data)?,
            inbatv_sf: Self::INBATV_SF.from_data(data)?,
            inoutwrte_sf: Self::INOUTWRTE_SF.from_data(data)?,
        })
    }
}
