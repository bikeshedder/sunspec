//! Storage
/// Storage
///
/// Basic Storage Controls
///
/// Notes: Ref 3: 8.7.4.2
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Model124 {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    pub w_cha_max: u16,
    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    pub w_cha_gra: u16,
    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    pub w_dis_cha_gra: u16,
    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode. Bitfield value.
    pub stor_ctl_mod: StorCtlMod,
    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    pub va_cha_max: Option<u16>,
    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    pub min_rsv_pct: Option<u16>,
    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    pub cha_state: Option<u16>,
    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    pub stor_aval: Option<u16>,
    /// InBatV
    ///
    /// Internal battery voltage.
    pub in_bat_v: Option<u16>,
    /// ChaSt
    ///
    /// Charge status of storage device. Enumerated value.
    pub cha_st: Option<ChaSt>,
    /// OutWRte
    ///
    /// Percent of max discharge rate.
    pub out_w_rte: Option<i16>,
    /// InWRte
    ///
    /// Percent of max charging rate.
    pub in_w_rte: Option<i16>,
    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    pub in_out_w_rte_win_tms: Option<u16>,
    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    pub in_out_w_rte_rvrt_tms: Option<u16>,
    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    pub in_out_w_rte_rmp_tms: Option<u16>,
    #[allow(missing_docs)]
    pub cha_gri_set: Option<ChaGriSet>,
    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    pub w_cha_max_sf: i16,
    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    pub w_cha_dis_cha_gra_sf: i16,
    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    pub va_cha_max_sf: Option<i16>,
    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    pub min_rsv_pct_sf: Option<i16>,
    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    pub cha_state_sf: Option<i16>,
    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    pub stor_aval_sf: Option<i16>,
    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    pub in_bat_v_sf: Option<i16>,
    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    pub in_out_w_rte_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Model124 {
    pub const W_CHA_MAX: crate::Point<Self, u16> = crate::Point::new(0, 1, true);
    pub const W_CHA_GRA: crate::Point<Self, u16> = crate::Point::new(1, 1, true);
    pub const W_DIS_CHA_GRA: crate::Point<Self, u16> = crate::Point::new(2, 1, true);
    pub const STOR_CTL_MOD: crate::Point<Self, StorCtlMod> = crate::Point::new(3, 1, true);
    pub const VA_CHA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, true);
    pub const MIN_RSV_PCT: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, true);
    pub const CHA_STATE: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const STOR_AVAL: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, false);
    pub const IN_BAT_V: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, false);
    pub const CHA_ST: crate::Point<Self, Option<ChaSt>> = crate::Point::new(9, 1, false);
    pub const OUT_W_RTE: crate::Point<Self, Option<i16>> = crate::Point::new(10, 1, true);
    pub const IN_W_RTE: crate::Point<Self, Option<i16>> = crate::Point::new(11, 1, true);
    pub const IN_OUT_W_RTE_WIN_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(12, 1, true);
    pub const IN_OUT_W_RTE_RVRT_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(13, 1, true);
    pub const IN_OUT_W_RTE_RMP_TMS: crate::Point<Self, Option<u16>> =
        crate::Point::new(14, 1, true);
    pub const CHA_GRI_SET: crate::Point<Self, Option<ChaGriSet>> = crate::Point::new(15, 1, true);
    pub const W_CHA_MAX_SF: crate::Point<Self, i16> = crate::Point::new(16, 1, false);
    pub const W_CHA_DIS_CHA_GRA_SF: crate::Point<Self, i16> = crate::Point::new(17, 1, false);
    pub const VA_CHA_MAX_SF: crate::Point<Self, Option<i16>> = crate::Point::new(18, 1, false);
    pub const MIN_RSV_PCT_SF: crate::Point<Self, Option<i16>> = crate::Point::new(19, 1, false);
    pub const CHA_STATE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(20, 1, false);
    pub const STOR_AVAL_SF: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, false);
    pub const IN_BAT_V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, false);
    pub const IN_OUT_W_RTE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
}
impl crate::Model for Model124 {
    const ID: u16 = 124;
    fn from_data(data: &[u16]) -> Result<Self, crate::DecodeError> {
        Ok(Self {
            w_cha_max: Self::W_CHA_MAX.from_data(data)?,
            w_cha_gra: Self::W_CHA_GRA.from_data(data)?,
            w_dis_cha_gra: Self::W_DIS_CHA_GRA.from_data(data)?,
            stor_ctl_mod: Self::STOR_CTL_MOD.from_data(data)?,
            va_cha_max: Self::VA_CHA_MAX.from_data(data)?,
            min_rsv_pct: Self::MIN_RSV_PCT.from_data(data)?,
            cha_state: Self::CHA_STATE.from_data(data)?,
            stor_aval: Self::STOR_AVAL.from_data(data)?,
            in_bat_v: Self::IN_BAT_V.from_data(data)?,
            cha_st: Self::CHA_ST.from_data(data)?,
            out_w_rte: Self::OUT_W_RTE.from_data(data)?,
            in_w_rte: Self::IN_W_RTE.from_data(data)?,
            in_out_w_rte_win_tms: Self::IN_OUT_W_RTE_WIN_TMS.from_data(data)?,
            in_out_w_rte_rvrt_tms: Self::IN_OUT_W_RTE_RVRT_TMS.from_data(data)?,
            in_out_w_rte_rmp_tms: Self::IN_OUT_W_RTE_RMP_TMS.from_data(data)?,
            cha_gri_set: Self::CHA_GRI_SET.from_data(data)?,
            w_cha_max_sf: Self::W_CHA_MAX_SF.from_data(data)?,
            w_cha_dis_cha_gra_sf: Self::W_CHA_DIS_CHA_GRA_SF.from_data(data)?,
            va_cha_max_sf: Self::VA_CHA_MAX_SF.from_data(data)?,
            min_rsv_pct_sf: Self::MIN_RSV_PCT_SF.from_data(data)?,
            cha_state_sf: Self::CHA_STATE_SF.from_data(data)?,
            stor_aval_sf: Self::STOR_AVAL_SF.from_data(data)?,
            in_bat_v_sf: Self::IN_BAT_V_SF.from_data(data)?,
            in_out_w_rte_sf: Self::IN_OUT_W_RTE_SF.from_data(data)?,
        })
    }
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m124
    }
}
bitflags::bitflags! {
    #[doc = " StorCtl_Mod"] #[doc = " "] #[doc =
    " Activate hold/discharge/charge storage control mode. Bitfield value."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct StorCtlMod : u16 {
    #[allow(missing_docs)] const Charge = 1; #[allow(missing_docs)] const DiScharge = 2;
    }
}
impl crate::Value for StorCtlMod {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::Value for Option<StorCtlMod> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535u16 {
            Ok(Some(StorCtlMod::from_bits_retain(value)))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535u16.encode()
        }
    }
}
/// ChaSt
///
/// Charge status of storage device. Enumerated value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ChaSt {
    #[allow(missing_docs)]
    Off = 1,
    #[allow(missing_docs)]
    Empty = 2,
    #[allow(missing_docs)]
    Discharging = 3,
    #[allow(missing_docs)]
    Charging = 4,
    #[allow(missing_docs)]
    Full = 5,
    #[allow(missing_docs)]
    Holding = 6,
    #[allow(missing_docs)]
    Testing = 7,
}
impl crate::Value for ChaSt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ChaSt> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ChaSt::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ChaGriSet {
    #[allow(missing_docs)]
    Pv = 0,
    #[allow(missing_docs)]
    Grid = 1,
}
impl crate::Value for ChaGriSet {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ChaGriSet> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ChaGriSet::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
            ))
        } else {
            Ok(None)
        }
    }
    fn encode(self) -> Box<[u16]> {
        if let Some(value) = self {
            value.encode()
        } else {
            65535.encode()
        }
    }
}
