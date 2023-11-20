/// OutBack FM Charge Controller
#[derive(Debug)]
pub struct Model64112 {
    /// Port Number
    pub port: u16,
    #[allow(missing_docs)]
    pub v_sf: i16,
    #[allow(missing_docs)]
    pub c_sf: i16,
    #[allow(missing_docs)]
    pub h_sf: i16,
    #[allow(missing_docs)]
    pub p_sf: i16,
    #[allow(missing_docs)]
    pub ah_sf: i16,
    #[allow(missing_docs)]
    pub kwh_sf: i16,
    /// Faults
    pub cc_config_fault: u16,
    /// Absorb
    pub cc_config_absorb_v: u16,
    /// Absorb Time
    pub cc_config_absorb_hr: u16,
    /// Absorb End
    pub cc_config_absorb_end_a: u16,
    /// Rebulk
    pub cc_config_rebulk_v: u16,
    /// Float
    pub cc_config_float_v: u16,
    /// Maximum Charge
    pub cc_config_max_chg_a: u16,
    /// Equalize
    pub cc_config_equalize_v: u16,
    /// Equalize Time
    pub cc_config_equalize_hr: u16,
    /// Auto Equalize Interval
    pub cc_config_auto_equalize: u16,
    /// MPPT mode
    pub cc_config_mppt_mode: u16,
    /// Sweep Width
    pub cc_config_sweep_width: u16,
    /// Sweep Maximum
    pub cc_config_sweep_max: u16,
    /// U-Pick PWM Duty Cycle
    pub cc_config_u_pick_duty_cyc: u16,
    /// Grid Tie Mode
    pub cc_config_grid_tie: u16,
    /// Temp Comp Mode
    pub cc_config_temp_comp: u16,
    /// Temp Comp Lower Limit
    pub cc_config_temp_comp_llimt: u16,
    /// Temp Comp Upper Limit
    pub cc_config_temp_comp_hlimt: u16,
    /// Auto Restart Mode
    pub cc_config_auto_restart: u16,
    /// Wakeup VOC Change
    pub cc_config_wakeup_voc: u16,
    /// Snooze Mode
    pub cc_config_snooze_mode_a: u16,
    /// Wakeup Interval
    pub cc_config_wakeup_interval: u16,
    /// AUX Output Mode
    pub cc_config_aux_mode: u16,
    /// AUX Output Control
    pub cc_config_aux_control: u16,
    /// AUX Output State
    pub cc_config_aux_state: u16,
    /// AUX Output Polarity
    pub cc_config_aux_polarity: u16,
    /// AUX Low Battery Disconnect
    pub cc_config_aux_l_batt_disc: u16,
    /// AUX Low Battery Reconnect
    pub cc_config_aux_l_batt_rcon: u16,
    /// AUX Low Battery Disconnect Delay
    pub cc_config_aux_l_batt_dly: u16,
    /// AUX Vent Fan
    pub cc_config_aux_vent_fan_v: u16,
    /// AUX PV Trigger
    pub cc_config_aux_pv_triggerv: u16,
    /// AUX PV Trigger Hold Time
    pub cc_config_aux_pv_trg_h_tm: u16,
    /// AUX Night Light Threshold
    pub cc_config_aux_nlite_thrsv: u16,
    /// AUX Night Light On Time
    pub cc_config_aux_nlite_on_tm: u16,
    /// AUX Night Light On Hysteresis
    pub cc_config_aux_nlite_on_hist: u16,
    /// AUX Night Light Off Hysteresis
    pub cc_config_aux_nlite_off_hist: u16,
    /// AUX Error Output Low Battery
    pub cc_config_aux_error_batt_v: u16,
    /// AUX Divert Hold Time
    pub cc_config_aux_divert_h_time: u16,
    /// AUX Divert Delay Time
    pub cc_config_aux_divert_dly_time: u16,
    /// AUX Divert Relative
    pub cc_config_aux_divert_rel_v: u16,
    /// AUX Divert Hysteresis
    pub cc_config_aux_divert_hyst_v: u16,
    /// FM CC Major Firmware Number
    pub cc_config_majorfwrev: u16,
    /// FM CC Mid Firmware Number
    pub cc_config_midfwrev: u16,
    /// FM CC Minor Firmware Number
    pub cc_config_minorfwrev: u16,
    /// Set Data Log Day Offset
    pub cc_config_datalog_day_offset: u16,
    /// Current Data Log Day Offset
    pub cc_config_datalog_cur_day_off: u16,
    /// Data Log Daily (Ah)
    pub cc_config_datalog_daily_ah: u16,
    /// Data Log Daily (kWh)
    pub cc_config_datalog_daily_kwh: u16,
    /// Data Log Daily Maximum Output (A)
    pub cc_config_datalog_max_out_a: u16,
    /// Data Log Daily Maximum Output (W)
    pub cc_config_datalog_max_out_w: u16,
    /// Data Log Daily Absorb Time
    pub cc_config_datalog_absorb_t: u16,
    /// Data Log Daily Float Time
    pub cc_config_datalog_float_t: u16,
    /// Data Log Daily Minimum Battery
    pub cc_config_datalog_min_batt_v: u16,
    /// Data Log Daily Maximum Battery
    pub cc_config_datalog_max_batt_v: u16,
    /// Data Log Daily Maximum Input
    pub cc_config_datalog_max_input_v: u16,
    /// Data Log Clear
    pub cc_config_datalog_clear: u16,
    /// Data Log Clear Complement
    pub cc_config_datalog_clr_comp: u16,
}

#[allow(missing_docs)]

impl Model64112 {
    pub const PORT: crate::PointDef<Self, u16> = crate::PointDef::new(0, 1, false);
    pub const V_SF: crate::PointDef<Self, i16> = crate::PointDef::new(1, 1, false);
    pub const C_SF: crate::PointDef<Self, i16> = crate::PointDef::new(2, 1, false);
    pub const H_SF: crate::PointDef<Self, i16> = crate::PointDef::new(3, 1, false);
    pub const P_SF: crate::PointDef<Self, i16> = crate::PointDef::new(4, 1, false);
    pub const AH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(5, 1, false);
    pub const KWH_SF: crate::PointDef<Self, i16> = crate::PointDef::new(6, 1, false);
    pub const CC_CONFIG_FAULT: crate::PointDef<Self, u16> = crate::PointDef::new(7, 1, false);
    pub const CC_CONFIG_ABSORB_V: crate::PointDef<Self, u16> = crate::PointDef::new(8, 1, false);
    pub const CC_CONFIG_ABSORB_HR: crate::PointDef<Self, u16> = crate::PointDef::new(9, 1, false);
    pub const CC_CONFIG_ABSORB_END_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(10, 1, false);
    pub const CC_CONFIG_REBULK_V: crate::PointDef<Self, u16> = crate::PointDef::new(11, 1, false);
    pub const CC_CONFIG_FLOAT_V: crate::PointDef<Self, u16> = crate::PointDef::new(12, 1, false);
    pub const CC_CONFIG_MAX_CHG_A: crate::PointDef<Self, u16> = crate::PointDef::new(13, 1, false);
    pub const CC_CONFIG_EQUALIZE_V: crate::PointDef<Self, u16> = crate::PointDef::new(14, 1, false);
    pub const CC_CONFIG_EQUALIZE_HR: crate::PointDef<Self, u16> =
        crate::PointDef::new(15, 1, false);
    pub const CC_CONFIG_AUTO_EQUALIZE: crate::PointDef<Self, u16> =
        crate::PointDef::new(16, 1, false);
    pub const CC_CONFIG_MPPT_MODE: crate::PointDef<Self, u16> = crate::PointDef::new(17, 1, false);
    pub const CC_CONFIG_SWEEP_WIDTH: crate::PointDef<Self, u16> =
        crate::PointDef::new(18, 1, false);
    pub const CC_CONFIG_SWEEP_MAX: crate::PointDef<Self, u16> = crate::PointDef::new(19, 1, false);
    pub const CC_CONFIG_U_PICK_DUTY_CYC: crate::PointDef<Self, u16> =
        crate::PointDef::new(20, 1, false);
    pub const CC_CONFIG_GRID_TIE: crate::PointDef<Self, u16> = crate::PointDef::new(21, 1, false);
    pub const CC_CONFIG_TEMP_COMP: crate::PointDef<Self, u16> = crate::PointDef::new(22, 1, false);
    pub const CC_CONFIG_TEMP_COMP_LLIMT: crate::PointDef<Self, u16> =
        crate::PointDef::new(23, 1, false);
    pub const CC_CONFIG_TEMP_COMP_HLIMT: crate::PointDef<Self, u16> =
        crate::PointDef::new(24, 1, false);
    pub const CC_CONFIG_AUTO_RESTART: crate::PointDef<Self, u16> =
        crate::PointDef::new(25, 1, false);
    pub const CC_CONFIG_WAKEUP_VOC: crate::PointDef<Self, u16> = crate::PointDef::new(26, 1, false);
    pub const CC_CONFIG_SNOOZE_MODE_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(27, 1, false);
    pub const CC_CONFIG_WAKEUP_INTERVAL: crate::PointDef<Self, u16> =
        crate::PointDef::new(28, 1, false);
    pub const CC_CONFIG_AUX_MODE: crate::PointDef<Self, u16> = crate::PointDef::new(29, 1, false);
    pub const CC_CONFIG_AUX_CONTROL: crate::PointDef<Self, u16> =
        crate::PointDef::new(30, 1, false);
    pub const CC_CONFIG_AUX_STATE: crate::PointDef<Self, u16> = crate::PointDef::new(31, 1, false);
    pub const CC_CONFIG_AUX_POLARITY: crate::PointDef<Self, u16> =
        crate::PointDef::new(32, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DISC: crate::PointDef<Self, u16> =
        crate::PointDef::new(33, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_RCON: crate::PointDef<Self, u16> =
        crate::PointDef::new(34, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DLY: crate::PointDef<Self, u16> =
        crate::PointDef::new(35, 1, false);
    pub const CC_CONFIG_AUX_VENT_FAN_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(36, 1, false);
    pub const CC_CONFIG_AUX_PV_TRIGGERV: crate::PointDef<Self, u16> =
        crate::PointDef::new(37, 1, false);
    pub const CC_CONFIG_AUX_PV_TRG_H_TM: crate::PointDef<Self, u16> =
        crate::PointDef::new(38, 1, false);
    pub const CC_CONFIG_AUX_NLITE_THRSV: crate::PointDef<Self, u16> =
        crate::PointDef::new(39, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_TM: crate::PointDef<Self, u16> =
        crate::PointDef::new(40, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_HIST: crate::PointDef<Self, u16> =
        crate::PointDef::new(41, 1, false);
    pub const CC_CONFIG_AUX_NLITE_OFF_HIST: crate::PointDef<Self, u16> =
        crate::PointDef::new(42, 1, false);
    pub const CC_CONFIG_AUX_ERROR_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(43, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_H_TIME: crate::PointDef<Self, u16> =
        crate::PointDef::new(44, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_DLY_TIME: crate::PointDef<Self, u16> =
        crate::PointDef::new(45, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_REL_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(46, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_HYST_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(47, 1, false);
    pub const CC_CONFIG_MAJORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(48, 1, false);
    pub const CC_CONFIG_MIDFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(49, 1, false);
    pub const CC_CONFIG_MINORFWREV: crate::PointDef<Self, u16> = crate::PointDef::new(50, 1, false);
    pub const CC_CONFIG_DATALOG_DAY_OFFSET: crate::PointDef<Self, u16> =
        crate::PointDef::new(51, 1, false);
    pub const CC_CONFIG_DATALOG_CUR_DAY_OFF: crate::PointDef<Self, u16> =
        crate::PointDef::new(52, 1, false);
    pub const CC_CONFIG_DATALOG_DAILY_AH: crate::PointDef<Self, u16> =
        crate::PointDef::new(53, 1, false);
    pub const CC_CONFIG_DATALOG_DAILY_KWH: crate::PointDef<Self, u16> =
        crate::PointDef::new(54, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_OUT_A: crate::PointDef<Self, u16> =
        crate::PointDef::new(55, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_OUT_W: crate::PointDef<Self, u16> =
        crate::PointDef::new(56, 1, false);
    pub const CC_CONFIG_DATALOG_ABSORB_T: crate::PointDef<Self, u16> =
        crate::PointDef::new(57, 1, false);
    pub const CC_CONFIG_DATALOG_FLOAT_T: crate::PointDef<Self, u16> =
        crate::PointDef::new(58, 1, false);
    pub const CC_CONFIG_DATALOG_MIN_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(59, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_BATT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(60, 1, false);
    pub const CC_CONFIG_DATALOG_MAX_INPUT_V: crate::PointDef<Self, u16> =
        crate::PointDef::new(61, 1, false);
    pub const CC_CONFIG_DATALOG_CLEAR: crate::PointDef<Self, u16> =
        crate::PointDef::new(62, 1, false);
    pub const CC_CONFIG_DATALOG_CLR_COMP: crate::PointDef<Self, u16> =
        crate::PointDef::new(63, 1, false);
}

impl crate::Model for Model64112 {
    const ID: u16 = 64112;
    fn from_data(data: &[u16]) -> Result<Self, crate::ReadModelError> {
        Ok(Self {
            port: Self::PORT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            v_sf: Self::V_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            c_sf: Self::C_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            h_sf: Self::H_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            p_sf: Self::P_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            ah_sf: Self::AH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            kwh_sf: Self::KWH_SF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_fault: Self::CC_CONFIG_FAULT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_v: Self::CC_CONFIG_ABSORB_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_hr: Self::CC_CONFIG_ABSORB_HR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_absorb_end_a: Self::CC_CONFIG_ABSORB_END_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_rebulk_v: Self::CC_CONFIG_REBULK_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_float_v: Self::CC_CONFIG_FLOAT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_max_chg_a: Self::CC_CONFIG_MAX_CHG_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_equalize_v: Self::CC_CONFIG_EQUALIZE_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_equalize_hr: Self::CC_CONFIG_EQUALIZE_HR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_auto_equalize: Self::CC_CONFIG_AUTO_EQUALIZE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_mppt_mode: Self::CC_CONFIG_MPPT_MODE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_sweep_width: Self::CC_CONFIG_SWEEP_WIDTH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_sweep_max: Self::CC_CONFIG_SWEEP_MAX
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_u_pick_duty_cyc: Self::CC_CONFIG_U_PICK_DUTY_CYC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_grid_tie: Self::CC_CONFIG_GRID_TIE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp: Self::CC_CONFIG_TEMP_COMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp_llimt: Self::CC_CONFIG_TEMP_COMP_LLIMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_temp_comp_hlimt: Self::CC_CONFIG_TEMP_COMP_HLIMT
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_auto_restart: Self::CC_CONFIG_AUTO_RESTART
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_wakeup_voc: Self::CC_CONFIG_WAKEUP_VOC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_snooze_mode_a: Self::CC_CONFIG_SNOOZE_MODE_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_wakeup_interval: Self::CC_CONFIG_WAKEUP_INTERVAL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_mode: Self::CC_CONFIG_AUX_MODE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_control: Self::CC_CONFIG_AUX_CONTROL
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_state: Self::CC_CONFIG_AUX_STATE
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_polarity: Self::CC_CONFIG_AUX_POLARITY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_disc: Self::CC_CONFIG_AUX_L_BATT_DISC
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_rcon: Self::CC_CONFIG_AUX_L_BATT_RCON
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_l_batt_dly: Self::CC_CONFIG_AUX_L_BATT_DLY
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_vent_fan_v: Self::CC_CONFIG_AUX_VENT_FAN_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_pv_triggerv: Self::CC_CONFIG_AUX_PV_TRIGGERV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_pv_trg_h_tm: Self::CC_CONFIG_AUX_PV_TRG_H_TM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_thrsv: Self::CC_CONFIG_AUX_NLITE_THRSV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_on_tm: Self::CC_CONFIG_AUX_NLITE_ON_TM
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_on_hist: Self::CC_CONFIG_AUX_NLITE_ON_HIST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_nlite_off_hist: Self::CC_CONFIG_AUX_NLITE_OFF_HIST
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_error_batt_v: Self::CC_CONFIG_AUX_ERROR_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_h_time: Self::CC_CONFIG_AUX_DIVERT_H_TIME
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_dly_time: Self::CC_CONFIG_AUX_DIVERT_DLY_TIME
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_rel_v: Self::CC_CONFIG_AUX_DIVERT_REL_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_aux_divert_hyst_v: Self::CC_CONFIG_AUX_DIVERT_HYST_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_majorfwrev: Self::CC_CONFIG_MAJORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_midfwrev: Self::CC_CONFIG_MIDFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_minorfwrev: Self::CC_CONFIG_MINORFWREV
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_day_offset: Self::CC_CONFIG_DATALOG_DAY_OFFSET
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_cur_day_off: Self::CC_CONFIG_DATALOG_CUR_DAY_OFF
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_daily_ah: Self::CC_CONFIG_DATALOG_DAILY_AH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_daily_kwh: Self::CC_CONFIG_DATALOG_DAILY_KWH
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_out_a: Self::CC_CONFIG_DATALOG_MAX_OUT_A
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_out_w: Self::CC_CONFIG_DATALOG_MAX_OUT_W
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_absorb_t: Self::CC_CONFIG_DATALOG_ABSORB_T
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_float_t: Self::CC_CONFIG_DATALOG_FLOAT_T
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_min_batt_v: Self::CC_CONFIG_DATALOG_MIN_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_batt_v: Self::CC_CONFIG_DATALOG_MAX_BATT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_max_input_v: Self::CC_CONFIG_DATALOG_MAX_INPUT_V
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_clear: Self::CC_CONFIG_DATALOG_CLEAR
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
            cc_config_datalog_clr_comp: Self::CC_CONFIG_DATALOG_CLR_COMP
                .from_data(data)?
                .ok_or(crate::ReadPointError::MissingMandatoryValue)?,
        })
    }
}
