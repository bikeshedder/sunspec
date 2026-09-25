//! String Combiner (Advanced)
/// Type alias for [`StringCombinerAdvancedInputs`].
pub type Model404 = StringCombinerAdvancedInputs;
/// String Combiner (Advanced)
///
/// An advanced string combiner including voltage and energy measurements
///
/// Detail: This model supersedes model 402
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StringCombinerAdvancedInputs {
    /// Current scale factor
    pub dc_a_sf: i16,
    /// Amp-hour scale factor
    pub dc_ahr_sf: Option<i16>,
    /// Voltage scale factor
    pub dc_v_sf: Option<i16>,
    /// Power scale factor
    pub dc_w_sf: Option<i16>,
    /// Energy scale factor
    pub dc_wh_sf: Option<i16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    pub dc_a_max: u16,
    /// N
    ///
    /// Number of Inputs
    pub n: u16,
    /// Event
    ///
    /// Events
    pub evt: Evt,
    /// Vendor Event
    ///
    /// Vendor defined events
    pub evt_vnd: Option<EvtVnd>,
    /// Amps
    ///
    /// Total measured current
    pub dc_a: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    pub dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    pub dc_v: Option<i16>,
    /// Temp
    ///
    /// Internal operating temperature
    pub tmp: Option<i16>,
    /// Watts
    ///
    /// Output power
    pub dc_w: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    pub dc_pr: Option<i16>,
    /// Watt-hours
    ///
    /// Output energy
    pub dc_wh: Option<u32>,
    /// Current scale factor for inputs
    pub in_dc_a_sf: Option<i16>,
    /// Amp-hour scale factor for inputs
    pub in_dc_ahr_sf: Option<i16>,
    /// Voltage scale factor for inputs
    pub in_dc_v_sf: Option<i16>,
    /// Power scale factor for inputs
    pub in_dc_w_sf: Option<i16>,
    /// Energy scale factor for inputs
    pub in_dc_wh_sf: Option<i16>,
    #[allow(missing_docs)]
    pub string: Vec<String>,
}
#[allow(missing_docs)]
impl StringCombinerAdvancedInputs {
    pub const DC_A_SF: crate::Point<Self, i16> = crate::Point::new(0, 1, false);
    pub const DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(1, 1, false);
    pub const DC_V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(2, 1, false);
    pub const DC_W_SF: crate::Point<Self, Option<i16>> = crate::Point::new(3, 1, false);
    pub const DC_WH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(4, 1, false);
    pub const DC_A_MAX: crate::Point<Self, u16> = crate::Point::new(5, 1, false);
    pub const N: crate::Point<Self, u16> = crate::Point::new(6, 1, false);
    pub const EVT: crate::Point<Self, Evt> = crate::Point::new(7, 2, false);
    pub const EVT_VND: crate::Point<Self, Option<EvtVnd>> = crate::Point::new(9, 2, false);
    pub const DC_A: crate::Point<Self, i16> = crate::Point::new(11, 1, false);
    pub const DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(12, 2, false);
    pub const DC_V: crate::Point<Self, Option<i16>> = crate::Point::new(14, 1, false);
    pub const TMP: crate::Point<Self, Option<i16>> = crate::Point::new(15, 1, false);
    pub const DC_W: crate::Point<Self, Option<i16>> = crate::Point::new(16, 1, false);
    pub const DC_PR: crate::Point<Self, Option<i16>> = crate::Point::new(17, 1, false);
    pub const DC_WH: crate::Point<Self, Option<u32>> = crate::Point::new(18, 2, false);
    pub const IN_DC_A_SF: crate::Point<Self, Option<i16>> = crate::Point::new(20, 1, false);
    pub const IN_DC_AHR_SF: crate::Point<Self, Option<i16>> = crate::Point::new(21, 1, false);
    pub const IN_DC_V_SF: crate::Point<Self, Option<i16>> = crate::Point::new(22, 1, false);
    pub const IN_DC_W_SF: crate::Point<Self, Option<i16>> = crate::Point::new(23, 1, false);
    pub const IN_DC_WH_SF: crate::Point<Self, Option<i16>> = crate::Point::new(24, 1, false);
}
impl crate::Group for StringCombinerAdvancedInputs {
    const LEN: u16 = 25;
}
impl StringCombinerAdvancedInputs {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        let (nested_data, string) = String::parse_multiple(nested_data)?;
        Ok((
            nested_data,
            Self {
                dc_a_sf: Self::DC_A_SF.from_data(data)?,
                dc_ahr_sf: Self::DC_AHR_SF.from_data(data)?,
                dc_v_sf: Self::DC_V_SF.from_data(data)?,
                dc_w_sf: Self::DC_W_SF.from_data(data)?,
                dc_wh_sf: Self::DC_WH_SF.from_data(data)?,
                dc_a_max: Self::DC_A_MAX.from_data(data)?,
                n: Self::N.from_data(data)?,
                evt: Self::EVT.from_data(data)?,
                evt_vnd: Self::EVT_VND.from_data(data)?,
                dc_a: Self::DC_A.from_data(data)?,
                dc_ahr: Self::DC_AHR.from_data(data)?,
                dc_v: Self::DC_V.from_data(data)?,
                tmp: Self::TMP.from_data(data)?,
                dc_w: Self::DC_W.from_data(data)?,
                dc_pr: Self::DC_PR.from_data(data)?,
                dc_wh: Self::DC_WH.from_data(data)?,
                in_dc_a_sf: Self::IN_DC_A_SF.from_data(data)?,
                in_dc_ahr_sf: Self::IN_DC_AHR_SF.from_data(data)?,
                in_dc_v_sf: Self::IN_DC_V_SF.from_data(data)?,
                in_dc_w_sf: Self::IN_DC_W_SF.from_data(data)?,
                in_dc_wh_sf: Self::IN_DC_WH_SF.from_data(data)?,
                string,
            },
        ))
    }
}
bitflags::bitflags! {
    /// Event
    ///
    /// Events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct Evt: u32 {
        #[allow(missing_docs)]
        const LowVoltage = 1;
        #[allow(missing_docs)]
        const LowPower = 2;
        #[allow(missing_docs)]
        const LowEfficiency = 4;
        #[allow(missing_docs)]
        const Current = 8;
        #[allow(missing_docs)]
        const Voltage = 16;
        #[allow(missing_docs)]
        const Power = 32;
        #[allow(missing_docs)]
        const Pr = 64;
        #[allow(missing_docs)]
        const Disconnected = 128;
        #[allow(missing_docs)]
        const FuseFault = 256;
        #[allow(missing_docs)]
        const CombinerFuseFault = 512;
        #[allow(missing_docs)]
        const CombinerCabinetOpen = 1024;
        #[allow(missing_docs)]
        const Temp = 2048;
        #[allow(missing_docs)]
        const Groundfault = 4096;
        #[allow(missing_docs)]
        const ReversedPolarity = 8192;
        #[allow(missing_docs)]
        const Incompatible = 16384;
        #[allow(missing_docs)]
        const CommError = 32768;
        #[allow(missing_docs)]
        const InternalError = 65536;
        #[allow(missing_docs)]
        const Theft = 131072;
        #[allow(missing_docs)]
        const ArcDetected = 262144;
    }
}
impl crate::Value for Evt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for Evt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Vendor Event
    ///
    /// Vendor defined events
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct EvtVnd: u32 {
    }
}
impl crate::Value for EvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for EvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct String {
    /// ID
    ///
    /// Uniquely identifies this input set
    pub in_id: u16,
    /// Input Event
    ///
    /// String Input Event Flags
    pub in_evt: StringInEvt,
    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    pub in_evt_vnd: Option<StringInEvtVnd>,
    /// Amps
    ///
    /// String Input Current
    pub in_dc_a: i16,
    /// Amp-hours
    ///
    /// String Input Amp-Hours
    pub in_dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// String Input Voltage
    pub in_dc_v: Option<i16>,
    /// Watts
    ///
    /// String Input Power
    pub in_dc_w: Option<i16>,
    /// Watt-hours
    ///
    /// String Input Energy
    pub in_dc_wh: Option<u32>,
    /// PR
    ///
    /// String Performance Ratio
    pub in_dc_pr: Option<u16>,
    /// N
    ///
    /// Number of modules in this input string
    pub in_n: Option<u16>,
}
#[allow(missing_docs)]
impl String {
    pub const IN_ID: crate::Point<Self, u16> = crate::Point::new(0, 1, false);
    pub const IN_EVT: crate::Point<Self, StringInEvt> = crate::Point::new(1, 2, false);
    pub const IN_EVT_VND: crate::Point<Self, Option<StringInEvtVnd>> =
        crate::Point::new(3, 2, false);
    pub const IN_DC_A: crate::Point<Self, i16> = crate::Point::new(5, 1, false);
    pub const IN_DC_AHR: crate::Point<Self, Option<u32>> = crate::Point::new(6, 2, false);
    pub const IN_DC_V: crate::Point<Self, Option<i16>> = crate::Point::new(8, 1, false);
    pub const IN_DC_W: crate::Point<Self, Option<i16>> = crate::Point::new(9, 1, false);
    pub const IN_DC_WH: crate::Point<Self, Option<u32>> = crate::Point::new(10, 2, false);
    pub const IN_DC_PR: crate::Point<Self, Option<u16>> = crate::Point::new(12, 1, false);
    pub const IN_N: crate::Point<Self, Option<u16>> = crate::Point::new(13, 1, false);
}
impl crate::Group for String {
    const LEN: u16 = 14;
}
impl String {
    fn parse_group(data: &[u16]) -> Result<(&[u16], Self), crate::DecodeError> {
        let nested_data = data
            .get(usize::from(<Self as crate::Group>::LEN)..)
            .unwrap_or(&[]);
        Ok((
            nested_data,
            Self {
                in_id: Self::IN_ID.from_data(data)?,
                in_evt: Self::IN_EVT.from_data(data)?,
                in_evt_vnd: Self::IN_EVT_VND.from_data(data)?,
                in_dc_a: Self::IN_DC_A.from_data(data)?,
                in_dc_ahr: Self::IN_DC_AHR.from_data(data)?,
                in_dc_v: Self::IN_DC_V.from_data(data)?,
                in_dc_w: Self::IN_DC_W.from_data(data)?,
                in_dc_wh: Self::IN_DC_WH.from_data(data)?,
                in_dc_pr: Self::IN_DC_PR.from_data(data)?,
                in_n: Self::IN_N.from_data(data)?,
            },
        ))
    }
    fn parse_multiple(data: &[u16]) -> Result<(&[u16], Vec<Self>), crate::DecodeError> {
        let group_len = usize::from(<String as crate::Group>::LEN);
        if group_len == 0 {
            return Ok((data, Vec::new()));
        }
        if data.len() % group_len != 0 {
            return Err(crate::DecodeError::OutOfBounds);
        }
        let group_count = data.len() / group_len;
        let (data, groups) =
            (0..group_count).try_fold((data, Vec::new()), |(data, mut groups), _| {
                let (data, group) = String::parse_group(data)?;
                groups.push(group);
                Ok::<_, crate::DecodeError>((data, groups))
            })?;
        Ok((data, groups))
    }
}
bitflags::bitflags! {
    /// Input Event
    ///
    /// String Input Event Flags
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct StringInEvt: u32 {
        #[allow(missing_docs)]
        const LowVoltage = 1;
        #[allow(missing_docs)]
        const LowPower = 2;
        #[allow(missing_docs)]
        const LowEfficiency = 4;
        #[allow(missing_docs)]
        const Current = 8;
        #[allow(missing_docs)]
        const Voltage = 16;
        #[allow(missing_docs)]
        const Power = 32;
        #[allow(missing_docs)]
        const Pr = 64;
        #[allow(missing_docs)]
        const Disconnected = 128;
        #[allow(missing_docs)]
        const FuseFault = 256;
        #[allow(missing_docs)]
        const CombinerFuseFault = 512;
        #[allow(missing_docs)]
        const CombinerCabinetOpen = 1024;
        #[allow(missing_docs)]
        const Temp = 2048;
        #[allow(missing_docs)]
        const Groundfault = 4096;
        #[allow(missing_docs)]
        const ReversedPolarity = 8192;
        #[allow(missing_docs)]
        const Incompatible = 16384;
        #[allow(missing_docs)]
        const CommError = 32768;
        #[allow(missing_docs)]
        const InternalError = 65536;
        #[allow(missing_docs)]
        const Theft = 131072;
        #[allow(missing_docs)]
        const ArcDetected = 262144;
    }
}
impl crate::Value for StringInEvt {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringInEvt {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
bitflags::bitflags! {
    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    pub struct StringInEvtVnd: u32 {
    }
}
impl crate::Value for StringInEvtVnd {
    fn decode(data: &[u16]) -> Result<Self, crate::DecodeError> {
        let value = u32::decode(data)?;
        Ok(Self::from_bits_retain(value))
    }
    fn encode(self) -> Box<[u16]> {
        self.bits().encode()
    }
}
impl crate::FixedSize for StringInEvtVnd {
    const SIZE: u16 = 2u16;
    const INVALID: Self = Self::from_bits_retain(4294967295u32);
    fn is_invalid(&self) -> bool {
        self.bits() == 4294967295u32
    }
}
impl crate::Model for StringCombinerAdvancedInputs {
    const ID: u16 = 404;
    fn addr(models: &crate::Models) -> crate::ModelAddr<Self> {
        models.m404
    }
    fn parse(data: &[u16]) -> Result<Self, crate::ParseError<Self>> {
        let (_, model) = Self::parse_group(data)?;
        Ok(model)
    }
}
