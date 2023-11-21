/// Battery Base Model
#[derive(Debug)]
pub struct Model802 {
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
    pub so_c_max: Option<u16>,
    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    pub so_c_min: Option<u16>,
    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    pub soc_rsv_max: Option<u16>,
    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    pub so_c_rsv_min: Option<u16>,
    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub so_c: u16,
    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Notes: Measurement.
    pub do_d: Option<u16>,
    /// State of Health
    ///
    /// Percentage of battery life remaining.
    pub so_h: Option<u16>,
    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    pub n_cyc: Option<u32>,
    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    pub cha_st: Option<u16>,
    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Notes: Maps to DRCC.LocRemCtl in IEC 61850.
    pub loc_rem_ctl: u16,
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
    /// Notes: Battery should reset to 0 when reset is complete.
    pub alm_rst: u16,
    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Notes: Maps to DBAT.BatTyp in 61850.
    pub typ: u16,
    /// State of the Battery Bank
    ///
    /// State of the battery bank.  Enumeration.
    ///
    /// Notes: Must be reconciled with State in IEC 61850.
    pub state: u16,
    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state.  Enumeration.
    pub state_vnd: Option<u16>,
    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Notes: Number of days since 1/1/2000.
    pub warr_dt: Option<u32>,
    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    pub evt1: u32,
    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.  Bit flags.
    ///
    /// Notes: Reserved for future use.
    pub evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    pub evt_vnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    pub evt_vnd2: u32,
    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Notes: Maps to ZBAT.V in IEC 61850.
    pub v: u16,
    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// Notes: If not implemented, must implement AChaMax and ADisChaMax.
    pub v_max: Option<u16>,
    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// Notes: If not implemented, must implement AChaMax and ADisChaMax.
    pub v_min: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Notes: Measurement.
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
    /// Notes: Measurement.
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
    /// Notes: Calculation based on measurements.
    pub cell_v_avg: Option<u16>,
    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Notes: Measurement.
    pub a: i16,
    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Notes: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub a_cha_max: Option<u16>,
    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Notes: Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    pub a_dis_cha_max: Option<u16>,
    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// Notes: Measurement.
    pub w: i16,
    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter.  Enumeration.
    ///
    /// Notes: Used in special states such as manual battery charging.
    pub req_inv_state: Option<u16>,
    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Notes: Used in special states such as string balancing.
    pub req_w: Option<i16>,
    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting.  Enumeration.
    pub set_op: u16,
    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Notes: Information needed by battery for some operations.
    pub set_inv_state: u16,
    /// Scale factor for charge capacity.
    pub ah_rtg_sf: i16,
    /// Scale factor for energy capacity.
    pub wh_rtg_sf: i16,
    /// Scale factor for maximum charge and discharge rate.
    pub w_cha_dis_cha_max_sf: i16,
    /// Scale factor for self discharge rate.
    pub dis_cha_rte_sf: Option<i16>,
    /// Scale factor for state of charge values.
    pub so_c_sf: i16,
    /// Scale factor for depth of discharge.
    pub do_d_sf: Option<i16>,
    /// Scale factor for state of health.
    pub so_h_sf: Option<i16>,
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

impl Model802 {
    pub const AH_RTG: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const WH_RTG: crate::PointDef<Self, u16> = crate::PointDef::new(1, 1, false);
    pub const W_CHA_RTE_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(2, 1, false);
    pub const W_DIS_CHA_RTE_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(3, 1, false);
    pub const DIS_CHA_RTE: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(4, 1, false);
    pub const SO_C_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(5, 1, false);
    pub const SO_C_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(6, 1, false);
    pub const SOC_RSV_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(7, 1, true);
    pub const SO_C_RSV_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(8, 1, true);
    pub const SO_C: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const DO_D: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(10, 1, false);
    pub const SO_H: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(11, 1, false);
    pub const N_CYC: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(12, 2, false);
    pub const CHA_ST: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(14, 1, false);
    pub const LOC_REM_CTL: crate::PointDef<Self, u16> = crate::PointDef::new(15, 1, false);
    pub const HB: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(16, 1, false);
    pub const CTRL_HB: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(17, 1, true);
    pub const ALM_RST: crate::PointDef<Self, u16> = crate::PointDef::new(18, 1, true);
    pub const TYP: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const STATE: crate::PointDef<Self, u16> = crate::PointDef::new(20, 1, false);
    pub const STATE_VND: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(21, 1, false);
    pub const WARR_DT: crate::PointDef<Self, Option<u32>> = crate::PointDef::new(22, 2, false);
    pub const EVT1: crate::PointDef<Self, u32> = crate::PointDef::new(24, 2, false);
    pub const EVT2: crate::PointDef<Self, u32> = crate::PointDef::new(26, 2, false);
    pub const EVT_VND1: crate::PointDef<Self, u32> = crate::PointDef::new(28, 2, false);
    pub const EVT_VND2: crate::PointDef<Self, u32> = crate::PointDef::new(30, 2, false);
    pub const V: crate::PointDef<Self, u16> = crate::PointDef::new(32, 1, false);
    pub const V_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(33, 1, false);
    pub const V_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(34, 1, false);
    pub const CELL_V_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(35, 1, false);
    pub const CELL_V_MAX_STR: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(36, 1, false);
    pub const CELL_V_MAX_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(37, 1, false);
    pub const CELL_V_MIN: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(38, 1, false);
    pub const CELL_V_MIN_STR: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(39, 1, false);
    pub const CELL_V_MIN_MOD: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(40, 1, false);
    pub const CELL_V_AVG: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(41, 1, false);
    pub const A: crate::PointDef<Self, i16> = crate::PointDef::new(42, 1, false);
    pub const A_CHA_MAX: crate::PointDef<Self, Option<u16>> = crate::PointDef::new(43, 1, false);
    pub const A_DIS_CHA_MAX: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(44, 1, false);
    pub const W: crate::PointDef<Self, i16> = crate::PointDef::new(45, 1, false);
    pub const REQ_INV_STATE: crate::PointDef<Self, Option<u16>> =
        crate::PointDef::new(46, 1, false);
    pub const REQ_W: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(47, 1, false);
    pub const SET_OP: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, true);
    pub const SET_INV_STATE: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, true);
    pub const AH_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(50, 1, false);
    pub const WH_RTG_SF: crate::PointDef<Self, i16> = crate::PointDef::new(51, 1, false);
    pub const W_CHA_DIS_CHA_MAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(52, 1, false);
    pub const DIS_CHA_RTE_SF: crate::PointDef<Self, Option<i16>> =
        crate::PointDef::new(53, 1, false);
    pub const SO_C_SF: crate::PointDef<Self, i16> = crate::PointDef::new(54, 1, false);
    pub const DO_D_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(55, 1, false);
    pub const SO_H_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(56, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(57, 1, false);
    pub const CELL_V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(58, 1, false);
    pub const A_SF: crate::PointDef<Self, i16> = crate::PointDef::new(59, 1, false);
    pub const A_MAX_SF: crate::PointDef<Self, i16> = crate::PointDef::new(60, 1, false);
    pub const W_SF: crate::PointDef<Self, Option<i16>> = crate::PointDef::new(61, 1, false);
}

impl crate::Model for Model802 {
    const ID: u16 = 802;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            ah_rtg: Self::AH_RTG.from_data(data)?,
            wh_rtg: Self::WH_RTG.from_data(data)?,
            w_cha_rte_max: Self::W_CHA_RTE_MAX.from_data(data)?,
            w_dis_cha_rte_max: Self::W_DIS_CHA_RTE_MAX.from_data(data)?,
            dis_cha_rte: Self::DIS_CHA_RTE.from_data(data)?,
            so_c_max: Self::SO_C_MAX.from_data(data)?,
            so_c_min: Self::SO_C_MIN.from_data(data)?,
            soc_rsv_max: Self::SOC_RSV_MAX.from_data(data)?,
            so_c_rsv_min: Self::SO_C_RSV_MIN.from_data(data)?,
            so_c: Self::SO_C.from_data(data)?,
            do_d: Self::DO_D.from_data(data)?,
            so_h: Self::SO_H.from_data(data)?,
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
            so_c_sf: Self::SO_C_SF.from_data(data)?,
            do_d_sf: Self::DO_D_SF.from_data(data)?,
            so_h_sf: Self::SO_H_SF.from_data(data)?,
            v_sf: Self::V_SF.from_data(data)?,
            cell_v_sf: Self::CELL_V_SF.from_data(data)?,
            a_sf: Self::A_SF.from_data(data)?,
            a_max_sf: Self::A_MAX_SF.from_data(data)?,
            w_sf: Self::W_SF.from_data(data)?,
        })
    }
}
