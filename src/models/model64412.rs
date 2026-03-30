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
impl crate::Group for DerExploitation {
    const LEN: u16 = 43;
}
impl DerExploitation {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = &data[usize::from(<Self as crate::Group>::LEN)..];
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum DaManipulation {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for DaManipulation {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<DaManipulation> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                DaManipulation::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Falsify Device Identity
///
/// Change the DER manufacturer and model
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum FalsifyDeviceIdentity {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for FalsifyDeviceIdentity {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<FalsifyDeviceIdentity> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                FalsifyDeviceIdentity::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas P Always Nameplate
///
/// Set the DER meas to always be at nameplate power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasPAlwaysNameplate {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasPAlwaysNameplate {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasPAlwaysNameplate> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasPAlwaysNameplate::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Q Always Minimum
///
/// Set the DER to always be at minimum reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasQAlwaysMinimum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasQAlwaysMinimum {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasQAlwaysMinimum> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasQAlwaysMinimum::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Q Always Maximum
///
/// Set the DER to always be at maximum reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasQAlwaysMaximum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasQAlwaysMaximum {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasQAlwaysMaximum> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasQAlwaysMaximum::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Q Always Zero
///
/// Set the DER to always be at zero reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasQAlwaysZero {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasQAlwaysZero {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasQAlwaysZero> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasQAlwaysZero::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Zero P
///
/// Set the DER to always be at zero P, Q, and S
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasZeroP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasZeroP {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasZeroP> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasZeroP::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Invert Q
///
/// Set the DER to reverse the Q measurement data
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasInvertQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasInvertQ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasInvertQ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasInvertQ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low V
///
/// Set the DER to always measure low voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowV {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowV> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowV::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High V
///
/// Set the DER to always measure high voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighV {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighV> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighV::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low L1 V
///
/// Set the DER to always measure low line 1 voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowL1v {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowL1v> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowL1v::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High L1 V
///
/// Set the DER to always measure high line 1 voltage
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighL1v {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighL1v> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighL1v::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low F
///
/// Set the DER to always measure low frequency
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowF {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowF> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowF::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High F
///
/// Set the DER to always measure high frequency
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighF {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighF> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighF::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low Amps
///
/// Set the DER to always measure low current
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowAmps {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowAmps> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowAmps::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High Amps
///
/// Set the DER to always measure high current
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighAmps {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighAmps> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighAmps::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High S
///
/// Set the DER to always measure high apparent power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighS {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighS> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighS::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low S
///
/// Set the DER to always measure low apparent power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowS {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowS> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowS::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas High Q
///
/// Set the DER to always measure high reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasHighQ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasHighQ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasHighQ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low Q
///
/// Set the DER to always measure low reactive power
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowQ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowQ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowQ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low PF
///
/// Set the DER to always measure low power factor
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowPf {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowPf> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowPf::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Meas Low Reversed PF
///
/// Set the DER to always measure low reversed power factor
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum MeasLowReversedPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for MeasLowReversedPf {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<MeasLowReversedPf> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                MeasLowReversedPf::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate High P
///
/// Set the DER nameplate power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateHighP {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateHighP> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateHighP::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low P
///
/// Set the DER nameplate power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowP {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowP> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowP::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate High S
///
/// Set the DER nameplate apparent power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateHighS {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateHighS> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateHighS::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low S
///
/// Set the DER nameplate apparent power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowS {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowS> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowS::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate High Q
///
/// Set the DER nameplate reactive power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateHighQ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateHighQ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateHighQ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low Q
///
/// Set the DER nameplate reactive power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowQ {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowQ> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowQ::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate High Nom V
///
/// Set the DER nameplate voltage to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateHighNomV {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateHighNomV> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateHighNomV::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low Nom V
///
/// Set the DER nameplate voltage to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowNomV {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowNomV> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowNomV::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low Amps
///
/// Set the DER nameplate current to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowAmps {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowAmps> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowAmps::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low Varmaxinj
///
/// Set the DER nameplate VarMaxInj to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowVarmaxinj {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowVarmaxinj> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowVarmaxinj::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low Varmaxabs
///
/// Set the DER nameplate VarMaxAbs to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowVarmaxabs {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowVarmaxabs> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowVarmaxabs::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Nameplate Low PF
///
/// Set the DER nameplate power factor to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum NameplateLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for NameplateLowPf {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<NameplateLowPf> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                NameplateLowPf::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings High Nom V
///
/// Set the DER settings voltage to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsHighNomV {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsHighNomV> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsHighNomV::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings Low Amps
///
/// Set the DER settings current to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsLowAmps {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsLowAmps> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsLowAmps::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings High P
///
/// Set the DER settings power to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsHighP {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsHighP> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsHighP::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings Low P
///
/// Set the DER settings power to be low
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsLowP {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsLowP> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsLowP::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings High VAMax
///
/// Set the DER settings VAMax to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsHighVaMax {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsHighVaMax {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsHighVaMax> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsHighVaMax::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings High Varmaxinj
///
/// Set the DER settings VarMaxInj to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsHighVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsHighVarmaxinj {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsHighVarmaxinj> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsHighVarmaxinj::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Settings High Varmaxabs
///
/// Set the DER settings VarMaxAbs to be high
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum SettingsHighVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for SettingsHighVarmaxabs {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<SettingsHighVarmaxabs> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                SettingsHighVarmaxabs::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Change Common Model ID
///
/// Change the common model ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ChangeCommonModelId {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for ChangeCommonModelId {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ChangeCommonModelId> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ChangeCommonModelId::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
/// Change Common Model Length
///
/// Change the common model length
#[derive(Copy, Clone, Debug, Eq, PartialEq, strum::FromRepr)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[repr(u16)]
pub enum ChangeCommonModelLength {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
impl crate::Value for ChangeCommonModelLength {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        Self::from_repr(value).ok_or(crate::DecodeError::InvalidEnumValue)
    }
    fn encode(self) -> Box<[u16]> {
        (self as u16).encode()
    }
}
impl crate::Value for Option<ChangeCommonModelLength> {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u16::decode(data)?;
        if value != 65535 {
            Ok(Some(
                ChangeCommonModelLength::from_repr(value)
                    .ok_or(crate::DecodeError::InvalidEnumValue)?,
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
impl crate::Model for DerExploitation {
    const ID: u16 = 64412;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m64412
    }
    fn parse(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
