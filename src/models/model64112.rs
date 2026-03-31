//! OutBack FM Charge Controller
/// OutBack FM Charge Controller
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
    pub cc_config_fault: CcConfigFault,
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
    pub cc_config_mppt_mode: CcConfigMpptMode,
    /// Sweep Width
    pub cc_config_sweep_width: CcConfigSweepWidth,
    /// Sweep Maximum
    pub cc_config_sweep_max: CcConfigSweepMax,
    /// U-Pick PWM Duty Cycle
    pub cc_config_u_pick_duty_cyc: u16,
    /// Grid Tie Mode
    pub cc_config_grid_tie: CcConfigGridTie,
    /// Temp Comp Mode
    pub cc_config_temp_comp: CcConfigTempComp,
    /// Temp Comp Lower Limit
    pub cc_config_temp_comp_llimt: u16,
    /// Temp Comp Upper Limit
    pub cc_config_temp_comp_hlimt: u16,
    /// Auto Restart Mode
    pub cc_config_auto_restart: CcConfigAutoRestart,
    /// Wakeup VOC Change
    pub cc_config_wakeup_voc: u16,
    /// Snooze Mode
    pub cc_config_snooze_mode_a: u16,
    /// Wakeup Interval
    pub cc_config_wakeup_interval: u16,
    /// AUX Output Mode
    pub cc_config_aux_mode: CcConfigAuxMode,
    /// AUX Output Control
    pub cc_config_aux_control: CcConfigAuxControl,
    /// AUX Output State
    pub cc_config_aux_state: CcConfigAuxState,
    /// AUX Output Polarity
    pub cc_config_aux_polarity: CcConfigAuxPolarity,
    /// AUX Low Battery Disconnect
    pub cc_config_aux_l_batt_disc: u16,
    /// AUX Low Battery Reconnect
    pub cc_config_aux_l_batt_rcon: u16,
    /// AUX Low Battery Disconnect Delay
    pub cc_config_aux_l_batt_dly: u16,
    /// AUX Vent Fan
    pub cc_config_aux_vent_fan_v: u16,
    /// AUX PV Trigger
    pub cc_config_aux_pv_trigger_v: u16,
    /// AUX PV Trigger Hold Time
    pub cc_config_aux_pv_trg_h_tm: u16,
    /// AUX Night Light Threshold
    pub cc_config_aux_nlite_thrs_v: u16,
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
    pub cc_config_major_fw_rev: u16,
    /// FM CC Mid Firmware Number
    pub cc_config_mid_fw_rev: u16,
    /// FM CC Minor Firmware Number
    pub cc_config_minor_fw_rev: u16,
    /// Set Data Log Day Offset
    pub cc_config_data_log_day_offset: u16,
    /// Current Data Log Day Offset
    pub cc_config_data_log_cur_day_off: u16,
    /// Data Log Daily (Ah)
    pub cc_config_data_log_daily_ah: u16,
    /// Data Log Daily (kWh)
    pub cc_config_data_log_daily_kwh: u16,
    /// Data Log Daily Maximum Output (A)
    pub cc_config_data_log_max_out_a: u16,
    /// Data Log Daily Maximum Output (W)
    pub cc_config_data_log_max_out_w: u16,
    /// Data Log Daily Absorb Time
    pub cc_config_data_log_absorb_t: u16,
    /// Data Log Daily Float Time
    pub cc_config_data_log_float_t: u16,
    /// Data Log Daily Minimum Battery
    pub cc_config_data_log_min_batt_v: u16,
    /// Data Log Daily Maximum Battery
    pub cc_config_data_log_max_batt_v: u16,
    /// Data Log Daily Maximum Input
    pub cc_config_data_log_max_input_v: u16,
    /// Data Log Clear
    pub cc_config_data_log_clear: u16,
    /// Data Log Clear Complement
    pub cc_config_data_log_clr_comp: u16,
}
#[allow(missing_docs)]
impl Model64112 {
    pub const PORT: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const V_SF: crate::Point<Self, i16> = crate::Point::new(1, 1, false);
    pub const C_SF: crate::Point<Self, i16> = crate::Point::new(2, 1, false);
    pub const H_SF: crate::Point<Self, i16> = crate::Point::new(3, 1, false);
    pub const P_SF: crate::Point<Self, i16> = crate::Point::new(4, 1, false);
    pub const AH_SF: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const KWH_SF: crate::Point<Self, i16> = crate::Point::new(6, 1, false);
    pub const CC_CONFIG_FAULT: crate::Point<Self, CcConfigFault> = crate::Point::new(7, 1, false);
    pub const CC_CONFIG_ABSORB_V: crate::Point<Self, u16> = crate::Point::new(8, 1, false);
    pub const CC_CONFIG_ABSORB_HR: crate::Point<Self, u16> = crate::Point::new(9, 1, false);
    pub const CC_CONFIG_ABSORB_END_A: crate::Point<Self, u16> = crate::Point::new(10, 1, false);
    pub const CC_CONFIG_REBULK_V: crate::Point<Self, u16> = crate::Point::new(11, 1, false);
    pub const CC_CONFIG_FLOAT_V: crate::Point<Self, u16> = crate::Point::new(12, 1, false);
    pub const CC_CONFIG_MAX_CHG_A: crate::Point<Self, u16> = crate::Point::new(13, 1, false);
    pub const CC_CONFIG_EQUALIZE_V: crate::Point<Self, u16> = crate::Point::new(14, 1, false);
    pub const CC_CONFIG_EQUALIZE_HR: crate::Point<Self, u16> = crate::Point::new(15, 1, false);
    pub const CC_CONFIG_AUTO_EQUALIZE: crate::Point<Self, u16> = crate::Point::new(16, 1, false);
    pub const CC_CONFIG_MPPT_MODE: crate::Point<Self, CcConfigMpptMode> =
        crate::Point::new(17, 1, false);
    pub const CC_CONFIG_SWEEP_WIDTH: crate::Point<Self, CcConfigSweepWidth> =
        crate::Point::new(18, 1, false);
    pub const CC_CONFIG_SWEEP_MAX: crate::Point<Self, CcConfigSweepMax> =
        crate::Point::new(19, 1, false);
    pub const CC_CONFIG_U_PICK_DUTY_CYC: crate::Point<Self, u16> = crate::Point::new(20, 1, false);
    pub const CC_CONFIG_GRID_TIE: crate::Point<Self, CcConfigGridTie> =
        crate::Point::new(21, 1, false);
    pub const CC_CONFIG_TEMP_COMP: crate::Point<Self, CcConfigTempComp> =
        crate::Point::new(22, 1, false);
    pub const CC_CONFIG_TEMP_COMP_LLIMT: crate::Point<Self, u16> = crate::Point::new(23, 1, false);
    pub const CC_CONFIG_TEMP_COMP_HLIMT: crate::Point<Self, u16> = crate::Point::new(24, 1, false);
    pub const CC_CONFIG_AUTO_RESTART: crate::Point<Self, CcConfigAutoRestart> =
        crate::Point::new(25, 1, false);
    pub const CC_CONFIG_WAKEUP_VOC: crate::Point<Self, u16> = crate::Point::new(26, 1, false);
    pub const CC_CONFIG_SNOOZE_MODE_A: crate::Point<Self, u16> = crate::Point::new(27, 1, false);
    pub const CC_CONFIG_WAKEUP_INTERVAL: crate::Point<Self, u16> = crate::Point::new(28, 1, false);
    pub const CC_CONFIG_AUX_MODE: crate::Point<Self, CcConfigAuxMode> =
        crate::Point::new(29, 1, false);
    pub const CC_CONFIG_AUX_CONTROL: crate::Point<Self, CcConfigAuxControl> =
        crate::Point::new(30, 1, false);
    pub const CC_CONFIG_AUX_STATE: crate::Point<Self, CcConfigAuxState> =
        crate::Point::new(31, 1, false);
    pub const CC_CONFIG_AUX_POLARITY: crate::Point<Self, CcConfigAuxPolarity> =
        crate::Point::new(32, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DISC: crate::Point<Self, u16> = crate::Point::new(33, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_RCON: crate::Point<Self, u16> = crate::Point::new(34, 1, false);
    pub const CC_CONFIG_AUX_L_BATT_DLY: crate::Point<Self, u16> = crate::Point::new(35, 1, false);
    pub const CC_CONFIG_AUX_VENT_FAN_V: crate::Point<Self, u16> = crate::Point::new(36, 1, false);
    pub const CC_CONFIG_AUX_PV_TRIGGER_V: crate::Point<Self, u16> = crate::Point::new(37, 1, false);
    pub const CC_CONFIG_AUX_PV_TRG_H_TM: crate::Point<Self, u16> = crate::Point::new(38, 1, false);
    pub const CC_CONFIG_AUX_NLITE_THRS_V: crate::Point<Self, u16> = crate::Point::new(39, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_TM: crate::Point<Self, u16> = crate::Point::new(40, 1, false);
    pub const CC_CONFIG_AUX_NLITE_ON_HIST: crate::Point<Self, u16> =
        crate::Point::new(41, 1, false);
    pub const CC_CONFIG_AUX_NLITE_OFF_HIST: crate::Point<Self, u16> =
        crate::Point::new(42, 1, false);
    pub const CC_CONFIG_AUX_ERROR_BATT_V: crate::Point<Self, u16> = crate::Point::new(43, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_H_TIME: crate::Point<Self, u16> =
        crate::Point::new(44, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_DLY_TIME: crate::Point<Self, u16> =
        crate::Point::new(45, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_REL_V: crate::Point<Self, u16> = crate::Point::new(46, 1, false);
    pub const CC_CONFIG_AUX_DIVERT_HYST_V: crate::Point<Self, u16> =
        crate::Point::new(47, 1, false);
    pub const CC_CONFIG_MAJOR_FW_REV: crate::Point<Self, u16> = crate::Point::new(48, 1, false);
    pub const CC_CONFIG_MID_FW_REV: crate::Point<Self, u16> = crate::Point::new(49, 1, false);
    pub const CC_CONFIG_MINOR_FW_REV: crate::Point<Self, u16> = crate::Point::new(50, 1, false);
    pub const CC_CONFIG_DATA_LOG_DAY_OFFSET: crate::Point<Self, u16> =
        crate::Point::new(51, 1, false);
    pub const CC_CONFIG_DATA_LOG_CUR_DAY_OFF: crate::Point<Self, u16> =
        crate::Point::new(52, 1, false);
    pub const CC_CONFIG_DATA_LOG_DAILY_AH: crate::Point<Self, u16> =
        crate::Point::new(53, 1, false);
    pub const CC_CONFIG_DATA_LOG_DAILY_KWH: crate::Point<Self, u16> =
        crate::Point::new(54, 1, false);
    pub const CC_CONFIG_DATA_LOG_MAX_OUT_A: crate::Point<Self, u16> =
        crate::Point::new(55, 1, false);
    pub const CC_CONFIG_DATA_LOG_MAX_OUT_W: crate::Point<Self, u16> =
        crate::Point::new(56, 1, false);
    pub const CC_CONFIG_DATA_LOG_ABSORB_T: crate::Point<Self, u16> =
        crate::Point::new(57, 1, false);
    pub const CC_CONFIG_DATA_LOG_FLOAT_T: crate::Point<Self, u16> = crate::Point::new(58, 1, false);
    pub const CC_CONFIG_DATA_LOG_MIN_BATT_V: crate::Point<Self, u16> =
        crate::Point::new(59, 1, false);
    pub const CC_CONFIG_DATA_LOG_MAX_BATT_V: crate::Point<Self, u16> =
        crate::Point::new(60, 1, false);
    pub const CC_CONFIG_DATA_LOG_MAX_INPUT_V: crate::Point<Self, u16> =
        crate::Point::new(61, 1, false);
    pub const CC_CONFIG_DATA_LOG_CLEAR: crate::Point<Self, u16> = crate::Point::new(62, 1, false);
    pub const CC_CONFIG_DATA_LOG_CLR_COMP: crate::Point<Self, u16> =
        crate::Point::new(63, 1, false);
    fn has_invalid_points(&self) -> bool {
        Self::PORT.is_invalid(&self.port)
            || Self::V_SF.is_invalid(&self.v_sf)
            || Self::C_SF.is_invalid(&self.c_sf)
            || Self::H_SF.is_invalid(&self.h_sf)
            || Self::P_SF.is_invalid(&self.p_sf)
            || Self::AH_SF.is_invalid(&self.ah_sf)
            || Self::KWH_SF.is_invalid(&self.kwh_sf)
            || Self::CC_CONFIG_FAULT.is_invalid(&self.cc_config_fault)
            || Self::CC_CONFIG_ABSORB_V.is_invalid(&self.cc_config_absorb_v)
            || Self::CC_CONFIG_ABSORB_HR.is_invalid(&self.cc_config_absorb_hr)
            || Self::CC_CONFIG_ABSORB_END_A.is_invalid(&self.cc_config_absorb_end_a)
            || Self::CC_CONFIG_REBULK_V.is_invalid(&self.cc_config_rebulk_v)
            || Self::CC_CONFIG_FLOAT_V.is_invalid(&self.cc_config_float_v)
            || Self::CC_CONFIG_MAX_CHG_A.is_invalid(&self.cc_config_max_chg_a)
            || Self::CC_CONFIG_EQUALIZE_V.is_invalid(&self.cc_config_equalize_v)
            || Self::CC_CONFIG_EQUALIZE_HR.is_invalid(&self.cc_config_equalize_hr)
            || Self::CC_CONFIG_AUTO_EQUALIZE.is_invalid(&self.cc_config_auto_equalize)
            || Self::CC_CONFIG_MPPT_MODE.is_invalid(&self.cc_config_mppt_mode)
            || Self::CC_CONFIG_SWEEP_WIDTH.is_invalid(&self.cc_config_sweep_width)
            || Self::CC_CONFIG_SWEEP_MAX.is_invalid(&self.cc_config_sweep_max)
            || Self::CC_CONFIG_U_PICK_DUTY_CYC.is_invalid(&self.cc_config_u_pick_duty_cyc)
            || Self::CC_CONFIG_GRID_TIE.is_invalid(&self.cc_config_grid_tie)
            || Self::CC_CONFIG_TEMP_COMP.is_invalid(&self.cc_config_temp_comp)
            || Self::CC_CONFIG_TEMP_COMP_LLIMT.is_invalid(&self.cc_config_temp_comp_llimt)
            || Self::CC_CONFIG_TEMP_COMP_HLIMT.is_invalid(&self.cc_config_temp_comp_hlimt)
            || Self::CC_CONFIG_AUTO_RESTART.is_invalid(&self.cc_config_auto_restart)
            || Self::CC_CONFIG_WAKEUP_VOC.is_invalid(&self.cc_config_wakeup_voc)
            || Self::CC_CONFIG_SNOOZE_MODE_A.is_invalid(&self.cc_config_snooze_mode_a)
            || Self::CC_CONFIG_WAKEUP_INTERVAL.is_invalid(&self.cc_config_wakeup_interval)
            || Self::CC_CONFIG_AUX_MODE.is_invalid(&self.cc_config_aux_mode)
            || Self::CC_CONFIG_AUX_CONTROL.is_invalid(&self.cc_config_aux_control)
            || Self::CC_CONFIG_AUX_STATE.is_invalid(&self.cc_config_aux_state)
            || Self::CC_CONFIG_AUX_POLARITY.is_invalid(&self.cc_config_aux_polarity)
            || Self::CC_CONFIG_AUX_L_BATT_DISC.is_invalid(&self.cc_config_aux_l_batt_disc)
            || Self::CC_CONFIG_AUX_L_BATT_RCON.is_invalid(&self.cc_config_aux_l_batt_rcon)
            || Self::CC_CONFIG_AUX_L_BATT_DLY.is_invalid(&self.cc_config_aux_l_batt_dly)
            || Self::CC_CONFIG_AUX_VENT_FAN_V.is_invalid(&self.cc_config_aux_vent_fan_v)
            || Self::CC_CONFIG_AUX_PV_TRIGGER_V.is_invalid(&self.cc_config_aux_pv_trigger_v)
            || Self::CC_CONFIG_AUX_PV_TRG_H_TM.is_invalid(&self.cc_config_aux_pv_trg_h_tm)
            || Self::CC_CONFIG_AUX_NLITE_THRS_V.is_invalid(&self.cc_config_aux_nlite_thrs_v)
            || Self::CC_CONFIG_AUX_NLITE_ON_TM.is_invalid(&self.cc_config_aux_nlite_on_tm)
            || Self::CC_CONFIG_AUX_NLITE_ON_HIST.is_invalid(&self.cc_config_aux_nlite_on_hist)
            || Self::CC_CONFIG_AUX_NLITE_OFF_HIST.is_invalid(&self.cc_config_aux_nlite_off_hist)
            || Self::CC_CONFIG_AUX_ERROR_BATT_V.is_invalid(&self.cc_config_aux_error_batt_v)
            || Self::CC_CONFIG_AUX_DIVERT_H_TIME.is_invalid(&self.cc_config_aux_divert_h_time)
            || Self::CC_CONFIG_AUX_DIVERT_DLY_TIME.is_invalid(&self.cc_config_aux_divert_dly_time)
            || Self::CC_CONFIG_AUX_DIVERT_REL_V.is_invalid(&self.cc_config_aux_divert_rel_v)
            || Self::CC_CONFIG_AUX_DIVERT_HYST_V.is_invalid(&self.cc_config_aux_divert_hyst_v)
            || Self::CC_CONFIG_MAJOR_FW_REV.is_invalid(&self.cc_config_major_fw_rev)
            || Self::CC_CONFIG_MID_FW_REV.is_invalid(&self.cc_config_mid_fw_rev)
            || Self::CC_CONFIG_MINOR_FW_REV.is_invalid(&self.cc_config_minor_fw_rev)
            || Self::CC_CONFIG_DATA_LOG_DAY_OFFSET.is_invalid(&self.cc_config_data_log_day_offset)
            || Self::CC_CONFIG_DATA_LOG_CUR_DAY_OFF.is_invalid(&self.cc_config_data_log_cur_day_off)
            || Self::CC_CONFIG_DATA_LOG_DAILY_AH.is_invalid(&self.cc_config_data_log_daily_ah)
            || Self::CC_CONFIG_DATA_LOG_DAILY_KWH.is_invalid(&self.cc_config_data_log_daily_kwh)
            || Self::CC_CONFIG_DATA_LOG_MAX_OUT_A.is_invalid(&self.cc_config_data_log_max_out_a)
            || Self::CC_CONFIG_DATA_LOG_MAX_OUT_W.is_invalid(&self.cc_config_data_log_max_out_w)
            || Self::CC_CONFIG_DATA_LOG_ABSORB_T.is_invalid(&self.cc_config_data_log_absorb_t)
            || Self::CC_CONFIG_DATA_LOG_FLOAT_T.is_invalid(&self.cc_config_data_log_float_t)
            || Self::CC_CONFIG_DATA_LOG_MIN_BATT_V.is_invalid(&self.cc_config_data_log_min_batt_v)
            || Self::CC_CONFIG_DATA_LOG_MAX_BATT_V.is_invalid(&self.cc_config_data_log_max_batt_v)
            || Self::CC_CONFIG_DATA_LOG_MAX_INPUT_V.is_invalid(&self.cc_config_data_log_max_input_v)
            || Self::CC_CONFIG_DATA_LOG_CLEAR.is_invalid(&self.cc_config_data_log_clear)
            || Self::CC_CONFIG_DATA_LOG_CLR_COMP.is_invalid(&self.cc_config_data_log_clr_comp)
    }
}
impl crate::Group for Model64112 {
    const LEN: u16 = 64;
}
impl Model64112 {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
        Ok((
            nested_data,
            Self {
                port: Self::PORT.from_data(data)?,
                v_sf: Self::V_SF.from_data(data)?,
                c_sf: Self::C_SF.from_data(data)?,
                h_sf: Self::H_SF.from_data(data)?,
                p_sf: Self::P_SF.from_data(data)?,
                ah_sf: Self::AH_SF.from_data(data)?,
                kwh_sf: Self::KWH_SF.from_data(data)?,
                cc_config_fault: Self::CC_CONFIG_FAULT.from_data(data)?,
                cc_config_absorb_v: Self::CC_CONFIG_ABSORB_V.from_data(data)?,
                cc_config_absorb_hr: Self::CC_CONFIG_ABSORB_HR.from_data(data)?,
                cc_config_absorb_end_a: Self::CC_CONFIG_ABSORB_END_A.from_data(data)?,
                cc_config_rebulk_v: Self::CC_CONFIG_REBULK_V.from_data(data)?,
                cc_config_float_v: Self::CC_CONFIG_FLOAT_V.from_data(data)?,
                cc_config_max_chg_a: Self::CC_CONFIG_MAX_CHG_A.from_data(data)?,
                cc_config_equalize_v: Self::CC_CONFIG_EQUALIZE_V.from_data(data)?,
                cc_config_equalize_hr: Self::CC_CONFIG_EQUALIZE_HR.from_data(data)?,
                cc_config_auto_equalize: Self::CC_CONFIG_AUTO_EQUALIZE.from_data(data)?,
                cc_config_mppt_mode: Self::CC_CONFIG_MPPT_MODE.from_data(data)?,
                cc_config_sweep_width: Self::CC_CONFIG_SWEEP_WIDTH.from_data(data)?,
                cc_config_sweep_max: Self::CC_CONFIG_SWEEP_MAX.from_data(data)?,
                cc_config_u_pick_duty_cyc: Self::CC_CONFIG_U_PICK_DUTY_CYC.from_data(data)?,
                cc_config_grid_tie: Self::CC_CONFIG_GRID_TIE.from_data(data)?,
                cc_config_temp_comp: Self::CC_CONFIG_TEMP_COMP.from_data(data)?,
                cc_config_temp_comp_llimt: Self::CC_CONFIG_TEMP_COMP_LLIMT.from_data(data)?,
                cc_config_temp_comp_hlimt: Self::CC_CONFIG_TEMP_COMP_HLIMT.from_data(data)?,
                cc_config_auto_restart: Self::CC_CONFIG_AUTO_RESTART.from_data(data)?,
                cc_config_wakeup_voc: Self::CC_CONFIG_WAKEUP_VOC.from_data(data)?,
                cc_config_snooze_mode_a: Self::CC_CONFIG_SNOOZE_MODE_A.from_data(data)?,
                cc_config_wakeup_interval: Self::CC_CONFIG_WAKEUP_INTERVAL.from_data(data)?,
                cc_config_aux_mode: Self::CC_CONFIG_AUX_MODE.from_data(data)?,
                cc_config_aux_control: Self::CC_CONFIG_AUX_CONTROL.from_data(data)?,
                cc_config_aux_state: Self::CC_CONFIG_AUX_STATE.from_data(data)?,
                cc_config_aux_polarity: Self::CC_CONFIG_AUX_POLARITY.from_data(data)?,
                cc_config_aux_l_batt_disc: Self::CC_CONFIG_AUX_L_BATT_DISC.from_data(data)?,
                cc_config_aux_l_batt_rcon: Self::CC_CONFIG_AUX_L_BATT_RCON.from_data(data)?,
                cc_config_aux_l_batt_dly: Self::CC_CONFIG_AUX_L_BATT_DLY.from_data(data)?,
                cc_config_aux_vent_fan_v: Self::CC_CONFIG_AUX_VENT_FAN_V.from_data(data)?,
                cc_config_aux_pv_trigger_v: Self::CC_CONFIG_AUX_PV_TRIGGER_V.from_data(data)?,
                cc_config_aux_pv_trg_h_tm: Self::CC_CONFIG_AUX_PV_TRG_H_TM.from_data(data)?,
                cc_config_aux_nlite_thrs_v: Self::CC_CONFIG_AUX_NLITE_THRS_V.from_data(data)?,
                cc_config_aux_nlite_on_tm: Self::CC_CONFIG_AUX_NLITE_ON_TM.from_data(data)?,
                cc_config_aux_nlite_on_hist: Self::CC_CONFIG_AUX_NLITE_ON_HIST.from_data(data)?,
                cc_config_aux_nlite_off_hist: Self::CC_CONFIG_AUX_NLITE_OFF_HIST.from_data(data)?,
                cc_config_aux_error_batt_v: Self::CC_CONFIG_AUX_ERROR_BATT_V.from_data(data)?,
                cc_config_aux_divert_h_time: Self::CC_CONFIG_AUX_DIVERT_H_TIME.from_data(data)?,
                cc_config_aux_divert_dly_time: Self::CC_CONFIG_AUX_DIVERT_DLY_TIME
                    .from_data(data)?,
                cc_config_aux_divert_rel_v: Self::CC_CONFIG_AUX_DIVERT_REL_V.from_data(data)?,
                cc_config_aux_divert_hyst_v: Self::CC_CONFIG_AUX_DIVERT_HYST_V.from_data(data)?,
                cc_config_major_fw_rev: Self::CC_CONFIG_MAJOR_FW_REV.from_data(data)?,
                cc_config_mid_fw_rev: Self::CC_CONFIG_MID_FW_REV.from_data(data)?,
                cc_config_minor_fw_rev: Self::CC_CONFIG_MINOR_FW_REV.from_data(data)?,
                cc_config_data_log_day_offset: Self::CC_CONFIG_DATA_LOG_DAY_OFFSET
                    .from_data(data)?,
                cc_config_data_log_cur_day_off: Self::CC_CONFIG_DATA_LOG_CUR_DAY_OFF
                    .from_data(data)?,
                cc_config_data_log_daily_ah: Self::CC_CONFIG_DATA_LOG_DAILY_AH.from_data(data)?,
                cc_config_data_log_daily_kwh: Self::CC_CONFIG_DATA_LOG_DAILY_KWH.from_data(data)?,
                cc_config_data_log_max_out_a: Self::CC_CONFIG_DATA_LOG_MAX_OUT_A.from_data(data)?,
                cc_config_data_log_max_out_w: Self::CC_CONFIG_DATA_LOG_MAX_OUT_W.from_data(data)?,
                cc_config_data_log_absorb_t: Self::CC_CONFIG_DATA_LOG_ABSORB_T.from_data(data)?,
                cc_config_data_log_float_t: Self::CC_CONFIG_DATA_LOG_FLOAT_T.from_data(data)?,
                cc_config_data_log_min_batt_v: Self::CC_CONFIG_DATA_LOG_MIN_BATT_V
                    .from_data(data)?,
                cc_config_data_log_max_batt_v: Self::CC_CONFIG_DATA_LOG_MAX_BATT_V
                    .from_data(data)?,
                cc_config_data_log_max_input_v: Self::CC_CONFIG_DATA_LOG_MAX_INPUT_V
                    .from_data(data)?,
                cc_config_data_log_clear: Self::CC_CONFIG_DATA_LOG_CLEAR.from_data(data)?,
                cc_config_data_log_clr_comp: Self::CC_CONFIG_DATA_LOG_CLR_COMP.from_data(data)?,
            },
        ))
    }
}
bitflags::bitflags! {
    #[doc = " Faults"] #[derive(Copy, Clone, Debug, Eq, PartialEq)] #[cfg_attr(feature =
    "serde", derive(::serde::Serialize, ::serde::Deserialize))] pub struct CcConfigFault
    : u16 {}
}
impl crate::Value for CcConfigFault {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for CcConfigFault {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::from_bits_retain(65535u16);
    fn is_invalid(&self) -> bool {
        self.bits() == 65535u16
    }
}
/// MPPT mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigMpptMode {
    #[allow(missing_docs)]
    Auto,
    #[allow(missing_docs)]
    UPick,
    #[allow(missing_docs)]
    Wind,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigMpptMode {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Auto,
            1 => Self::UPick,
            2 => Self::Wind,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Auto => 0,
            Self::UPick => 1,
            Self::Wind => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigMpptMode {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Sweep Width
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigSweepWidth {
    #[allow(missing_docs)]
    Half,
    #[allow(missing_docs)]
    Full,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigSweepWidth {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Half,
            1 => Self::Full,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Half => 0,
            Self::Full => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigSweepWidth {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Sweep Maximum
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigSweepMax {
    #[allow(missing_docs)]
    EightyPercent,
    #[allow(missing_docs)]
    EightyFivePercent,
    #[allow(missing_docs)]
    NintyPercent,
    #[allow(missing_docs)]
    NintyNinePercent,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigSweepMax {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::EightyPercent,
            1 => Self::EightyFivePercent,
            2 => Self::NintyPercent,
            3 => Self::NintyNinePercent,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::EightyPercent => 0,
            Self::EightyFivePercent => 1,
            Self::NintyPercent => 2,
            Self::NintyNinePercent => 3,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigSweepMax {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Grid Tie Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigGridTie {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigGridTie {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigGridTie {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Temp Comp Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigTempComp {
    #[allow(missing_docs)]
    Wide,
    #[allow(missing_docs)]
    Limited,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigTempComp {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Wide,
            1 => Self::Limited,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Wide => 0,
            Self::Limited => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigTempComp {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Auto Restart Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigAutoRestart {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Every90Minutes,
    #[allow(missing_docs)]
    Every90MinutesIfAbsorbOrFloat,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigAutoRestart {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Every90Minutes,
            2 => Self::Every90MinutesIfAbsorbOrFloat,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::Every90Minutes => 1,
            Self::Every90MinutesIfAbsorbOrFloat => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigAutoRestart {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// AUX Output Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigAuxMode {
    #[allow(missing_docs)]
    Float,
    #[allow(missing_docs)]
    DiversionRelay,
    #[allow(missing_docs)]
    DiversionSolidSt,
    #[allow(missing_docs)]
    LowBattDisconnect,
    #[allow(missing_docs)]
    Remote,
    #[allow(missing_docs)]
    VentFan,
    #[allow(missing_docs)]
    PvTrigger,
    #[allow(missing_docs)]
    ErrorOutput,
    #[allow(missing_docs)]
    NightLight,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigAuxMode {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Float,
            1 => Self::DiversionRelay,
            2 => Self::DiversionSolidSt,
            3 => Self::LowBattDisconnect,
            4 => Self::Remote,
            5 => Self::VentFan,
            6 => Self::PvTrigger,
            7 => Self::ErrorOutput,
            8 => Self::NightLight,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Float => 0,
            Self::DiversionRelay => 1,
            Self::DiversionSolidSt => 2,
            Self::LowBattDisconnect => 3,
            Self::Remote => 4,
            Self::VentFan => 5,
            Self::PvTrigger => 6,
            Self::ErrorOutput => 7,
            Self::NightLight => 8,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigAuxMode {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// AUX Output Control
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigAuxControl {
    #[allow(missing_docs)]
    Off,
    #[allow(missing_docs)]
    Auto,
    #[allow(missing_docs)]
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigAuxControl {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Auto,
            2 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::Auto => 1,
            Self::On => 2,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigAuxControl {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// AUX Output State
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigAuxState {
    #[allow(missing_docs)]
    Disabled,
    #[allow(missing_docs)]
    Enabled,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigAuxState {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Disabled => 0,
            Self::Enabled => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigAuxState {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// AUX Output Polarity
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CcConfigAuxPolarity {
    #[allow(missing_docs)]
    Low,
    #[allow(missing_docs)]
    High,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for CcConfigAuxPolarity {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::High,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Low => 0,
            Self::High => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for CcConfigAuxPolarity {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for Model64112 {
    const ID: u16 = 64112;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64112
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        if model.has_invalid_points() {
            Err(crate::ParseError::InvalidPointData(
                crate::InvalidPointData { model },
            ))
        } else {
            Ok(model)
        }
    }
}
