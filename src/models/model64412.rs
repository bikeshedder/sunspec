//! DER Cyber Exploitation
/// Type alias for [`DerExploitation`].
pub type Model64412 = DerExploitation;
/// DER Cyber Exploitation
///
/// Operations that represent a hacked DER device
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct DerExploitation {
    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    pub da_manipulation: Option<DaManipulation>,
    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    pub falsify_device_identity: Option<FalsifyDeviceIdentity>,
    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    pub meas_p_always_nameplate: Option<MeasPAlwaysNameplate>,
    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    pub meas_q_always_minimum: Option<MeasQAlwaysMinimum>,
    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    pub meas_q_always_maximum: Option<MeasQAlwaysMaximum>,
    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    pub meas_q_always_zero: Option<MeasQAlwaysZero>,
    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    pub meas_zero_p: Option<MeasZeroP>,
    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    pub meas_invert_q: Option<MeasInvertQ>,
    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    pub meas_low_v: Option<MeasLowV>,
    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    pub meas_high_v: Option<MeasHighV>,
    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    pub meas_low_l1v: Option<MeasLowL1v>,
    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    pub meas_high_l1v: Option<MeasHighL1v>,
    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    pub meas_low_f: Option<MeasLowF>,
    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    pub meas_high_f: Option<MeasHighF>,
    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    pub meas_low_amps: Option<MeasLowAmps>,
    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    pub meas_high_amps: Option<MeasHighAmps>,
    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    pub meas_high_s: Option<MeasHighS>,
    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    pub meas_low_s: Option<MeasLowS>,
    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    pub meas_high_q: Option<MeasHighQ>,
    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    pub meas_low_q: Option<MeasLowQ>,
    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    pub meas_low_pf: Option<MeasLowPf>,
    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    pub meas_low_reversed_pf: Option<MeasLowReversedPf>,
    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    pub nameplate_high_p: Option<NameplateHighP>,
    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    pub nameplate_low_p: Option<NameplateLowP>,
    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    pub nameplate_high_s: Option<NameplateHighS>,
    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    pub nameplate_low_s: Option<NameplateLowS>,
    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    pub nameplate_high_q: Option<NameplateHighQ>,
    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    pub nameplate_low_q: Option<NameplateLowQ>,
    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    pub nameplate_high_nom_v: Option<NameplateHighNomV>,
    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    pub nameplate_low_nom_v: Option<NameplateLowNomV>,
    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    pub nameplate_low_amps: Option<NameplateLowAmps>,
    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    pub nameplate_low_varmaxinj: Option<NameplateLowVarmaxinj>,
    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    pub nameplate_low_varmaxabs: Option<NameplateLowVarmaxabs>,
    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    pub nameplate_low_pf: Option<NameplateLowPf>,
    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    pub settings_high_nom_v: Option<SettingsHighNomV>,
    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    pub settings_low_amps: Option<SettingsLowAmps>,
    /// Settings High P
    ///
    /// Set the DER settings power to be high
    pub settings_high_p: Option<SettingsHighP>,
    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    pub settings_low_p: Option<SettingsLowP>,
    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    pub settings_high_va_max: Option<SettingsHighVaMax>,
    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    pub settings_high_varmaxinj: Option<SettingsHighVarmaxinj>,
    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    pub settings_high_varmaxabs: Option<SettingsHighVarmaxabs>,
    /// Change Common Model ID
    ///
    /// Change the common model ID
    pub change_common_model_id: Option<ChangeCommonModelId>,
    /// Change Common Model Length
    ///
    /// Change the common model length
    pub change_common_model_length: Option<ChangeCommonModelLength>,
}
#[allow(missing_docs)]
impl DerExploitation {
    pub const DA_MANIPULATION: crate::Point<Self, Option<DaManipulation>> =
        crate::Point::new(0, 1, true);
    pub const FALSIFY_DEVICE_IDENTITY: crate::Point<Self, Option<FalsifyDeviceIdentity>> =
        crate::Point::new(1, 1, true);
    pub const MEAS_P_ALWAYS_NAMEPLATE: crate::Point<Self, Option<MeasPAlwaysNameplate>> =
        crate::Point::new(2, 1, true);
    pub const MEAS_Q_ALWAYS_MINIMUM: crate::Point<Self, Option<MeasQAlwaysMinimum>> =
        crate::Point::new(3, 1, true);
    pub const MEAS_Q_ALWAYS_MAXIMUM: crate::Point<Self, Option<MeasQAlwaysMaximum>> =
        crate::Point::new(4, 1, true);
    pub const MEAS_Q_ALWAYS_ZERO: crate::Point<Self, Option<MeasQAlwaysZero>> =
        crate::Point::new(5, 1, true);
    pub const MEAS_ZERO_P: crate::Point<Self, Option<MeasZeroP>> = crate::Point::new(6, 1, true);
    pub const MEAS_INVERT_Q: crate::Point<Self, Option<MeasInvertQ>> =
        crate::Point::new(7, 1, true);
    pub const MEAS_LOW_V: crate::Point<Self, Option<MeasLowV>> = crate::Point::new(8, 1, true);
    pub const MEAS_HIGH_V: crate::Point<Self, Option<MeasHighV>> = crate::Point::new(9, 1, true);
    pub const MEAS_LOW_L1V: crate::Point<Self, Option<MeasLowL1v>> = crate::Point::new(10, 1, true);
    pub const MEAS_HIGH_L1V: crate::Point<Self, Option<MeasHighL1v>> =
        crate::Point::new(11, 1, true);
    pub const MEAS_LOW_F: crate::Point<Self, Option<MeasLowF>> = crate::Point::new(12, 1, true);
    pub const MEAS_HIGH_F: crate::Point<Self, Option<MeasHighF>> = crate::Point::new(13, 1, true);
    pub const MEAS_LOW_AMPS: crate::Point<Self, Option<MeasLowAmps>> =
        crate::Point::new(14, 1, true);
    pub const MEAS_HIGH_AMPS: crate::Point<Self, Option<MeasHighAmps>> =
        crate::Point::new(15, 1, true);
    pub const MEAS_HIGH_S: crate::Point<Self, Option<MeasHighS>> = crate::Point::new(16, 1, true);
    pub const MEAS_LOW_S: crate::Point<Self, Option<MeasLowS>> = crate::Point::new(17, 1, true);
    pub const MEAS_HIGH_Q: crate::Point<Self, Option<MeasHighQ>> = crate::Point::new(18, 1, true);
    pub const MEAS_LOW_Q: crate::Point<Self, Option<MeasLowQ>> = crate::Point::new(19, 1, true);
    pub const MEAS_LOW_PF: crate::Point<Self, Option<MeasLowPf>> = crate::Point::new(20, 1, true);
    pub const MEAS_LOW_REVERSED_PF: crate::Point<Self, Option<MeasLowReversedPf>> =
        crate::Point::new(21, 1, true);
    pub const NAMEPLATE_HIGH_P: crate::Point<Self, Option<NameplateHighP>> =
        crate::Point::new(22, 1, true);
    pub const NAMEPLATE_LOW_P: crate::Point<Self, Option<NameplateLowP>> =
        crate::Point::new(23, 1, true);
    pub const NAMEPLATE_HIGH_S: crate::Point<Self, Option<NameplateHighS>> =
        crate::Point::new(24, 1, true);
    pub const NAMEPLATE_LOW_S: crate::Point<Self, Option<NameplateLowS>> =
        crate::Point::new(25, 1, true);
    pub const NAMEPLATE_HIGH_Q: crate::Point<Self, Option<NameplateHighQ>> =
        crate::Point::new(26, 1, true);
    pub const NAMEPLATE_LOW_Q: crate::Point<Self, Option<NameplateLowQ>> =
        crate::Point::new(27, 1, true);
    pub const NAMEPLATE_HIGH_NOM_V: crate::Point<Self, Option<NameplateHighNomV>> =
        crate::Point::new(28, 1, true);
    pub const NAMEPLATE_LOW_NOM_V: crate::Point<Self, Option<NameplateLowNomV>> =
        crate::Point::new(29, 1, true);
    pub const NAMEPLATE_LOW_AMPS: crate::Point<Self, Option<NameplateLowAmps>> =
        crate::Point::new(30, 1, true);
    pub const NAMEPLATE_LOW_VARMAXINJ: crate::Point<Self, Option<NameplateLowVarmaxinj>> =
        crate::Point::new(31, 1, true);
    pub const NAMEPLATE_LOW_VARMAXABS: crate::Point<Self, Option<NameplateLowVarmaxabs>> =
        crate::Point::new(32, 1, true);
    pub const NAMEPLATE_LOW_PF: crate::Point<Self, Option<NameplateLowPf>> =
        crate::Point::new(33, 1, true);
    pub const SETTINGS_HIGH_NOM_V: crate::Point<Self, Option<SettingsHighNomV>> =
        crate::Point::new(34, 1, true);
    pub const SETTINGS_LOW_AMPS: crate::Point<Self, Option<SettingsLowAmps>> =
        crate::Point::new(35, 1, true);
    pub const SETTINGS_HIGH_P: crate::Point<Self, Option<SettingsHighP>> =
        crate::Point::new(36, 1, true);
    pub const SETTINGS_LOW_P: crate::Point<Self, Option<SettingsLowP>> =
        crate::Point::new(37, 1, true);
    pub const SETTINGS_HIGH_VA_MAX: crate::Point<Self, Option<SettingsHighVaMax>> =
        crate::Point::new(38, 1, true);
    pub const SETTINGS_HIGH_VARMAXINJ: crate::Point<Self, Option<SettingsHighVarmaxinj>> =
        crate::Point::new(39, 1, true);
    pub const SETTINGS_HIGH_VARMAXABS: crate::Point<Self, Option<SettingsHighVarmaxabs>> =
        crate::Point::new(40, 1, true);
    pub const CHANGE_COMMON_MODEL_ID: crate::Point<Self, Option<ChangeCommonModelId>> =
        crate::Point::new(41, 1, true);
    pub const CHANGE_COMMON_MODEL_LENGTH: crate::Point<Self, Option<ChangeCommonModelLength>> =
        crate::Point::new(42, 1, true);
}
static DER_EXPLOITATION_FIELDS: &[crate::FieldInfo] = &[
    crate::FieldInfo {
        name: "da_manipulation",
        label: "DA Manipulation",
        description: "Modify the device ID of the DER",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "falsify_device_identity",
        label: "Falsify Device Identity",
        description: "Change the DER manufacturer and model",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_p_always_nameplate",
        label: "Meas P Always Nameplate",
        description: "Set the DER meas to always be at nameplate power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_q_always_minimum",
        label: "Meas Q Always Minimum",
        description: "Set the DER to always be at minimum reactive power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_q_always_maximum",
        label: "Meas Q Always Maximum",
        description: "Set the DER to always be at maximum reactive power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_q_always_zero",
        label: "Meas Q Always Zero",
        description: "Set the DER to always be at zero reactive power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_zero_p",
        label: "Meas Zero P",
        description: "Set the DER to always be at zero P, Q, and S",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_invert_q",
        label: "Meas Invert Q",
        description: "Set the DER to reverse the Q measurement data",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_v",
        label: "Meas Low V",
        description: "Set the DER to always measure low voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_v",
        label: "Meas High V",
        description: "Set the DER to always measure high voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_l1v",
        label: "Meas Low L1 V",
        description: "Set the DER to always measure low line 1 voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_l1v",
        label: "Meas High L1 V",
        description: "Set the DER to always measure high line 1 voltage",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_f",
        label: "Meas Low F",
        description: "Set the DER to always measure low frequency",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_f",
        label: "Meas High F",
        description: "Set the DER to always measure high frequency",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_amps",
        label: "Meas Low Amps",
        description: "Set the DER to always measure low current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_amps",
        label: "Meas High Amps",
        description: "Set the DER to always measure high current",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_s",
        label: "Meas High S",
        description: "Set the DER to always measure high apparent power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_s",
        label: "Meas Low S",
        description: "Set the DER to always measure low apparent power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_high_q",
        label: "Meas High Q",
        description: "Set the DER to always measure high reactive power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_q",
        label: "Meas Low Q",
        description: "Set the DER to always measure low reactive power",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_pf",
        label: "Meas Low PF",
        description: "Set the DER to always measure low power factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "meas_low_reversed_pf",
        label: "Meas Low Reversed PF",
        description: "Set the DER to always measure low reversed power factor",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_high_p",
        label: "Nameplate High P",
        description: "Set the DER nameplate power to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_p",
        label: "Nameplate Low P",
        description: "Set the DER nameplate power to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_high_s",
        label: "Nameplate High S",
        description: "Set the DER nameplate apparent power to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_s",
        label: "Nameplate Low S",
        description: "Set the DER nameplate apparent power to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_high_q",
        label: "Nameplate High Q",
        description: "Set the DER nameplate reactive power to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_q",
        label: "Nameplate Low Q",
        description: "Set the DER nameplate reactive power to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_high_nom_v",
        label: "Nameplate High Nom V",
        description: "Set the DER nameplate voltage to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_nom_v",
        label: "Nameplate Low Nom V",
        description: "Set the DER nameplate voltage to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_amps",
        label: "Nameplate Low Amps",
        description: "Set the DER nameplate current to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_varmaxinj",
        label: "Nameplate Low Varmaxinj",
        description: "Set the DER nameplate VarMaxInj to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_varmaxabs",
        label: "Nameplate Low Varmaxabs",
        description: "Set the DER nameplate VarMaxAbs to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "nameplate_low_pf",
        label: "Nameplate Low PF",
        description: "Set the DER nameplate power factor to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_high_nom_v",
        label: "Settings High Nom V",
        description: "Set the DER settings voltage to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_low_amps",
        label: "Settings Low Amps",
        description: "Set the DER settings current to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_high_p",
        label: "Settings High P",
        description: "Set the DER settings power to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_low_p",
        label: "Settings Low P",
        description: "Set the DER settings power to be low",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_high_va_max",
        label: "Settings High VAMax",
        description: "Set the DER settings VAMax to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_high_varmaxinj",
        label: "Settings High Varmaxinj",
        description: "Set the DER settings VarMaxInj to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "settings_high_varmaxabs",
        label: "Settings High Varmaxabs",
        description: "Set the DER settings VarMaxAbs to be high",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "change_common_model_id",
        label: "Change Common Model ID",
        description: "Change the common model ID",
        kind: crate::FieldKind::Point,
    },
    crate::FieldInfo {
        name: "change_common_model_length",
        label: "Change Common Model Length",
        description: "Change the common model length",
        kind: crate::FieldKind::Point,
    },
];
static DER_EXPLOITATION_GROUP_INFO: crate::GroupInfo = crate::GroupInfo {
    name: "DERExploitation",
    label: "DER Cyber Exploitation",
    description: "Operations that represent a hacked DER device",
    fields: DER_EXPLOITATION_FIELDS,
};
impl crate::GroupMeta for DerExploitation {
    fn group_info() -> &'static crate::GroupInfo {
        &DER_EXPLOITATION_GROUP_INFO
    }
}
impl crate::Group for DerExploitation {
    const LEN: u16 = 43;
}
impl DerExploitation {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                da_manipulation: Self::DA_MANIPULATION.from_data(data)?,
                falsify_device_identity: Self::FALSIFY_DEVICE_IDENTITY.from_data(data)?,
                meas_p_always_nameplate: Self::MEAS_P_ALWAYS_NAMEPLATE.from_data(data)?,
                meas_q_always_minimum: Self::MEAS_Q_ALWAYS_MINIMUM.from_data(data)?,
                meas_q_always_maximum: Self::MEAS_Q_ALWAYS_MAXIMUM.from_data(data)?,
                meas_q_always_zero: Self::MEAS_Q_ALWAYS_ZERO.from_data(data)?,
                meas_zero_p: Self::MEAS_ZERO_P.from_data(data)?,
                meas_invert_q: Self::MEAS_INVERT_Q.from_data(data)?,
                meas_low_v: Self::MEAS_LOW_V.from_data(data)?,
                meas_high_v: Self::MEAS_HIGH_V.from_data(data)?,
                meas_low_l1v: Self::MEAS_LOW_L1V.from_data(data)?,
                meas_high_l1v: Self::MEAS_HIGH_L1V.from_data(data)?,
                meas_low_f: Self::MEAS_LOW_F.from_data(data)?,
                meas_high_f: Self::MEAS_HIGH_F.from_data(data)?,
                meas_low_amps: Self::MEAS_LOW_AMPS.from_data(data)?,
                meas_high_amps: Self::MEAS_HIGH_AMPS.from_data(data)?,
                meas_high_s: Self::MEAS_HIGH_S.from_data(data)?,
                meas_low_s: Self::MEAS_LOW_S.from_data(data)?,
                meas_high_q: Self::MEAS_HIGH_Q.from_data(data)?,
                meas_low_q: Self::MEAS_LOW_Q.from_data(data)?,
                meas_low_pf: Self::MEAS_LOW_PF.from_data(data)?,
                meas_low_reversed_pf: Self::MEAS_LOW_REVERSED_PF.from_data(data)?,
                nameplate_high_p: Self::NAMEPLATE_HIGH_P.from_data(data)?,
                nameplate_low_p: Self::NAMEPLATE_LOW_P.from_data(data)?,
                nameplate_high_s: Self::NAMEPLATE_HIGH_S.from_data(data)?,
                nameplate_low_s: Self::NAMEPLATE_LOW_S.from_data(data)?,
                nameplate_high_q: Self::NAMEPLATE_HIGH_Q.from_data(data)?,
                nameplate_low_q: Self::NAMEPLATE_LOW_Q.from_data(data)?,
                nameplate_high_nom_v: Self::NAMEPLATE_HIGH_NOM_V.from_data(data)?,
                nameplate_low_nom_v: Self::NAMEPLATE_LOW_NOM_V.from_data(data)?,
                nameplate_low_amps: Self::NAMEPLATE_LOW_AMPS.from_data(data)?,
                nameplate_low_varmaxinj: Self::NAMEPLATE_LOW_VARMAXINJ.from_data(data)?,
                nameplate_low_varmaxabs: Self::NAMEPLATE_LOW_VARMAXABS.from_data(data)?,
                nameplate_low_pf: Self::NAMEPLATE_LOW_PF.from_data(data)?,
                settings_high_nom_v: Self::SETTINGS_HIGH_NOM_V.from_data(data)?,
                settings_low_amps: Self::SETTINGS_LOW_AMPS.from_data(data)?,
                settings_high_p: Self::SETTINGS_HIGH_P.from_data(data)?,
                settings_low_p: Self::SETTINGS_LOW_P.from_data(data)?,
                settings_high_va_max: Self::SETTINGS_HIGH_VA_MAX.from_data(data)?,
                settings_high_varmaxinj: Self::SETTINGS_HIGH_VARMAXINJ.from_data(data)?,
                settings_high_varmaxabs: Self::SETTINGS_HIGH_VARMAXABS.from_data(data)?,
                change_common_model_id: Self::CHANGE_COMMON_MODEL_ID.from_data(data)?,
                change_common_model_length: Self::CHANGE_COMMON_MODEL_LENGTH.from_data(data)?,
            },
        ))
    }
}
/// DA Manipulation
///
/// Modify the device ID of the DER
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum DaManipulation {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for DaManipulation {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for DaManipulation {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Falsify Device Identity
///
/// Change the DER manufacturer and model
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum FalsifyDeviceIdentity {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for FalsifyDeviceIdentity {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for FalsifyDeviceIdentity {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas P Always Nameplate
///
/// Set the DER meas to always be at nameplate power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasPAlwaysNameplate {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasPAlwaysNameplate {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasPAlwaysNameplate {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Q Always Minimum
///
/// Set the DER to always be at minimum reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasQAlwaysMinimum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasQAlwaysMinimum {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasQAlwaysMinimum {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Q Always Maximum
///
/// Set the DER to always be at maximum reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasQAlwaysMaximum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasQAlwaysMaximum {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasQAlwaysMaximum {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Q Always Zero
///
/// Set the DER to always be at zero reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasQAlwaysZero {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasQAlwaysZero {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasQAlwaysZero {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Zero P
///
/// Set the DER to always be at zero P, Q, and S
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasZeroP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasZeroP {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasZeroP {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Invert Q
///
/// Set the DER to reverse the Q measurement data
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasInvertQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasInvertQ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasInvertQ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low V
///
/// Set the DER to always measure low voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowV {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowV {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High V
///
/// Set the DER to always measure high voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighV {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighV {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low L1 V
///
/// Set the DER to always measure low line 1 voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowL1v {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowL1v {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High L1 V
///
/// Set the DER to always measure high line 1 voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighL1v {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighL1v {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low F
///
/// Set the DER to always measure low frequency
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowF {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowF {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High F
///
/// Set the DER to always measure high frequency
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighF {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighF {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low Amps
///
/// Set the DER to always measure low current
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowAmps {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowAmps {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High Amps
///
/// Set the DER to always measure high current
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighAmps {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighAmps {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High S
///
/// Set the DER to always measure high apparent power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighS {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighS {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low S
///
/// Set the DER to always measure low apparent power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowS {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowS {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas High Q
///
/// Set the DER to always measure high reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasHighQ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasHighQ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low Q
///
/// Set the DER to always measure low reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowQ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowQ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low PF
///
/// Set the DER to always measure low power factor
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowPf {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowPf {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Meas Low Reversed PF
///
/// Set the DER to always measure low reversed power factor
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MeasLowReversedPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for MeasLowReversedPf {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for MeasLowReversedPf {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate High P
///
/// Set the DER nameplate power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateHighP {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateHighP {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low P
///
/// Set the DER nameplate power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowP {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowP {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate High S
///
/// Set the DER nameplate apparent power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateHighS {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateHighS {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low S
///
/// Set the DER nameplate apparent power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowS {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowS {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate High Q
///
/// Set the DER nameplate reactive power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateHighQ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateHighQ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low Q
///
/// Set the DER nameplate reactive power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowQ {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowQ {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate High Nom V
///
/// Set the DER nameplate voltage to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateHighNomV {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateHighNomV {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low Nom V
///
/// Set the DER nameplate voltage to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowNomV {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowNomV {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low Amps
///
/// Set the DER nameplate current to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowAmps {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowAmps {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low Varmaxinj
///
/// Set the DER nameplate VarMaxInj to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowVarmaxinj {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowVarmaxinj {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low Varmaxabs
///
/// Set the DER nameplate VarMaxAbs to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowVarmaxabs {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowVarmaxabs {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Nameplate Low PF
///
/// Set the DER nameplate power factor to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NameplateLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for NameplateLowPf {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for NameplateLowPf {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings High Nom V
///
/// Set the DER settings voltage to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsHighNomV {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsHighNomV {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings Low Amps
///
/// Set the DER settings current to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsLowAmps {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsLowAmps {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings High P
///
/// Set the DER settings power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsHighP {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsHighP {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings Low P
///
/// Set the DER settings power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsLowP {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsLowP {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings High VAMax
///
/// Set the DER settings VAMax to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsHighVaMax {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsHighVaMax {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsHighVaMax {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings High Varmaxinj
///
/// Set the DER settings VarMaxInj to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsHighVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsHighVarmaxinj {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsHighVarmaxinj {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Settings High Varmaxabs
///
/// Set the DER settings VarMaxAbs to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum SettingsHighVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for SettingsHighVarmaxabs {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for SettingsHighVarmaxabs {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Change Common Model ID
///
/// Change the common model ID
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ChangeCommonModelId {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ChangeCommonModelId {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ChangeCommonModelId {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
/// Change Common Model Length
///
/// Change the common model length
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ChangeCommonModelLength {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On,
    /// Raw enum value not defined by the SunSpec model.
    Invalid(u16),
}
impl crate::EnumValue for ChangeCommonModelLength {
    type Repr = u16;
    const INVALID: Self::Repr = 65535;
    fn from_repr(value: Self::Repr) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::On,
            value => Self::Invalid(value),
        }
    }
    fn to_repr(self) -> Self::Repr {
        match self {
            Self::Off => 0,
            Self::On => 1,
            Self::Invalid(value) => value,
        }
    }
}
impl crate::FixedSize for ChangeCommonModelLength {
    const SIZE: u16 = 1u16;
    const INVALID: Self = Self::Invalid(65535);
    fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}
impl crate::Model for DerExploitation {
    const ID: u16 = 64412;
    const NAME: &'static str = "DERExploitation";
    const LABEL: &'static str = "DER Cyber Exploitation";
    const DESCRIPTION: &'static str = "Operations that represent a hacked DER device";
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64412
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
