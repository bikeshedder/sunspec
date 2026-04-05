//! Battery Base Model
/// Type alias for [`Battery`].
pub type Model802 = Battery;
/// Battery Base Model
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Battery {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    pub ah_rtg: u16,
    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    pub wh_rtg: u16,
    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    pub w_cha_rte_max: u16,
    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    pub w_dis_cha_rte_max: u16,
    /// Self Discharge Rate
    ///
    /// Self discharge rate.  Percentage of capacity (WHRtg) discharged per day.
    pub dis_cha_rte: Option<u16>,
    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    pub soc_max: Option<u16>,
    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    pub soc_min: Option<u16>,
    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    pub soc_rsv_max: Option<u16>,
    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    pub soc_rsv_min: Option<u16>,
    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Detail: Measurement.
    pub soc: u16,
    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Detail: Measurement.
    pub do_d: Option<u16>,
    /// State of Health
    ///
    /// Percentage of battery life remaining.
    pub soh: Option<u16>,
    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    pub n_cyc: Option<u32>,
    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    pub cha_st: Option<ChaSt>,
    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Detail: Maps to DRCC.LocRemCtl in IEC 61850.
    pub loc_rem_ctl: LocRemCtl,
    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    pub hb: Option<u16>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    pub ctrl_hb: Option<u16>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms.  1 = Reset.
    ///
    /// Detail: Battery should reset to 0 when reset is complete.
    pub alm_rst: u16,
    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Detail: Maps to DBAT.BatTyp in 61850.
    pub typ: Typ,
    /// State of the Battery Bank
    ///
    /// State of the battery bank.  Enumeration.
    ///
    /// Detail: Must be reconciled with State in IEC 61850.
    pub state: State,
    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state.  Enumeration.
    pub state_vnd: Option<u16>,
    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Detail: Number of days since 1/1/2000.
    pub warr_dt: Option<u32>,
    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    pub evt1: Evt1,
    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    ///
    /// Detail: Reserved for future use.
    pub evt2: Evt2,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evt_vnd1: EvtVnd1,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evt_vnd2: EvtVnd2,
    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Detail: Maps to ZBAT.V in IEC 61850.
    pub v: u16,
    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// Detail: If not implemented, must implement AChaMax and ADisChaMax.
    pub v_max: Option<u16>,
    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// Detail: If not implemented, must implement AChaMax and ADisChaMax.
    pub v_min: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Detail: Measurement.
    pub cell_v_max: Option<u16>,
    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    pub cell_v_max_str: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    pub cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Detail: Measurement.
    pub cell_v_min: Option<u16>,
    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    pub cell_v_min_str: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    pub cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Detail: Calculation based on measurements.
    pub cell_v_avg: Option<u16>,
    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Detail: Measurement.
    pub a: i16,
    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Detail: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub a_cha_max: Option<u16>,
    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Detail: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub a_dis_cha_max: Option<u16>,
    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// Detail: DC Measurement.
    pub w: i16,
    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter.  Enumeration.
    ///
    /// Detail: Used in special states such as manual battery charging.
    pub req_inv_state: Option<ReqInvState>,
    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Detail: Used in special states such as string balancing.
    pub req_w: Option<i16>,
    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting.  Enumeration.
    pub set_op: SetOp,
    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Detail: Information needed by battery for some operations.
    pub set_inv_state: SetInvState,
    /// Scale factor for charge capacity.
    pub ah_rtg_sf: i16,
    /// Scale factor for energy capacity.
    pub wh_rtg_sf: i16,
    /// Scale factor for maximum charge and discharge rate.
    pub w_cha_dis_cha_max_sf: i16,
    /// Scale factor for self discharge rate.
    pub dis_cha_rte_sf: Option<i16>,
    /// Scale factor for state of charge values.
    pub soc_sf: i16,
    /// Scale factor for depth of discharge.
    pub do_d_sf: Option<i16>,
    /// Scale factor for state of health.
    pub soh_sf: Option<i16>,
    /// Scale factor for DC bus voltage.
    pub v_sf: i16,
    /// Scale factor for cell voltage.
    pub cell_v_sf: i16,
    /// Scale factor for DC current.
    pub a_sf: i16,
    /// Scale factor for instantaneous DC charge/discharge current.
    pub a_max_sf: i16,
    /// Scale factor for AC power request.
    pub w_sf: Option<i16>,
}
#[allow(missing_docs)]
impl Battery {
    pub const AH_RTG: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const WH_RTG: crate::Point<Self, u16> = crate::Point::new(1, 1, false);
    pub const W_CHA_RTE_MAX: crate::Point<Self, u16> = crate::Point::new(2, 1, false);
    pub const W_DIS_CHA_RTE_MAX: crate::Point<Self, u16> = crate::Point::new(3, 1, false);
    pub const DIS_CHA_RTE: crate::Point<Self, Option<u16>> = crate::Point::new(4, 1, false);
    pub const SOC_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(5, 1, false);
    pub const SOC_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(6, 1, false);
    pub const SOC_RSV_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(7, 1, true);
    pub const SOC_RSV_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(8, 1, true);
    pub const SOC: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const DO_D: crate::Point<Self, Option<u16>> = crate::Point::new(10, 1, false);
    pub const SOH: crate::Point<Self, Option<u16>> = crate::Point::new(11, 1, false);
    pub const N_CYC: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
    pub const CHA_ST: crate::Point<Self, Option<ChaSt>> = crate::Point::new(14, 1, false);
    pub const LOC_REM_CTL: crate::Point<Self, LocRemCtl> = crate::Point::new(15, 1, false);
    pub const HB: crate::Point<Self, Option<u16>> = crate::Point::new(16, 1, false);
    pub const CTRL_HB: crate::Point<Self, Option<u16>> = crate::Point::new(17, 1, true);
    pub const ALM_RST: crate::Point<Self, u16> = crate::Point::new(18, 1, true);
    pub const TYP: crate::Point<Self, Typ> = crate::Point::new(19, 1, false);
    pub const STATE: crate::Point<Self, State> = crate::Point::new(20, 1, false);
    pub const STATE_VND: crate::Point<Self, Option<u16>> = crate::Point::new(21, 1, false);
    pub const WARR_DT: crate::Point<Self, Option<u32>> = crate::Point::new(22, 2, false);
    pub const EVT1: crate::Point<Self, Evt1> = crate::Point::new(24, 2, false);
    pub const EVT2: crate::Point<Self, Evt2> = crate::Point::new(26, 2, false);
    pub const EVT_VND1: crate::Point<Self, EvtVnd1> = crate::Point::new(28, 2, false);
    pub const EVT_VND2: crate::Point<Self, EvtVnd2> = crate::Point::new(30, 2, false);
    pub const V: crate::Point<Self, u16> = crate::Point::new(32, 1, false);
    pub const V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(33, 1, false);
    pub const V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(34, 1, false);
    pub const CELL_V_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(35, 1, false);
    pub const CELL_V_MAX_STR: crate::Point<Self, Option<u16>> = crate::Point::new(36, 1, false);
    pub const CELL_V_MAX_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(37, 1, false);
    pub const CELL_V_MIN: crate::Point<Self, Option<u16>> = crate::Point::new(38, 1, false);
    pub const CELL_V_MIN_STR: crate::Point<Self, Option<u16>> = crate::Point::new(39, 1, false);
    pub const CELL_V_MIN_MOD: crate::Point<Self, Option<u16>> = crate::Point::new(40, 1, false);
    pub const CELL_V_AVG: crate::Point<Self, Option<u16>> = crate::Point::new(41, 1, false);
    pub const A: crate::Point<Self, i16> = crate::Point::new(42, 1, false);
    pub const A_CHA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(43, 1, false);
    pub const A_DIS_CHA_MAX: crate::Point<Self, Option<u16>> = crate::Point::new(44, 1, false);
    pub const W: crate::Point<Self, i16> = crate::Point::new(45, 1, false);
    pub const REQ_INV_STATE: crate::Point<Self, Option<ReqInvState>> =
        crate::Point::new(46, 1, false);
    pub const REQ_W: crate::Point<Self, Option<i16>> = crate::Point::new(47, 1, false);
    pub const SET_OP: crate::Point<Self, SetOp> = crate::Point::new(48, 1, true);
    pub const SET_INV_STATE: crate::Point<Self, SetInvState> = crate::Point::new(49, 1, true);
    pub const AH_RTG_SF: crate::Point<Self, i16> = crate::Point::new(50, 1, false);
    pub const WH_RTG_SF: crate::Point<Self, i16> = crate::Point::new(51, 1, false);
    pub const W_CHA_DIS_CHA_MAX_SF: crate::Point<Self, i16> = crate::Point::new(52, 1, false);
    pub const DIS_CHA_RTE_SF: crate::Point<Self, Option<i16>> = crate::Point::new(53, 1, false);
    pub const SOC_SF: crate::Point<Self, i16> = crate::Point::new(54, 1, false);
    pub const DO_D_SF: crate::Point<Self, Option<i16>> = crate::Point::new(55, 1, false);
    pub const SOH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(56, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(57, 1, false);
    pub const CELL_V_SF: crate::Point<Self, i16> = crate::Point::new(58, 1, false);
    pub const A_SF: crate::Point<Self, i16> = crate::Point::new(59, 1, false);
    pub const A_MAX_SF: crate::Point<Self, i16> = crate::Point::new(60, 1, false);
    pub const W_SF: crate::Point<Self, Option<i16>> = crate::Point::new(61, 1, false);
}
static BATTERY_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "ah_rtg",
        label: "Nameplate Charge Capacity",
        description: "Nameplate charge capacity in amp-hours.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wh_rtg",
        label: "Nameplate Energy Capacity",
        description: "Nameplate energy capacity in DC watt-hours.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_cha_rte_max",
        label: "Nameplate Max Charge Rate",
        description: "Maximum rate of energy transfer into the storage device in DC watts.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_dis_cha_rte_max",
        label: "Nameplate Max Discharge Rate",
        description: "Maximum rate of energy transfer out of the storage device in DC watts.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dis_cha_rte",
        label: "Self Discharge Rate",
        description: "Self discharge rate.  Percentage of capacity (WHRtg) discharged per day.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_max",
        label: "Nameplate Max SoC",
        description: "Manufacturer maximum state of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_min",
        label: "Nameplate Min SoC",
        description: "Manufacturer minimum state of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_rsv_max",
        label: "Max Reserve Percent",
        description: "Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_rsv_min",
        label: "Min Reserve Percent",
        description: "Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc",
        label: "State of Charge",
        description: "State of charge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "do_d",
        label: "Depth of Discharge",
        description: "Depth of discharge, expressed as a percentage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soh",
        label: "State of Health",
        description: "Percentage of battery life remaining.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "n_cyc",
        label: "Cycle Count",
        description: "Number of cycles executed in the battery.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cha_st",
        label: "Charge Status",
        description: "Charge status of storage device. Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "loc_rem_ctl",
        label: "Control Mode",
        description: "Battery control mode. Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "hb",
        label: "Battery Heartbeat",
        description: "Value is incremented every second with periodic resets to zero.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ctrl_hb",
        label: "Controller Heartbeat",
        description: "Value is incremented every second with periodic resets to zero.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "alm_rst",
        label: "Alarm Reset",
        description: "Used to reset any latched alarms.  1 = Reset.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "typ",
        label: "Battery Type",
        description: "Type of battery. Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "state",
        label: "State of the Battery Bank",
        description: "State of the battery bank.  Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "state_vnd",
        label: "Vendor Battery Bank State",
        description: "Vendor specific battery bank state.  Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "warr_dt",
        label: "Warranty Date",
        description: "Date the device warranty expires.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt1",
        label: "Battery Event 1 Bitfield",
        description: "Alarms and warnings.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt2",
        label: "Battery Event 2 Bitfield",
        description: "Alarms and warnings.  Bit flags.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd1",
        label: "Vendor Event Bitfield 1",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "evt_vnd2",
        label: "Vendor Event Bitfield 2",
        description: "Vendor defined events.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v",
        label: "External Battery Voltage",
        description: "DC Bus Voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_max",
        label: "Max Battery Voltage",
        description: "Instantaneous maximum battery voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_min",
        label: "Min Battery Voltage",
        description: "Instantaneous minimum battery voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_max",
        label: "Max Cell Voltage",
        description: "Maximum voltage for all cells in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_max_str",
        label: "Max Cell Voltage String",
        description: "String containing the cell with maximum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_max_mod",
        label: "Max Cell Voltage Module",
        description: "Module containing the cell with maximum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_min",
        label: "Min Cell Voltage",
        description: "Minimum voltage for all cells in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_min_str",
        label: "Min Cell Voltage String",
        description: "String containing the cell with minimum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_min_mod",
        label: "Min Cell Voltage Module",
        description: "Module containing the cell with minimum voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_avg",
        label: "Average Cell Voltage",
        description: "Average cell voltage for all cells in the bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a",
        label: "Total DC Current",
        description: "Total DC current flowing to/from the battery bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_cha_max",
        label: "Max Charge Current",
        description: "Instantaneous maximum DC charge current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_dis_cha_max",
        label: "Max Discharge Current",
        description: "Instantaneous maximum DC discharge current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w",
        label: "Total Power",
        description: "Total power flowing to/from the battery bank.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "req_inv_state",
        label: "Inverter State Request",
        description: "Request from battery to start or stop the inverter.  Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "req_w",
        label: "Battery Power Request",
        description: "AC Power requested by battery.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "set_op",
        label: "Set Operation",
        description: "Instruct the battery bank to perform an operation such as connecting.  Enumeration.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "set_inv_state",
        label: "Set Inverter State",
        description: "Set the current state of the inverter.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "ah_rtg_sf",
        label: "AHRtg_SF",
        description: "Scale factor for charge capacity.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "wh_rtg_sf",
        label: "WHRtg_SF",
        description: "Scale factor for energy capacity.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_cha_dis_cha_max_sf",
        label: "WChaDisChaMax_SF",
        description: "Scale factor for maximum charge and discharge rate.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "dis_cha_rte_sf",
        label: "DisChaRte_SF",
        description: "Scale factor for self discharge rate.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soc_sf",
        label: "SoC_SF",
        description: "Scale factor for state of charge values.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "do_d_sf",
        label: "DoD_SF",
        description: "Scale factor for depth of discharge.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "soh_sf",
        label: "SoH_SF",
        description: "Scale factor for state of health.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "v_sf",
        label: "V_SF",
        description: "Scale factor for DC bus voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "cell_v_sf",
        label: "CellV_SF",
        description: "Scale factor for cell voltage.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_sf",
        label: "A_SF",
        description: "Scale factor for DC current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "a_max_sf",
        label: "AMax_SF",
        description: "Scale factor for instantaneous DC charge/discharge current.",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "w_sf",
        label: "W_SF",
        description: "Scale factor for AC power request.",
        kind: crate::FieldKind::Point,
    },
];
static BATTERY_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "battery",
    label: "Battery Base Model",
    description: "",
    fields: BATTERY_FIELDS,
};
impl crate::GroupMeta for Battery {
    fn group_info() -> &'static crate::GroupInfo {
        &BATTERY_GROUP_INFO
    }
}
impl crate::Group for Battery {
    const LEN: u16 = 62;
}
impl Battery {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                ah_rtg: Self::AH_RTG.from_data(data)?,
                wh_rtg: Self::WH_RTG.from_data(data)?,
                w_cha_rte_max: Self::W_CHA_RTE_MAX.from_data(data)?,
                w_dis_cha_rte_max: Self::W_DIS_CHA_RTE_MAX.from_data(data)?,
                dis_cha_rte: Self::DIS_CHA_RTE.from_data(data)?,
                soc_max: Self::SOC_MAX.from_data(data)?,
                soc_min: Self::SOC_MIN.from_data(data)?,
                soc_rsv_max: Self::SOC_RSV_MAX.from_data(data)?,
                soc_rsv_min: Self::SOC_RSV_MIN.from_data(data)?,
                soc: Self::SOC.from_data(data)?,
                do_d: Self::DO_D.from_data(data)?,
                soh: Self::SOH.from_data(data)?,
                n_cyc: Self::N_CYC.from_data(data)?,
                cha_st: Self::CHA_ST.from_data(data)?,
                loc_rem_ctl: Self::LOC_REM_CTL.from_data(data)?,
                hb: Self::HB.from_data(data)?,
                ctrl_hb: Self::CTRL_HB.from_data(data)?,
                alm_rst: Self::ALM_RST.from_data(data)?,
                typ: Self::TYP.from_data(data)?,
                state: Self::STATE.from_data(data)?,
                state_vnd: Self::STATE_VND.from_data(data)?,
                warr_dt: Self::WARR_DT.from_data(data)?,
                evt1: Self::EVT1.from_data(data)?,
                evt2: Self::EVT2.from_data(data)?,
                evt_vnd1: Self::EVT_VND1.from_data(data)?,
                evt_vnd2: Self::EVT_VND2.from_data(data)?,
                v: Self::V.from_data(data)?,
                v_max: Self::V_MAX.from_data(data)?,
                v_min: Self::V_MIN.from_data(data)?,
                cell_v_max: Self::CELL_V_MAX.from_data(data)?,
                cell_v_max_str: Self::CELL_V_MAX_STR.from_data(data)?,
                cell_v_max_mod: Self::CELL_V_MAX_MOD.from_data(data)?,
                cell_v_min: Self::CELL_V_MIN.from_data(data)?,
                cell_v_min_str: Self::CELL_V_MIN_STR.from_data(data)?,
                cell_v_min_mod: Self::CELL_V_MIN_MOD.from_data(data)?,
                cell_v_avg: Self::CELL_V_AVG.from_data(data)?,
                a: Self::A.from_data(data)?,
                a_cha_max: Self::A_CHA_MAX.from_data(data)?,
                a_dis_cha_max: Self::A_DIS_CHA_MAX.from_data(data)?,
                w: Self::W.from_data(data)?,
                req_inv_state: Self::REQ_INV_STATE.from_data(data)?,
                req_w: Self::REQ_W.from_data(data)?,
                set_op: Self::SET_OP.from_data(data)?,
                set_inv_state: Self::SET_INV_STATE.from_data(data)?,
                ah_rtg_sf: Self::AH_RTG_SF.from_data(data)?,
                wh_rtg_sf: Self::WH_RTG_SF.from_data(data)?,
                w_cha_dis_cha_max_sf: Self::W_CHA_DIS_CHA_MAX_SF.from_data(data)?,
                dis_cha_rte_sf: Self::DIS_CHA_RTE_SF.from_data(data)?,
                soc_sf: Self::SOC_SF.from_data(data)?,
                do_d_sf: Self::DO_D_SF.from_data(data)?,
                soh_sf: Self::SOH_SF.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                cell_v_sf: Self::CELL_V_SF.from_data(data)?,
                a_sf: Self::A_SF.from_data(data)?,
                a_max_sf: Self::A_MAX_SF.from_data(data)?,
                w_sf: Self::W_SF.from_data(data)?,
            },
        ))
    }
}
/// Charge Status
///
/// Charge status of storage device. Enumeration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ChaSt {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Empty,
    #[allow(missing_docs)]
    Discharging,
    #[allow(missing_docs)]
    Charging,
    #[allow(missing_docs)]
    Full,
    #[allow(missing_docs)]
    Holding,
    #[allow(missing_docs)]
    Testing,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ChaSt {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Off,
            2 => Self::Empty,
            3 => Self::Discharging,
            4 => Self::Charging,
            5 => Self::Full,
            6 => Self::Holding,
            7 => Self::Testing,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 1,
            Self::Empty => 2,
            Self::Discharging => 3,
            Self::Charging => 4,
            Self::Full => 5,
            Self::Holding => 6,
            Self::Testing => 7,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ChaSt {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Control Mode
///
/// Battery control mode. Enumeration.
///
/// Detail: Maps to DRCC.LocRemCtl in IEC 61850.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum LocRemCtl {
    /// Detail: Value of 0 matches LocRemCtl in IEC 61850.
    Remote,
    /// Detail: Value of 1 matches LocRemCtl in IEC 61850.
    Local,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for LocRemCtl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Remote,
            1 => Self::Local,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Remote => 0,
            Self::Local => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for LocRemCtl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Battery Type
///
/// Type of battery. Enumeration.
///
/// Detail: Maps to DBAT.BatTyp in 61850.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Typ {
    #[allow(missing_docs)]
    NotApplicableUnknown,
    #[allow(missing_docs)]
    LeadAcid,
    #[allow(missing_docs)]
    NickelMetalHydrate,
    #[allow(missing_docs)]
    NickelCadmium,
    #[allow(missing_docs)]
    LithiumIon,
    #[allow(missing_docs)]
    CarbonZinc,
    #[allow(missing_docs)]
    ZincChloride,
    #[allow(missing_docs)]
    Alkaline,
    #[allow(missing_docs)]
    RechargeableAlkaline,
    #[allow(missing_docs)]
    SodiumSulfur,
    #[allow(missing_docs)]
    Flow,
    #[allow(missing_docs)]
    Other,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for Typ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::NotApplicableUnknown,
            1 => Self::LeadAcid,
            2 => Self::NickelMetalHydrate,
            3 => Self::NickelCadmium,
            4 => Self::LithiumIon,
            5 => Self::CarbonZinc,
            6 => Self::ZincChloride,
            7 => Self::Alkaline,
            8 => Self::RechargeableAlkaline,
            9 => Self::SodiumSulfur,
            10 => Self::Flow,
            99 => Self::Other,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::NotApplicableUnknown => 0,
            Self::LeadAcid => 1,
            Self::NickelMetalHydrate => 2,
            Self::NickelCadmium => 3,
            Self::LithiumIon => 4,
            Self::CarbonZinc => 5,
            Self::ZincChloride => 6,
            Self::Alkaline => 7,
            Self::RechargeableAlkaline => 8,
            Self::SodiumSulfur => 9,
            Self::Flow => 10,
            Self::Other => 99,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for Typ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// State of the Battery Bank
///
/// State of the battery bank.  Enumeration.
///
/// Detail: Must be reconciled with State in IEC 61850.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum State {
    #[allow(missing_docs)]
    Disconnected,
    #[allow(missing_docs)]
    Initializing,
    #[allow(missing_docs)]
    Connected,
    #[allow(missing_docs)]
    Standby,
    #[allow(missing_docs)]
    SocProtection,
    #[allow(missing_docs)]
    Suspending,
    #[allow(missing_docs)]
    Fault,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for State {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Disconnected,
            2 => Self::Initializing,
            3 => Self::Connected,
            4 => Self::Standby,
            5 => Self::SocProtection,
            6 => Self::Suspending,
            99 => Self::Fault,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disconnected => 1,
            Self::Initializing => 2,
            Self::Connected => 3,
            Self::Standby => 4,
            Self::SocProtection => 5,
            Self::Suspending => 6,
            Self::Fault => 99,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for State {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
bitflags::bitflags! {
    #[doc = " Battery Event 1 Bitfield"] #[doc = " "] #[doc =
    " Alarms and warnings.  Bit flags."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Evt1 : u32 { #[allow(missing_docs)] const CommunicationError = 1;
    #[allow(missing_docs)] const OverTempAlarm = 2; #[allow(missing_docs)] const
    OverTempWarning = 4; #[allow(missing_docs)] const UnderTempAlarm = 8;
    #[allow(missing_docs)] const UnderTempWarning = 16; #[doc = " Detail: See AChaMax."]
    const OverChargeCurrentAlarm = 32; #[doc = " Detail: See AChaMax."] const
    OverChargeCurrentWarning = 64; #[doc = " Detail: See ADisChaMax."] const
    OverDischargeCurrentAlarm = 128; #[doc = " Detail: See ADisChaMax."] const
    OverDischargeCurrentWarning = 256; #[allow(missing_docs)] const OverVoltAlarm = 512;
    #[allow(missing_docs)] const OverVoltWarning = 1024; #[allow(missing_docs)] const
    UnderVoltAlarm = 2048; #[allow(missing_docs)] const UnderVoltWarning = 4096;
    #[allow(missing_docs)] const UnderSocMinAlarm = 8192; #[allow(missing_docs)] const
    UnderSocMinWarning = 16384; #[allow(missing_docs)] const OverSocMaxAlarm = 32768;
    #[allow(missing_docs)] const OverSocMaxWarning = 65536; #[allow(missing_docs)] const
    VoltageImbalanceWarning = 131072; #[allow(missing_docs)] const
    TemperatureImbalanceAlarm = 262144; #[allow(missing_docs)] const
    TemperatureImbalanceWarning = 524288; #[allow(missing_docs)] const ContactorError =
    1048576; #[allow(missing_docs)] const FanError = 2097152; #[allow(missing_docs)]
    const GroundFault = 4194304; #[allow(missing_docs)] const OpenDoorError = 8388608;
    #[allow(missing_docs)] const CurrentImbalanceWarning = 16777216; #[doc =
    " Detail: See EvtVnd1 and EvtVnd2 for more information."] const OtherAlarm =
    33554432; #[doc = " Detail: See EvtVnd1 and EvtVnd2 for more information."] const
    OtherWarning = 67108864; #[doc = " Detail: Do not implement."] const Reserved1 =
    134217728; #[allow(missing_docs)] const ConfigurationAlarm = 268435456;
    #[allow(missing_docs)] const ConfigurationWarning = 536870912; }
}
impl crate::Value for Evt1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Battery Event 2 Bitfield"] #[doc = " "] #[doc =
    " Alarms and warnings.  Bit flags."] #[doc = " "] #[doc =
    " Detail: Reserved for future use."] #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub
    struct Evt2 : u32 {}
}
impl crate::Value for Evt2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 1"] #[doc = " "] #[doc = " Vendor defined events."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd1 : u32 {}
}
impl crate::Value for EvtVnd1 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd1 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    #[doc = " Vendor Event Bitfield 2"] #[doc = " "] #[doc = " Vendor defined events."]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature = "serde",
    derive(::serde::Serialize, ::serde::Deserialize))] pub struct EvtVnd2 : u32 {}
}
impl crate::Value for EvtVnd2 {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd2 {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
/// Inverter State Request
///
/// Request from battery to start or stop the inverter.  Enumeration.
///
/// Detail: Used in special states such as manual battery charging.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ReqInvState {
    #[allow(missing_docs)]
    NoRequest,
    /// Detail: Battery is notified of inverter state change through SetInvState.
    Start,
    /// Detail: Battery is notified of inverter state change through SetInvState.
    Stop,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ReqInvState {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::NoRequest,
            1 => Self::Start,
            2 => Self::Stop,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::NoRequest => 0,
            Self::Start => 1,
            Self::Stop => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ReqInvState {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Operation
///
/// Instruct the battery bank to perform an operation such as connecting.  Enumeration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SetOp {
    #[allow(missing_docs)]
    Connect,
    #[allow(missing_docs)]
    Disconnect,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SetOp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::Connect,
            2 => Self::Disconnect,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Connect => 1,
            Self::Disconnect => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SetOp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Set Inverter State
///
/// Set the current state of the inverter.
///
/// Detail: Information needed by battery for some operations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SetInvState {
    #[allow(missing_docs)]
    InverterStopped,
    #[allow(missing_docs)]
    InverterStandby,
    #[allow(missing_docs)]
    InverterStarted,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SetInvState {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            1 => Self::InverterStopped,
            2 => Self::InverterStandby,
            3 => Self::InverterStarted,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::InverterStopped => 1,
            Self::InverterStandby => 2,
            Self::InverterStarted => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SetInvState {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Battery {
    const ID: u16 = 802;
    const NAME: &'static str = "battery";
    const LABEL: &'static str = "Battery Base Model";
    const DESCRIPTION: &'static str = "";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m802
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
